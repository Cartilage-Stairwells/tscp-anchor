#[allow(unused_imports)]
#[allow(unused_imports)]
use p3_field::{Field, PrimeCharacteristicRing, PrimeField32, PrimeField64};
use p3_baby_bear::BabyBear;
use p3_challenger::{CanObserve, CanSample};
use crate::merkle::{MerkleTree, MerkleOpening, verify_opening};
use crate::fri::{fri_fold_step, fold_domain};
use crate::fri_protocol::{Challenger, FriCommitment};

/// Everything needed to check one round's fold consistency at a single
/// query index, without the verifier ever seeing the full evaluation
/// vectors: the two openings from round i (at idx and idx+half) and
/// the one opening from round i+1 (at idx mod half).
pub struct FriQueryRound {
    pub opening_x: MerkleOpening,
    pub opening_neg_x: MerkleOpening,
    pub opening_folded: MerkleOpening,
}

/// Generates the query openings for a single round, given that round's
/// tree, the next round's tree, and a query index into the current
/// (pre-fold) evaluation vector.
pub fn fri_query_round(
    current_tree: &MerkleTree,
    next_tree: &MerkleTree,
    query_index: usize,
) -> FriQueryRound {
    let half = current_tree.leaves.len() / 2;
    let idx = query_index % half;
    FriQueryRound {
        opening_x: current_tree.open(idx),
        opening_neg_x: current_tree.open(idx + half),
        opening_folded: next_tree.open(idx),
    }
}

/// Verifies one round's fold consistency: that the folded value really
/// is p_even(x^2) + beta * p_odd(x^2), computed from the two openings
/// at x and -x, AND that all three openings are valid against their
/// respective claimed roots.
pub fn verify_fri_query_round(
    root_current: BabyBear,
    root_next: BabyBear,
    x: BabyBear,
    beta: BabyBear,
    query: &FriQueryRound,
) -> bool {
    if !verify_opening(root_current, &query.opening_x) {
        return false;
    }
    if !verify_opening(root_current, &query.opening_neg_x) {
        return false;
    }
    if !verify_opening(root_next, &query.opening_folded) {
        return false;
    }

    let two_inv = BabyBear::from_u64(2).inverse();
    let p_x = query.opening_x.leaf_value;
    let p_neg_x = query.opening_neg_x.leaf_value;

    let even_part = (p_x + p_neg_x) * two_inv;
    let odd_part = (p_x - p_neg_x) * two_inv * x.inverse();
    let expected_folded = even_part + beta * odd_part;

    query.opening_folded.leaf_value == expected_folded
}

/// A complete FRI proof: the commit-phase data (roots + final value)
/// plus, for each sampled query index, one `FriQueryRound` per fold
/// round showing that round's consistency at that index.
pub struct FriProof {
    pub commitment: FriCommitment,
    /// query_proofs[q] = the per-round openings for query index q.
    /// query_proofs[q][r] corresponds to fold round r.
    pub query_proofs: Vec<Vec<FriQueryRound>>,
    pub query_indices: Vec<usize>,
}

/// Runs the full FRI prover: commits round-by-round (deriving betas
/// from the transcript, same as `fri_commit_transcript`), then -- after
/// the full commitment is in the transcript -- samples `num_queries`
/// indices from the transcript and produces opening proofs for each
/// one across every round.
///
/// Querying only after the full commit phase is essential: it ensures
/// the prover committed to evaluations before learning which indices
/// would be checked, so it can't selectively cheat at unchecked points.
pub fn fri_prove(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    challenger: &mut Challenger,
    num_queries: usize,
) -> FriProof {
    let n = evals.len();
    assert!(n.is_power_of_two(), "initial evaluation count must be a power of two");
    let expected_rounds = n.trailing_zeros() as usize;

    let mut current_evals = evals;
    let mut current_domain = domain;
    let mut roots = Vec::with_capacity(expected_rounds + 1);
    let mut trees = Vec::with_capacity(expected_rounds + 1);

    let initial_tree = MerkleTree::build(current_evals.clone());
    let initial_root = initial_tree.root();
    challenger.observe(initial_root);
    roots.push(initial_root);
    trees.push(initial_tree);

    for _ in 0..expected_rounds {
        let beta: BabyBear = challenger.sample();
        current_evals = fri_fold_step(&current_evals, &current_domain, beta);
        current_domain = fold_domain(&current_domain);

        let tree = MerkleTree::build(current_evals.clone());
        let root = tree.root();
        challenger.observe(root);
        roots.push(root);
        trees.push(tree);
    }

    assert_eq!(current_evals.len(), 1, "after log2(n) folds exactly one value must remain");
    let commitment = FriCommitment { roots, final_value: current_evals[0] };

    // Query phase: sample indices only now, after every round's root
    // is already in the transcript.
    let initial_leaf_count = trees[0].leaves.len();
    let mut query_indices = Vec::with_capacity(num_queries);
    let mut query_proofs = Vec::with_capacity(num_queries);

    for _ in 0..num_queries {
        let raw: BabyBear = challenger.sample();
        let index = (raw.as_canonical_u32() as usize) % initial_leaf_count;
        query_indices.push(index);

        let mut rounds_for_this_query = Vec::with_capacity(expected_rounds);
        for r in 0..expected_rounds {
            rounds_for_this_query.push(fri_query_round(&trees[r], &trees[r + 1], index));
        }
        query_proofs.push(rounds_for_this_query);
    }

    FriProof { commitment, query_proofs, query_indices }
}

/// Runs the full FRI verifier: re-derives the same betas and query
/// indices from a fresh transcript (which must be seeded identically
/// to the prover's), then checks every round of every query proof
/// against the claimed roots.
///
/// `domain` is the original (pre-fold) evaluation domain -- needed to
/// recover the `x` value at each query index for each round's fold
/// check.
pub fn fri_verify(
    domain: &[BabyBear],
    proof: &FriProof,
    challenger: &mut Challenger,
    num_queries: usize,
) -> bool {
    let expected_rounds = proof.commitment.roots.len().saturating_sub(1);
    if expected_rounds == 0 {
        return false;
    }

    // Re-derive betas the same way the prover did, observing the same
    // roots in the same order.
    challenger.observe(proof.commitment.roots[0]);
    let mut betas = Vec::with_capacity(expected_rounds);
    for r in 0..expected_rounds {
        let beta: BabyBear = challenger.sample();
        betas.push(beta);
        challenger.observe(proof.commitment.roots[r + 1]);
    }

    // Re-derive query indices the same way.
    let initial_leaf_count = domain.len();
    let mut domains = Vec::with_capacity(expected_rounds);
    let mut current_domain = domain.to_vec();
    domains.push(current_domain.clone());
    for _ in 0..expected_rounds {
        current_domain = fold_domain(&current_domain);
        domains.push(current_domain.clone());
    }

    if proof.query_indices.len() != num_queries || proof.query_proofs.len() != num_queries {
        return false;
    }

    for q in 0..num_queries {
        let raw: BabyBear = challenger.sample();
        let expected_index = (raw.as_canonical_u32() as usize) % initial_leaf_count;
        if proof.query_indices[q] != expected_index {
            return false;
        }

        let index = proof.query_indices[q];
        if proof.query_proofs[q].len() != expected_rounds {
            return false;
        }

        for r in 0..expected_rounds {
            let root_current = proof.commitment.roots[r];
            let root_next = proof.commitment.roots[r + 1];
            let half = domains[r].len() / 2;
            let x = domains[r][index % half];
            let beta = betas[r];
            let query = &proof.query_proofs[q][r];

            if !verify_fri_query_round(root_current, root_next, x, beta, query) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::Radix2Interpolator;
    use crate::fri::{fri_fold_step};
    use p3_baby_bear::{BabyBear, default_babybear_poseidon2_16};

    type F = BabyBear;

    fn fresh_challenger() -> Challenger {
        Challenger::new(default_babybear_poseidon2_16())
    }

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

    fn setup_one_round() -> (MerkleTree, MerkleTree, Vec<BabyBear>, BabyBear) {
        let w = omega8();
        let domain = negation_closed_domain(w, 8);
        let coeffs = vec![F::from_u64(1), F::from_u64(2), F::from_u64(3), F::from_u64(4),
                           F::ZERO, F::ZERO, F::ZERO, F::ZERO];
        let evals: Vec<F> = domain.iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();
        let beta = F::from_u64(10);
        let folded = fri_fold_step(&evals, &domain, beta);

        let current_tree = MerkleTree::build(evals);
        let next_tree = MerkleTree::build(folded);
        (current_tree, next_tree, domain, beta)
    }

    #[test]
    fn honest_query_verifies_for_every_index() {
        let (current_tree, next_tree, domain, beta) = setup_one_round();
        let root_current = current_tree.root();
        let root_next = next_tree.root();

        for idx in 0..4 {
            let query = fri_query_round(&current_tree, &next_tree, idx);
            let x = domain[idx];
            assert!(verify_fri_query_round(root_current, root_next, x, beta, &query),
                "honest query at index {idx} must verify");
        }
    }

    #[test]
    fn tampered_folded_value_fails_verification() {
        let (current_tree, next_tree, domain, beta) = setup_one_round();
        let root_current = current_tree.root();
        let root_next = next_tree.root();

        let mut query = fri_query_round(&current_tree, &next_tree, 0);
        query.opening_folded.leaf_value = F::from_u64(999999);
        let x = domain[0];
        assert!(!verify_fri_query_round(root_current, root_next, x, beta, &query),
            "a tampered folded value must fail verification even if its Merkle path is internally consistent with a different root");
    }

    #[test]
    fn wrong_beta_fails_verification() {
        let (current_tree, next_tree, domain, beta) = setup_one_round();
        let root_current = current_tree.root();
        let root_next = next_tree.root();

        let query = fri_query_round(&current_tree, &next_tree, 0);
        let x = domain[0];
        let wrong_beta = beta + F::ONE;
        assert!(!verify_fri_query_round(root_current, root_next, x, wrong_beta, &query),
            "verifying with the wrong beta must fail, since the fold relation no longer holds");
    }

    #[test]
    fn mismatched_x_and_neg_x_openings_fails_verification() {
        let (current_tree, next_tree, domain, beta) = setup_one_round();
        let root_current = current_tree.root();
        let root_next = next_tree.root();

        let mut query = fri_query_round(&current_tree, &next_tree, 0);
        query.opening_neg_x = current_tree.open(2); // wrong pairing
        let x = domain[0];
        assert!(!verify_fri_query_round(root_current, root_next, x, beta, &query),
            "mismatched x/-x pairing must fail the fold relation check");
    }

    #[test]
    fn forged_opening_with_wrong_root_fails_even_with_correct_arithmetic() {
        let (current_tree, next_tree, _domain, beta) = setup_one_round();
        let root_next = next_tree.root();
        let bogus_root_current = F::from_u64(123456789);

        let query = fri_query_round(&current_tree, &next_tree, 0);
        let x = F::from_u64(1);
        assert!(!verify_fri_query_round(bogus_root_current, root_next, x, beta, &query),
            "verification must fail immediately if the current root doesn't match");
    }

    #[test]
    fn full_protocol_honest_proof_verifies() {
        let w = omega8();
        let mut domain = negation_closed_domain(w, 8);
        // Extend to a slightly bigger domain (16 points) so there's
        // more than one fold round to exercise the multi-round driver.
        // We rebuild from scratch with n=16 instead of reusing omega8.
        let g16 = {
            let base = F::from_u64(31);
            let exp = (F::ORDER_U64 - 1) / 16;
            base.exp_u64(exp)
        };
        let half = 8;
        let mut d: Vec<F> = (0..half as u64).map(|i| g16.exp_u64(i)).collect();
        let negs: Vec<F> = d.iter().map(|&x| F::ZERO - x).collect();
        d.extend(negs);
        domain = d;

        let coeffs = vec![
            F::from_u64(1), F::from_u64(2), F::from_u64(3), F::from_u64(4),
            F::from_u64(5), F::from_u64(6), F::from_u64(7), F::from_u64(8),
            F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO,
        ];
        let evals: Vec<F> = domain.iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();

        let num_queries = 5;

        let mut prover_challenger = fresh_challenger();
        let proof = fri_prove(evals, domain.clone(), &mut prover_challenger, num_queries);

        let mut verifier_challenger = fresh_challenger();
        assert!(fri_verify(&domain, &proof, &mut verifier_challenger, num_queries),
            "an honestly generated proof must verify against a freshly-seeded transcript");
    }

    #[test]
    fn full_protocol_tampered_query_opening_fails_verification() {
        let g16 = {
            let base = F::from_u64(31);
            let exp = (F::ORDER_U64 - 1) / 16;
            base.exp_u64(exp)
        };
        let half = 8;
        let mut d: Vec<F> = (0..half as u64).map(|i| g16.exp_u64(i)).collect();
        let negs: Vec<F> = d.iter().map(|&x| F::ZERO - x).collect();
        d.extend(negs);
        let domain = d;

        let coeffs = vec![
            F::from_u64(2), F::from_u64(0), F::from_u64(1), F::from_u64(3),
            F::from_u64(0), F::from_u64(0), F::from_u64(0), F::from_u64(0),
            F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO,
        ];
        let evals: Vec<F> = domain.iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();

        let num_queries = 4;
        let mut prover_challenger = fresh_challenger();
        let mut proof = fri_prove(evals, domain.clone(), &mut prover_challenger, num_queries);

        // Tamper with one opening in one query's first round.
        proof.query_proofs[0][0].opening_x.leaf_value = F::from_u64(7777777);

        let mut verifier_challenger = fresh_challenger();
        assert!(!fri_verify(&domain, &proof, &mut verifier_challenger, num_queries),
            "a tampered query opening must cause verification to fail");
    }

    #[test]
    fn full_protocol_wrong_query_indices_fail_verification() {
        let g16 = {
            let base = F::from_u64(31);
            let exp = (F::ORDER_U64 - 1) / 16;
            base.exp_u64(exp)
        };
        let half = 8;
        let mut d: Vec<F> = (0..half as u64).map(|i| g16.exp_u64(i)).collect();
        let negs: Vec<F> = d.iter().map(|&x| F::ZERO - x).collect();
        d.extend(negs);
        let domain = d;

        let coeffs = vec![
            F::from_u64(1), F::from_u64(1), F::from_u64(1), F::from_u64(1),
            F::from_u64(0), F::from_u64(0), F::from_u64(0), F::from_u64(0),
            F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO,
        ];
        let evals: Vec<F> = domain.iter()
            .map(|&x| Radix2Interpolator::evaluate_coeffs(&coeffs, x))
            .collect();

        let num_queries = 4;
        let mut prover_challenger = fresh_challenger();
        let mut proof = fri_prove(evals, domain.clone(), &mut prover_challenger, num_queries);

        // A malicious prover swaps in a different (but still in-range)
        // query index than the one the transcript actually committed to.
        proof.query_indices[0] = (proof.query_indices[0] + 1) % 8;

        let mut verifier_challenger = fresh_challenger();
        assert!(!fri_verify(&domain, &proof, &mut verifier_challenger, num_queries),
            "substituting a different query index than the transcript-derived one must fail verification");
    }
}