use crate::oracle::MleOracle;
use p3_field::Field;

pub struct SumcheckClaim<F: Field> {
    pub claimed_sum: F,
    pub n_vars: usize,
}

/// One round of sumcheck: prover sends g(X) = sum over residual hypercube
/// Returns the degree-1 round polynomial [g(0), g(1)]
pub fn sumcheck_round<F: Field>(oracle: &impl MleOracle<F>, prefix: &[F]) -> [F; 2] {
    let remaining = oracle.n_vars() - prefix.len();
    assert!(remaining > 0);

    let sum_size = 1usize << (remaining - 1);

    let eval_at = |bit: F| -> F {
        let mut pre = prefix.to_vec();
        pre.push(bit);
        (0..sum_size)
            .map(|idx| {
                let mut full = pre.clone();
                for i in 0..(remaining - 1) {
                    full.push(if (idx >> i) & 1 == 1 { F::ONE } else { F::ZERO });
                }
                oracle.eval(&full)
            })
            .fold(F::ZERO, |a, b| a + b)
    };

    [eval_at(F::ZERO), eval_at(F::ONE)]
}

// ─── Verifier (Issue #35, ARCHER Finding F20) ────────────────────────────────
//
// Prior state: the sumcheck protocol in this crate was prover-only. A
// verifier existed only as a private helper inside the prover-server
// binary (prover-server/src/main.rs `sumcheck_verify`), where it could
// not be reused or tested by the library that defines the protocol.
// This module closes that gap: a library-level verifier over the same
// degree-1 round-polynomial protocol used by `sumcheck_round`.
//
// Scope boundary: this verifier checks the protocol mathematics —
// round-to-round consistency and the two-part final binding
// (transcript binding + oracle binding). Challenge DERIVATION policy
// (Fiat-Shamim re-derivation from a transcript) is deliberately the
// caller's concern: prover-server binds challenges through its Poseidon2
// Challenger, and transcript/accumulator binding is tracked separately
// as Issue #39. Here, challenges are explicit verifier inputs.

/// A complete sumcheck transcript, as presented by a prover.
///
/// The protocol: the prover claims `claimed_sum` = the sum of the
/// multilinear oracle over the full Boolean hypercube. One round
/// polynomial `(g(0), g(1))` is presented per variable, in order.
/// After all rounds fold under the verifier's challenges, the running
/// claim equals the transcript's implied final evaluation, which the
/// prover additionally states explicitly as `final_eval`.
#[derive(Clone, Debug)]
pub struct SumcheckProof<F: Field> {
    /// The prover's claim: the total sum over the full Boolean hypercube.
    pub claimed_sum: F,
    /// Number of variables (rounds) in the protocol.
    pub n_vars: usize,
    /// One `(g(0), g(1))` pair per variable, in order.
    pub rounds: Vec<[F; 2]>,
    /// The prover's explicit final-evaluation claim at the full
    /// challenge point. Must equal both the folded running claim
    /// (transcript binding) and the oracle's actual evaluation at the
    /// challenge point (oracle binding).
    pub final_eval: F,
}

/// Why a sumcheck transcript was rejected, with the exact check that
/// fired. Round indices are 0-based.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SumcheckError {
    /// The transcript has fewer or more round polynomials than the
    /// claimed number of variables.
    RoundCount { expected: usize, received: usize },
    /// The verifier was given fewer or more challenges than rounds.
    ChallengeCount { expected: usize, received: usize },
    /// `g(0) + g(1)` at this round does not equal the running claim
    /// (the prover's `claimed_sum` for round 0, the previous round's
    /// folded value afterwards).
    RoundConsistency { round: usize },
    /// The folded running claim after the last round does not equal the
    /// prover's explicit `final_eval` claim: the transcript is not
    /// self-consistent.
    TranscriptBinding,
    /// The prover's `final_eval` claim does not equal the oracle
    /// evaluated at the full challenge point: the transcript does not
    /// bind to the oracle it was proven about.
    FinalEvalBinding,
}

impl core::fmt::Display for SumcheckError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SumcheckError::RoundCount { expected, received } => write!(
                f,
                "round count mismatch: transcript claims {expected} variables but carries {received} round polynomials"
            ),
            SumcheckError::ChallengeCount { expected, received } => write!(
                f,
                "challenge count mismatch: {expected} rounds but {received} challenges"
            ),
            SumcheckError::RoundConsistency { round } => write!(
                f,
                "round {round}: g(0) + g(1) != running claim"
            ),
            SumcheckError::TranscriptBinding => write!(
                f,
                "transcript binding: folded running claim != prover's final_eval claim"
            ),
            SumcheckError::FinalEvalBinding => write!(
                f,
                "final-eval binding: final_eval claim != oracle eval at full challenge point"
            ),
        }
    }
}

impl core::error::Error for SumcheckError {}

/// Verify a complete sumcheck transcript against an oracle.
///
/// Checks, in order:
///
/// 1. `rounds.len() == n_vars` and `challenges.len() == n_vars`.
/// 2. Round consistency (each round, 0-based): `g(0) + g(1)` equals the
///    running claim; the claim then folds to `g(0) + r * (g(1) - g(0))`
///    under that round's challenge.
/// 3. Transcript binding: the folded running claim after the last round
///    equals the prover's explicit `final_eval` claim.
/// 4. Oracle binding: `final_eval` equals the oracle evaluated at the
///    full challenge point.
///
/// Returns the first failure as a `SumcheckError` naming the check and
/// round; returns `Ok(())` only when every check passes.
pub fn verify_sumcheck<F: Field>(
    proof: &SumcheckProof<F>,
    challenges: &[F],
    oracle: &impl MleOracle<F>,
) -> Result<(), SumcheckError> {
    if proof.rounds.len() != proof.n_vars {
        return Err(SumcheckError::RoundCount {
            expected: proof.n_vars,
            received: proof.rounds.len(),
        });
    }
    if challenges.len() != proof.n_vars {
        return Err(SumcheckError::ChallengeCount {
            expected: proof.n_vars,
            received: challenges.len(),
        });
    }

    let mut running_claim = proof.claimed_sum;
    let mut point: Vec<F> = Vec::with_capacity(proof.n_vars);

    for (round, round_poly) in proof.rounds.iter().enumerate() {
        let &[g0, g1] = round_poly;
        if g0 + g1 != running_claim {
            return Err(SumcheckError::RoundConsistency { round });
        }
        let r = challenges[round];
        running_claim = g0 + r * (g1 - g0);
        point.push(r);
    }

    // Transcript binding: the folded running claim IS the transcript's
    // implied final evaluation. The prover's explicit claim must agree.
    if running_claim != proof.final_eval {
        return Err(SumcheckError::TranscriptBinding);
    }

    // Oracle binding: the final evaluation claim must match the oracle
    // it was proven about, at the full challenge point. A prover can
    // pass every round check with a false claim (round polys are folded
    // forward, not re-derived); this check is what catches that lie.
    if oracle.eval(&point) != proof.final_eval {
        return Err(SumcheckError::FinalEvalBinding);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::folded::FoldedOracleBuilder;
    use crate::oracle::ColumnOracle;
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeCharacteristicRing;

    type F = BabyBear;

    // ── Fixtures ───────────────────────────────────────────────────────

    /// Fixed, nontrivial challenges (odd small values; none is 1/2 in the
    /// field, so no tamper can accidentally fold to itself).
    fn test_challenges(n_vars: usize) -> Vec<F> {
        (0..n_vars).map(|i| F::from_u32(3 + 2 * i as u32)).collect()
    }

    /// Deterministic pseudo-random column oracle (LCG fill).
    fn lcg_oracle(n_vars: usize, seed: u32) -> ColumnOracle<F> {
        let len = 1usize << n_vars;
        let mut x = seed;
        let values = (0..len)
            .map(|_| {
                x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                F::from_u32(x)
            })
            .collect();
        ColumnOracle { values, n_vars }
    }

    /// Sum of the oracle over the full Boolean hypercube (the value an
    /// honest prover claims).
    fn hypercube_sum(oracle: &impl MleOracle<F>, n_vars: usize) -> F {
        (0..(1usize << n_vars))
            .map(|i| {
                let pt: Vec<F> = (0..n_vars)
                    .map(|b| if (i >> b) & 1 == 1 { F::ONE } else { F::ZERO })
                    .collect();
                oracle.eval(&pt)
            })
            .fold(F::ZERO, |a, b| a + b)
    }

    /// Honest prover: drives the library's `sumcheck_round` per round and
    /// records the transcript. The prover picks its messages BEFORE the
    /// challenge of its own round (interactive order); the final_eval is
    /// the oracle's true evaluation at the full challenge point.
    fn honest_prove(oracle: &impl MleOracle<F>, challenges: &[F]) -> SumcheckProof<F> {
        let n_vars = oracle.n_vars();
        assert_eq!(challenges.len(), n_vars);
        let mut rounds = Vec::with_capacity(n_vars);
        let mut prefix: Vec<F> = Vec::with_capacity(n_vars);
        for &ch in challenges {
            rounds.push(sumcheck_round(oracle, &prefix));
            prefix.push(ch);
        }
        SumcheckProof {
            claimed_sum: hypercube_sum(oracle, n_vars),
            n_vars,
            rounds,
            final_eval: oracle.eval(&prefix),
        }
    }

    /// The folded-oracle fixture from the fibonacci demo (8 rows, 3
    /// variables, folded over two columns with an absorbed challenge).
    fn folded_fibonacci_oracle() -> crate::folded::FoldedOracle<F> {
        let n = 8usize;
        let n_vars = 3;
        let mut col0 = vec![F::ZERO; n];
        let mut col1 = vec![F::ZERO; n];
        col0[0] = F::ONE;
        col1[0] = F::ONE;
        for i in 1..n {
            col0[i] = col0[i - 1] + col1[i - 1];
            col1[i] = col1[i - 1] + col0[i];
        }
        let alpha = F::from_u32(7);
        FoldedOracleBuilder::new(vec![col0, col1], n_vars)
            .absorb_challenge(alpha)
            .build()
    }

    // ── Named test path 1: honest accept ───────────────────────────────

    #[test]
    fn honest_proof_verifies() {
        let oracle = folded_fibonacci_oracle();
        let challenges = test_challenges(3);
        let proof = honest_prove(&oracle, &challenges);
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &oracle),
            Ok(()),
            "honest transcript over the folded Fibonacci oracle must verify"
        );
    }

    #[test]
    fn honest_proof_verifies_lcg_column_oracle() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let proof = honest_prove(&oracle, &challenges);
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &oracle),
            Ok(()),
            "honest transcript over a 4-variable pseudo-random oracle must verify"
        );
    }

    // ── Named test path 2: wrong-sum reject ────────────────────────────

    #[test]
    fn wrong_claimed_sum_fails_verification() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let mut proof = honest_prove(&oracle, &challenges);
        proof.claimed_sum += F::ONE; // lie about the sum
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &oracle),
            Err(SumcheckError::RoundConsistency { round: 0 }),
            "a wrong claimed_sum must be rejected at round 0, where g(0)+g(1) \
             is checked against the claim"
        );
    }

    // ── Named test path 3: tampered-round reject ───────────────────────

    #[test]
    fn tampered_round_message_fails_verification() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let mut proof = honest_prove(&oracle, &challenges);
        // Tamper a middle round (index 1 of 0..4): its message no longer
        // sums to the running claim from round 0's fold.
        proof.rounds[1][0] += F::ONE;
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &oracle),
            Err(SumcheckError::RoundConsistency { round: 1 }),
            "a tampered round message must be rejected at the round it corrupts"
        );
    }

    #[test]
    fn tampered_final_round_mass_preserving_fails_transcript_binding() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let mut proof = honest_prove(&oracle, &challenges);
        // Tamper the LAST round preserving g(0)+g(1): every round check
        // still passes, but the fold lands on a different value, so the
        // transcript binding must catch it. This exercises the final
        // binding's teeth, not just the per-round sum checks.
        let d = F::from_u32(5);
        let [g0, g1] = proof.rounds[3];
        proof.rounds[3] = [g0 - d, g1 + d];
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &oracle),
            Err(SumcheckError::TranscriptBinding),
            "a mass-preserving tamper on the last round passes all round \
             checks and must be caught by the transcript binding"
        );
    }

    // ── Named test path 4: tampered-final-eval reject ──────────────────

    #[test]
    fn tampered_final_eval_fails_verification() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let mut proof = honest_prove(&oracle, &challenges);
        // All rounds honest; only the explicit final-eval claim is lied
        // about. The folded running claim still holds the true value, so
        // the transcript binding fires.
        proof.final_eval -= F::ONE;
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &oracle),
            Err(SumcheckError::TranscriptBinding),
            "a tampered final_eval claim inconsistent with the honest rounds \
             must be rejected by transcript binding"
        );
    }

    #[test]
    fn wrong_oracle_fails_final_eval_binding() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let proof = honest_prove(&oracle, &challenges);
        // Fully honest transcript — verified against a DIFFERENT oracle.
        // Rounds and transcript binding all pass; only the oracle
        // binding can catch the mismatch.
        let wrong_oracle = lcg_oracle(4, 0x1BADB002);
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &wrong_oracle),
            Err(SumcheckError::FinalEvalBinding),
            "an honest transcript verified against the wrong oracle must be \
             rejected by the oracle binding"
        );
    }

    // ── Shape checks ───────────────────────────────────────────────────

    #[test]
    fn round_count_mismatch_is_rejected() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let mut proof = honest_prove(&oracle, &challenges);
        proof.rounds.pop(); // claim 4 variables, carry 3 rounds
        assert_eq!(
            verify_sumcheck(&proof, &challenges, &oracle),
            Err(SumcheckError::RoundCount {
                expected: 4,
                received: 3,
            })
        );
    }

    #[test]
    fn challenge_count_mismatch_is_rejected() {
        let oracle = lcg_oracle(4, 0xDEADBEEF);
        let challenges = test_challenges(4);
        let proof = honest_prove(&oracle, &challenges);
        let short = &challenges[..3];
        assert_eq!(
            verify_sumcheck(&proof, short, &oracle),
            Err(SumcheckError::ChallengeCount {
                expected: 4,
                received: 3,
            })
        );
    }
}
