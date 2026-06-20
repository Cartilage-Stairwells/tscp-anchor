#[allow(unused_imports)]
#[allow(unused_imports)]
use p3_field::{Field, PrimeCharacteristicRing, PrimeField64};
use p3_baby_bear::BabyBear;
use crate::merkle::{MerkleTree, MerkleOpening, verify_opening};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::Radix2Interpolator;
    use crate::fri::{fri_fold_step};
    use p3_baby_bear::BabyBear;

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
}
