use crate::fri::{fold_domain, fri_fold_step};
use crate::merkle::{Commitment, MerkleTree};
use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_challenger::{CanObserve, CanSample, DuplexChallenger};
#[allow(unused_imports)]
use p3_field::{Field, PrimeCharacteristicRing, PrimeField64};

/// The Fiat-Shamir transcript type used to derive FRI challenges.
/// Matches the pattern already proven out in prover-server.
pub type Challenger = DuplexChallenger<BabyBear, Poseidon2BabyBear<16>, 16, 8>;

/// The full output of running FRI's commit phase: one Merkle root per
/// round (including the initial commitment to the unfolded
/// evaluations), and the final constant value the polynomial folds
/// down to.
pub struct FriCommitment {
    pub roots: Vec<Commitment>, // roots[0] = initial commitment, roots.last() = final round's tree (single leaf)
    pub final_value: BabyBear,
}

/// Runs the FRI commit phase: repeatedly fold the evaluations in half,
/// committing to each round's evaluations via Merkle tree, until a
/// single value remains.
///
/// `betas` must supply exactly log2(initial evals.len()) challenges,
/// one per fold round — passed in explicitly here rather than derived
/// from a transcript, so this function's correctness can be checked
/// independently of any Fiat-Shamir wiring.
pub fn fri_commit(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    betas: &[BabyBear],
) -> FriCommitment {
    let n = evals.len();
    assert!(
        n.is_power_of_two(),
        "initial evaluation count must be a power of two"
    );
    let expected_rounds = n.trailing_zeros() as usize;
    assert_eq!(
        betas.len(),
        expected_rounds,
        "must supply exactly log2(n) = {expected_rounds} challenges, got {}",
        betas.len()
    );

    let mut current_evals = evals;
    let mut current_domain = domain;
    let mut roots = Vec::with_capacity(expected_rounds + 1);

    // Commit to the initial, unfolded evaluations.
    let initial_tree = MerkleTree::build(current_evals.clone());
    roots.push(initial_tree.root());

    for &beta in betas {
        current_evals = fri_fold_step(&current_evals, &current_domain, beta);
        current_domain = fold_domain(&current_domain);

        let tree = MerkleTree::build(current_evals.clone());
        roots.push(tree.root());
    }

    assert_eq!(
        current_evals.len(),
        1,
        "after log2(n) folds exactly one value must remain"
    );
    FriCommitment {
        roots,
        final_value: current_evals[0],
    }
}

/// Recomputes the same commit phase independently (no shared state
/// with `fri_commit`) purely from the original evaluations, domain,
/// and betas, and checks that every round's root and the final value
/// match. This is the consistency check a verifier ultimately relies
/// on: that re-deriving the same protocol from the same transcript of
/// challenges reproduces the same commitment.
pub fn fri_verify_commitment(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    betas: &[BabyBear],
    claimed: &FriCommitment,
) -> bool {
    let recomputed = fri_commit(evals, domain, betas);
    recomputed.roots == claimed.roots && recomputed.final_value == claimed.final_value
}

/// Fiat-Shamir variant of `fri_commit`: instead of taking the betas as
/// caller-supplied data, this derives each round's beta from a
/// transcript that has observed that round's Merkle root. This is what
/// makes FRI non-interactive and sound against an adaptive prover: the
/// prover cannot choose evaluations after seeing beta, because beta
/// itself is a deterministic function of the prover's own commitment.
///
/// The transcript must already have observed anything that should be
/// bound into the challenges (e.g. public parameters, the original
/// polynomial's claimed degree) before this function is called.
pub fn fri_commit_transcript(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    challenger: &mut Challenger,
) -> FriCommitment {
    let n = evals.len();
    assert!(
        n.is_power_of_two(),
        "initial evaluation count must be a power of two"
    );
    let expected_rounds = n.trailing_zeros() as usize;

    let mut current_evals = evals;
    let mut current_domain = domain;
    let mut roots = Vec::with_capacity(expected_rounds + 1);

    let initial_tree = MerkleTree::build(current_evals.clone());
    let initial_root = initial_tree.root();
    challenger.observe(initial_root.clone());
    roots.push(initial_root);

    for _ in 0..expected_rounds {
        let beta: BabyBear = challenger.sample();

        current_evals = fri_fold_step(&current_evals, &current_domain, beta);
        current_domain = fold_domain(&current_domain);

        let tree = MerkleTree::build(current_evals.clone());
        let root = tree.root();
        challenger.observe(root.clone());
        roots.push(root);
    }

    assert_eq!(
        current_evals.len(),
        1,
        "after log2(n) folds exactly one value must remain"
    );
    FriCommitment {
        roots,
        final_value: current_evals[0],
    }
}

/// Recomputes `fri_commit_transcript` independently from a fresh
/// transcript seeded the same way, and checks it matches the claimed
/// commitment. Because betas are derived (not supplied), the verifier
/// re-derives them by observing the same roots in the same order.
pub fn fri_verify_commitment_transcript(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    challenger: &mut Challenger,
    claimed: &FriCommitment,
) -> bool {
    let recomputed = fri_commit_transcript(evals, domain, challenger);
    recomputed.roots == claimed.roots && recomputed.final_value == claimed.final_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::Radix2Interpolator;
    use p3_baby_bear::{default_babybear_poseidon2_16, BabyBear};

    fn fresh_challenger() -> Challenger {
        Challenger::new(default_babybear_poseidon2_16())
    }

    type F = BabyBear;

    fn omega8() -> F {
        let g = F::from_u64(31);
        let exp = (F::ORDER_U64 - 1) / 8;
        g.exp_u64(exp)
    }

    fn negation_closed_domain(w: F, n: usize) -> Vec<F> {
        let half = n / 2;
        let mut d: Vec<F> = (0..half as u64).map(|i| w.exp_u64(i)).collect();
        let negs: Vec<F> = d.iter().map(|&x| F::ZERO - x).collect();
        d.extend(negs);
        d
    }

    #[test]
    fn commit_on_constant_polynomial_ends_at_that_constant() {
        // p(x) = 7 everywhere (degree 0). After any number of folds,
        // the final value must still be 7, since p_even = 7 and
        // p_odd = 0 at every stage, so beta never matters and the
        // even part is preserved exactly.
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let evals = vec![F::from_u64(7); 8];
        let betas = vec![F::from_u64(3), F::from_u64(5), F::from_u64(11)];

        let commitment = fri_commit(evals, domain, &betas);
        assert_eq!(
            commitment.final_value,
            F::from_u64(7),
            "a constant polynomial must fold down to its own constant value"
        );
        assert_eq!(
            commitment.roots.len(),
            4,
            "must have 1 initial + 3 round roots for n=8"
        );
    }

    #[test]
    fn commit_matches_independent_recomputation() {
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let coeffs = vec![
            F::from_u64(1),
            F::from_u64(2),
            F::from_u64(3),
            F::from_u64(4),
            F::ZERO,
            F::ZERO,
            F::ZERO,
            F::ZERO,
        ];
        let evals: Vec<F> = domain
            .iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();
        let betas = vec![F::from_u64(10), F::from_u64(20), F::from_u64(30)];

        let commitment = fri_commit(evals.clone(), domain.clone(), &betas);
        assert!(
            fri_verify_commitment(evals, domain, &betas, &commitment),
            "independently recomputing the same commit phase must match exactly"
        );
    }

    #[test]
    fn different_betas_produce_different_roots_and_final_value() {
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let coeffs = vec![
            F::from_u64(1),
            F::from_u64(2),
            F::from_u64(3),
            F::from_u64(4),
            F::ZERO,
            F::ZERO,
            F::ZERO,
            F::ZERO,
        ];
        let evals: Vec<F> = domain
            .iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();

        let betas_a = vec![F::from_u64(1), F::from_u64(1), F::from_u64(1)];
        let betas_b = vec![F::from_u64(2), F::from_u64(2), F::from_u64(2)];

        let commit_a = fri_commit(evals.clone(), domain.clone(), &betas_a);
        let commit_b = fri_commit(evals, domain, &betas_b);

        assert_ne!(
            commit_a.final_value, commit_b.final_value,
            "different challenge sequences must produce different final values"
        );
    }

    #[test]
    fn tampering_with_evaluations_changes_the_initial_root_and_propagates() {
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let coeffs = vec![
            F::from_u64(1),
            F::from_u64(2),
            F::from_u64(3),
            F::from_u64(4),
            F::ZERO,
            F::ZERO,
            F::ZERO,
            F::ZERO,
        ];
        let evals_honest: Vec<F> = domain
            .iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();
        let mut evals_tampered = evals_honest.clone();
        evals_tampered[3] = F::from_u64(999999);

        let betas = vec![F::from_u64(7), F::from_u64(7), F::from_u64(7)];
        let commit_honest = fri_commit(evals_honest, domain.clone(), &betas);
        let commit_tampered = fri_commit(evals_tampered, domain, &betas);

        assert_ne!(
            commit_honest.roots[0], commit_tampered.roots[0],
            "tampering with input evaluations must change the initial commitment root"
        );
        assert_ne!(
            commit_honest.final_value, commit_tampered.final_value,
            "tampering must also generally change the final folded value"
        );
    }

    #[test]
    #[should_panic(expected = "must supply exactly log2(n)")]
    fn wrong_number_of_betas_panics() {
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let evals = vec![F::ONE; 8];
        let too_few_betas = vec![F::from_u64(1), F::from_u64(2)]; // need 3, gave 2
        fri_commit(evals, domain, &too_few_betas);
    }

    #[test]
    fn transcript_commit_is_deterministic_for_identical_inputs() {
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let coeffs = vec![
            F::from_u64(1),
            F::from_u64(2),
            F::from_u64(3),
            F::from_u64(4),
            F::ZERO,
            F::ZERO,
            F::ZERO,
            F::ZERO,
        ];
        let evals: Vec<F> = domain
            .iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();

        let mut challenger_a = fresh_challenger();
        let commit_a = fri_commit_transcript(evals.clone(), domain.clone(), &mut challenger_a);

        let mut challenger_b = fresh_challenger();
        let commit_b = fri_commit_transcript(evals, domain, &mut challenger_b);

        assert_eq!(commit_a.roots, commit_b.roots,
            "identical evals/domain with freshly-seeded transcripts must derive identical betas and roots");
        assert_eq!(commit_a.final_value, commit_b.final_value);
    }

    #[test]
    fn transcript_commit_verifies_against_fresh_transcript() {
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let coeffs = vec![
            F::from_u64(5),
            F::from_u64(1),
            F::from_u64(0),
            F::from_u64(2),
            F::ZERO,
            F::ZERO,
            F::ZERO,
            F::ZERO,
        ];
        let evals: Vec<F> = domain
            .iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();

        let mut prover_challenger = fresh_challenger();
        let commitment =
            fri_commit_transcript(evals.clone(), domain.clone(), &mut prover_challenger);

        let mut verifier_challenger = fresh_challenger();
        assert!(fri_verify_commitment_transcript(evals, domain, &mut verifier_challenger, &commitment),
            "verifier re-deriving betas from the same transcript protocol must reproduce the prover's commitment");
    }

    #[test]
    fn tampering_with_evaluations_changes_the_derived_betas_and_root() {
        // This is the key Fiat-Shamir soundness property: since beta is
        // derived from the root (which depends on the evaluations), a
        // prover cannot tamper with evaluations without also changing
        // the challenges used to fold them -- unlike the old caller-
        // supplied-betas version, where tampered evals could still be
        // folded with the same betas as the honest run.
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let coeffs = vec![
            F::from_u64(1),
            F::from_u64(2),
            F::from_u64(3),
            F::from_u64(4),
            F::ZERO,
            F::ZERO,
            F::ZERO,
            F::ZERO,
        ];
        let evals_honest: Vec<F> = domain
            .iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();
        let mut evals_tampered = evals_honest.clone();
        evals_tampered[3] = F::from_u64(999999);

        let mut challenger_honest = fresh_challenger();
        let commit_honest =
            fri_commit_transcript(evals_honest, domain.clone(), &mut challenger_honest);

        let mut challenger_tampered = fresh_challenger();
        let commit_tampered =
            fri_commit_transcript(evals_tampered, domain, &mut challenger_tampered);

        assert_ne!(
            commit_honest.roots[0], commit_tampered.roots[0],
            "tampered evaluations must produce a different initial root"
        );
        assert_ne!(commit_honest.roots, commit_tampered.roots,
            "since later betas are derived from earlier roots, tampering must cascade into different later roots too");
    }
}
