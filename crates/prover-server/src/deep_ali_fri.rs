//! Issue #34 / ARCHER Finding F25: connect the DEEP-ALI quotient to the
//! oracle-layer FRI prover.
//!
//! Interface grounding, recorded before implementation:
//!
//! DEEP-ALI side (`crate::deep_ali`):
//! - Producer: `DeepAliVerifier::compute_deep_quotient` on
//!   `BabyBearDeepAli<MMCS, P>`, a public trait method.
//! - Output shape: `Vec<BabyBear>` in coefficient form (monomial basis),
//!   per-query quotients combined with challenger-sampled weights,
//!   trailing zeros trimmed; degree bound `< max_degree` enforced
//!   producer-side.
//! - Field: BabyBear.
//! - Commitment: NONE existed. The quotient left the producer
//!   uncommitted and nothing in the workspace consumed it — that
//!   absence is exactly the #34 gap.
//!
//! FRI side (`oracle_layer::fri_query` / `fri_protocol`):
//! - `fri_prove` input: evaluation form over a power-of-two domain in
//!   natural two-adic order (for a primitive n-th root omega, element
//!   i+n/2 = -element i — the pairing `fri_fold_step` requires), plus a
//!   Fiat-Shamir `Challenger` (Poseidon2 width 16, rate 8) that derives
//!   the per-round betas and query indices, plus `num_queries`.
//! - Output: `FriProof` — one Merkle root per round, the final folded
//!   value, and per-query openings across every round.
//! - Verifier: `fri_verify` re-derives betas and indices from a fresh,
//!   identically-seeded challenger and checks every opening and fold.
//!
//! Mismatch class: form + missing call path. NOT a field mismatch
//! (BabyBear on both sides) and NOT a commitment-format conflict
//! (oracle-layer Merkle is the only commitment format in play; the
//! DEEP-ALI side simply had none yet, which is the gap itself). Per the
//! #34 guardrails this is recorded as wiring, not as a finding.
//!
//! Scope boundary — what this closes and what stays open. This module
//! closes the commitment/evaluation flow: the DEEP-ALI quotient now
//! reaches the real FRI prover and is committed through it. Full
//! DEEP-ALI soundness additionally requires (a) binding the quotient to
//! the trace (the quotient must equal the DEEP-ALI combination of
//! trace openings at sampled points against the trace commitment) and
//! (b) a single Fiat-Shamir challenge stream. Today the DEEP-ALI
//! combination weights come from `DeepAliChallenger` (Poseidon2 width
//! 24, rate 7) while FRI betas and query indices come from
//! `oracle_layer::fri_protocol::Challenger` (width 16, rate 8): two
//! separate transcripts. Unifying them is Issue #39; this module does
//! not silently pretend they are one.

use oracle_layer::fft::Radix2Interpolator;
use oracle_layer::fri_protocol::Challenger;
use oracle_layer::fri_query::{fri_prove, fri_verify, FriProof};
use p3_baby_bear::BabyBear;
use p3_field::{PrimeCharacteristicRing, PrimeField64};

/// Errors of the DEEP-ALI → FRI pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeepAliFriError {
    /// The pipeline was handed no quotient: the DEEP-ALI side produced
    /// an empty polynomial. Explicit rejection, never a silent accept
    /// — the wiring must not invent a proof from nothing.
    QuotientAbsent,
    /// FRI rejected the proof: tampered or forged committed
    /// evaluations, openings that do not match the claimed roots, or a
    /// transcript that does not re-derive the prover's challenges.
    FriRejected,
}

impl core::fmt::Display for DeepAliFriError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeepAliFriError::QuotientAbsent => {
                write!(
                    f,
                    "quotient absent: DEEP-ALI produced no polynomial to wire into FRI"
                )
            }
            DeepAliFriError::FriRejected => {
                write!(f, "FRI rejected the DEEP-ALI quotient proof")
            }
        }
    }
}

impl core::error::Error for DeepAliFriError {}

/// FRI output plus the wiring metadata a verifier needs.
pub struct DeepAliFriOutput {
    /// The FRI proof: commitment chain (one Merkle root per round,
    /// then the final folded value) plus per-query openings.
    pub proof: FriProof,
    /// The evaluation domain the quotient was committed over, in the
    /// same natural two-adic order the evaluations were produced in:
    /// element i + n/2 is the negation of element i.
    pub domain: Vec<BabyBear>,
    /// Quotient length BEFORE zero-padding (trimmed coefficient
    /// count, i.e. degree + 1).
    pub quotient_len: usize,
    /// Domain size: next power of two >= quotient_len, minimum 2.
    pub domain_size: usize,
}

/// A primitive n-th root of unity in BabyBear (n a power of two,
/// n <= 2^27), derived from the standard generator 31 — the same
/// convention as the oracle-layer FRI test fixtures.
fn primitive_root(n: usize) -> BabyBear {
    debug_assert!(n.is_power_of_two() && n >= 2);
    let g = BabyBear::from_u64(31);
    let exp = (BabyBear::ORDER_U64 - 1) / n as u64;
    g.exp_u64(exp)
}

/// The FRI evaluation domain: omega^0 .. omega^(n-1) in natural order.
/// For a primitive n-th root, omega^(n/2) = -1, so element i + n/2 is
/// the negation of element i — exactly the (x, -x) pairing that
/// `fri_fold_step` and `verify_fri_query_round` require, and exactly
/// the order `Radix2Interpolator::fft` produces evaluations in.
fn two_adic_domain(n: usize) -> Vec<BabyBear> {
    let omega = primitive_root(n);
    (0..n as u64).map(|i| omega.exp_u64(i)).collect()
}

/// Wire a DEEP-ALI combined quotient (coefficient form) into the FRI
/// prover.
///
/// The quotient is zero-padded to the FRI domain size n (the next
/// power of two above its trimmed length, minimum 2), evaluated over
/// the two-adic domain via FFT, and committed through the real FRI
/// commit + query phases. Because the FFT of the padded coefficient
/// vector yields exactly the evaluations at omega^0..omega^(n-1), the
/// evaluations and the domain line up element-by-element: `evals[i]`
/// is the quotient evaluated at `domain[i]`.
///
/// The challenger is the FRI transcript only. Per the scope boundary
/// above, the DEEP-ALI combination weights are sampled from the
/// DEEP-ALI challenger during quotient computation, before this
/// function is called.
pub fn deep_ali_fri_prove(
    quotient_coeffs: &[BabyBear],
    fri_challenger: &mut Challenger,
    num_queries: usize,
) -> Result<DeepAliFriOutput, DeepAliFriError> {
    if quotient_coeffs.is_empty() {
        return Err(DeepAliFriError::QuotientAbsent);
    }

    let n = quotient_coeffs.len().next_power_of_two().max(2);
    let mut padded = quotient_coeffs.to_vec();
    padded.resize(n, BabyBear::ZERO);

    let omega = primitive_root(n);
    let evals = Radix2Interpolator::fft(&padded, omega);
    let domain = two_adic_domain(n);

    let proof = fri_prove(evals, domain.clone(), fri_challenger, num_queries);

    Ok(DeepAliFriOutput {
        proof,
        domain,
        quotient_len: quotient_coeffs.len(),
        domain_size: n,
    })
}

/// Verify a DEEP-ALI → FRI output against a fresh, identically-seeded
/// FRI challenger: re-derives the betas and query indices from the
/// transcript and checks every round of every query against the
/// claimed roots.
pub fn deep_ali_fri_verify(
    out: &DeepAliFriOutput,
    fri_challenger: &mut Challenger,
    num_queries: usize,
) -> Result<(), DeepAliFriError> {
    if fri_verify(&out.domain, &out.proof, fri_challenger, num_queries) {
        Ok(())
    } else {
        Err(DeepAliFriError::FriRejected)
    }
}

// Re-exported for downstream pipeline code: the wiring consumes the
// DEEP-ALI producer's query type, and callers need both challenger
// types in scope to run the two-transcript flow this module documents.
pub use crate::deep_ali::BabyBearDeepAli;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deep_ali::{DeepAliChallenger, DeepAliVerifier, DeepQuery};
    use batch_merkle::BatchMerkle;
    use p3_baby_bear::{
        default_babybear_poseidon2_16, default_babybear_poseidon2_24, Poseidon2BabyBear,
    };
    use p3_challenger::DuplexChallenger;
    use p3_matrix::dense::RowMajorMatrix;
    use sha2::Digest;

    type DeepPermutation = Poseidon2BabyBear<24>;

    fn fresh_fri_challenger() -> Challenger {
        Challenger::new(default_babybear_poseidon2_16())
    }

    fn fresh_deep_challenger() -> DeepAliChallenger<DeepPermutation> {
        DuplexChallenger::new(default_babybear_poseidon2_24())
    }

    /// Deterministic pseudo-random trace (LCG fill), the same fill
    /// convention as the sumcheck verifier tests in this crate family.
    fn lcg_trace(rows: usize, cols: usize, seed: u32) -> RowMajorMatrix<BabyBear> {
        let mut x = seed;
        let values = (0..rows * cols)
            .map(|_| {
                x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                BabyBear::from_u32(x)
            })
            .collect();
        RowMajorMatrix::new(values, cols)
    }

    fn test_queries() -> Vec<DeepQuery<BabyBear>> {
        vec![
            DeepQuery {
                point: BabyBear::from_u32(101),
                trace_index: 0,
                column_index: 0,
            },
            DeepQuery {
                point: BabyBear::from_u32(202),
                trace_index: 1,
                column_index: 1,
            },
        ]
    }

    /// The OWSL bridge gates `compute_deep_quotient` behind a status
    /// file written by the Python OWSL daemon; a missing or stale file
    /// blocks verification by design. Tests therefore supply a
    /// genuine, fresh, correctly-hashed PROCEED status through the
    /// bridge's own reading path — the gate runs, it just reads a
    /// real status instead of an error.
    ///
    /// Issue #47 (resolved): the bridge now has an injection point —
    /// this suite passes its OWN status path (via
    /// BabyBearDeepAli::with_owsl_status_path), so no OWSL-dependent
    /// suite writes the process-global file and the shared-file race
    /// is structurally gone. Atomic publication lives in the bridge
    /// (publish_status_atomic), not re-derived here; the bounded
    /// retry loop the race forced is deleted — a gate block on an
    /// owned file is a genuine failure and fails the test loudly.
    fn write_owsl_proceed_status(path: &str) {
        // FINDING (recorded for the #34 PR body): serde_json's default
        // f64 parse is not correctly-rounded — the optional
        // `float_roundtrip` feature exists for exactly this — so a
        // fixture that embeds a fractional as_secs_f64() timestamp in
        // JSON and hashes over its Display digits breaks
        // value-dependently after the round-trip (measured: ~9% of a
        // dyadic-fraction sweep). The pre-existing OWSL fixtures in
        // owsl_bridge.rs and main.rs carry the same latent flake; the
        // workspace now enables float_roundtrip (separate commit). This
        // fixture additionally uses whole-second timestamps, which
        // parse exactly regardless: belt and suspenders.
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock")
            .as_secs() as f64;
        // Hash input uses the same format!("{}", ..) strings the
        // reader recomputes after parsing. Round/bits/frame/window
        // values chosen so both sides agree exactly; anomalies empty.
        // NOTE: window values are hashed as Display renders the parsed
        // f64 ("0" and "1000"), not as source literals — the reader
        // recomputes format!("{}", 0.0) = "0", format!("{}", 1000.0) =
        // "100" + "0". The JSON below carries 0.0/1000.0 which parse
        // back to exactly those displays.
        let hash_input = format!("{}|PROCEED|CONTINUE|1|0|4096||1|0|1000", timestamp);
        let content_hash = format!("{:x}", sha2::Sha256::digest(hash_input.as_bytes()));
        let json = format!(
            r#"{{"timestamp": {},"status": "PROCEED","action": "CONTINUE","round": 1,"bits_consumed": 0,"bits_remaining": 4096,"anomalies": [],"frame_count": 1,"window_start": 0.0,"window_end": 1000.0,"checksum_valid": true,"content_hash": "{}"}}"#,
            timestamp, content_hash
        );
        crate::owsl_bridge::publish_status_atomic(path, &json)
            .expect("atomically publish OWSL status");
    }

    /// Issue #47: each honest_pipeline run owns its OWSL status file —
    /// a fresh unique path per call, published through the bridge's
    /// own atomic publication helper. No suite shares mutable state
    /// with any other; the shared-file race (and the retry loop it
    /// forced) is structurally gone.
    fn owned_owsl_status_path(tag: &str) -> String {
        static PATH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = PATH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!(
            "{}/tscp_owsl_test_{}_{}_{}.json",
            std::env::temp_dir().display(),
            tag,
            std::process::id(),
            n
        )
    }

    /// Run the REAL DEEP-ALI producer over the trace (through the OWSL
    /// gate, through the combination-weight sampling), then wire its
    /// combined quotient into FRI. Returns the output.
    fn honest_pipeline(
        trace: &RowMajorMatrix<BabyBear>,
        queries: &[DeepQuery<BabyBear>],
        num_queries: usize,
        status_path: &str,
    ) -> DeepAliFriOutput {
        // Issue #47: this suite owns its status file (status_path is
        // unique per call), so there is no rival writer and no retry
        // policy — a gate block here is a genuine failure and fails
        // the test loudly on the first attempt.
        write_owsl_proceed_status(status_path);
        let prover: BabyBearDeepAli<BatchMerkle, DeepPermutation> =
            BabyBearDeepAli::new(64, 2).with_owsl_status_path(status_path);

        let mut deep_challenger = fresh_deep_challenger();
        let quotient = prover
            .compute_deep_quotient(trace, queries, &mut deep_challenger)
            .expect("honest DEEP-ALI quotient computation (OWSL gate passes on an owned file)");
        assert!(
            !quotient.is_empty(),
            "quotient from a real trace must have degree >= 0"
        );
        let mut prove_challenger = fresh_fri_challenger();
        deep_ali_fri_prove(&quotient, &mut prove_challenger, num_queries)
            .expect("wiring an honest quotient into FRI must succeed")
    }

    // ── Named test path 1: honest quotient → FRI accept ─────────────

    #[test]
    fn honest_quotient_fri_accept() {
        let trace = lcg_trace(8, 2, 0xBEEF);
        let out = honest_pipeline(
            &trace,
            &test_queries(),
            4,
            &owned_owsl_status_path("accept"),
        );
        let mut verify_challenger = fresh_fri_challenger();
        assert_eq!(
            deep_ali_fri_verify(&out, &mut verify_challenger, 4),
            Ok(()),
            "a DEEP-ALI quotient honestly wired into FRI must verify"
        );
    }

    // ── Named test path 2: tampered quotient → FRI reject ───────────

    #[test]
    fn tampered_quotient_fri_reject() {
        let trace = lcg_trace(8, 2, 0xBEEF);
        let mut out = honest_pipeline(
            &trace,
            &test_queries(),
            4,
            &owned_owsl_status_path("tamper"),
        );
        // Tamper a committed quotient evaluation: the initial round's
        // first opening at x. The opening no longer matches its Merkle
        // root and the fold arithmetic breaks with it. This is the
        // tamper FRI can and must catch — a lie in the committed
        // evaluations of the quotient.
        out.proof.query_proofs[0][0].opening_x.leaf_value += BabyBear::ONE;
        let mut verify_challenger = fresh_fri_challenger();
        assert_eq!(
            deep_ali_fri_verify(&out, &mut verify_challenger, 4),
            Err(DeepAliFriError::FriRejected),
            "a tampered committed quotient evaluation must be rejected"
        );
    }

    // ── Named test path 3: quotient absent → explicit error ────────

    #[test]
    fn missing_quotient_is_explicit_error() {
        let mut challenger = fresh_fri_challenger();
        let result = deep_ali_fri_prove(&[], &mut challenger, 4);
        assert!(
            matches!(result, Err(DeepAliFriError::QuotientAbsent)),
            "an absent quotient must produce the explicit QuotientAbsent error, \
             not a silent accept and not a panic"
        );
    }

    // ── Supplemental: transcript seed mismatch → reject ────────────

    #[test]
    fn transcript_seed_mismatch_rejects() {
        let trace = lcg_trace(8, 2, 0xBEEF);
        let out = honest_pipeline(&trace, &test_queries(), 4, &owned_owsl_status_path("seed"));
        // A verifier whose transcript diverges before the commit phase
        // re-derives different query indices than the prover's — the
        // proof cannot verify against it.
        let mut wrong_challenger = fresh_fri_challenger();
        use p3_challenger::CanObserve;
        wrong_challenger.observe(BabyBear::ONE);
        assert_eq!(
            deep_ali_fri_verify(&out, &mut wrong_challenger, 4),
            Err(DeepAliFriError::FriRejected),
            "a differently-seeded transcript must reject the proof"
        );
    }

    // ── Supplemental: wiring shape is honest ───────────────────────

    #[test]
    fn domain_shape_matches_padded_quotient() {
        let trace = lcg_trace(8, 2, 0xBEEF);
        let out = honest_pipeline(
            &trace,
            &test_queries(),
            4,
            &owned_owsl_status_path("domain"),
        );

        // Quotient of trimmed length 7 (8-row trace, degree 6) lands
        // on the next power-of-two domain, strictly larger than the
        // quotient.
        assert_eq!(out.quotient_len, 7);
        assert_eq!(out.domain_size, 8);
        assert_eq!(out.domain.len(), out.domain_size);
        assert!(out.quotient_len <= out.domain_size);

        // Negation-closed domain: element i + n/2 = -element i, the
        // pairing the FRI fold checks rely on.
        let half = out.domain_size / 2;
        let negation_closed =
            (0..half).all(|i| out.domain[i] + out.domain[i + half] == BabyBear::ZERO);
        assert!(
            negation_closed,
            "the FRI domain must be negation-closed (element i+n/2 = -element i)"
        );

        // One Merkle root per fold round plus the initial commitment.
        assert_eq!(
            out.proof.commitment.roots.len(),
            out.domain_size.trailing_zeros() as usize + 1
        );

        // And the shape checks are not vacuous: this output verifies.
        let mut verify_challenger = fresh_fri_challenger();
        assert_eq!(deep_ali_fri_verify(&out, &mut verify_challenger, 4), Ok(()));
    }

    // ── Supplemental: a second trace size exercises the padding ────

    #[test]
    fn honest_quotient_fri_accept_16_row_trace() {
        // A 16-row trace yields quotient length 15 → domain 16, one
        // more fold round than the 8-row case.
        let trace = lcg_trace(16, 2, 0x5EED);
        let queries = vec![
            DeepQuery {
                point: BabyBear::from_u32(77),
                trace_index: 2,
                column_index: 0,
            },
            DeepQuery {
                point: BabyBear::from_u32(154),
                trace_index: 5,
                column_index: 1,
            },
        ];
        let out = honest_pipeline(&trace, &queries, 6, &owned_owsl_status_path("padded"));
        assert_eq!(out.quotient_len, 15);
        assert_eq!(out.domain_size, 16);
        let mut verify_challenger = fresh_fri_challenger();
        assert_eq!(
            deep_ali_fri_verify(&out, &mut verify_challenger, 6),
            Ok(()),
            "a second trace size must also wire and verify"
        );
    }
}
