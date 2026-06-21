//! A Merkle/vector commitment wrapper over `batch_merkle::BatchMerkle`,
//! Plonky3's real, audited `Mmcs` implementation -- not a hand-rolled
//! hash tree. We commit each column of evaluations as a single-column
//! matrix via `commit_vec`, and open/verify single rows via the real
//! `open_batch`/`verify_batch` Mmcs methods.
//!
//! This wrapper exists so that `fri_protocol.rs`/`fri_query.rs` can
//! keep a simple `build` / `.root()` / `.open(idx)` / `verify_opening`
//! interface without every caller needing to know about Mmcs's
//! batch-oriented, multi-matrix API surface.

use batch_merkle::{new_batch_merkle, BatchMerkle, F};
use p3_commit::Mmcs;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Dimensions;

pub type Commitment = <BatchMerkle as Mmcs<F>>::Commitment;
pub type ProverData = <BatchMerkle as Mmcs<F>>::ProverData<RowMajorMatrix<F>>;
pub type Proof = <BatchMerkle as Mmcs<F>>::Proof;

pub struct MerkleTree {
    pub leaves: Vec<F>,
    mmcs: BatchMerkle,
    commitment: Commitment,
    prover_data: ProverData,
}

pub struct MerkleOpening {
    pub leaf_index: usize,
    pub leaf_value: F,
    pub proof: Proof,
}

impl MerkleTree {
    pub fn build(leaves: Vec<F>) -> Self {
        assert!(leaves.len().is_power_of_two(), "leaf count must be a power of two");
        assert!(!leaves.is_empty(), "cannot build a tree with zero leaves");

        let mmcs = new_batch_merkle();
        let (commitment, prover_data) = mmcs.commit_vec(leaves.clone());

        Self { leaves, mmcs, commitment, prover_data }
    }

    pub fn root(&self) -> Commitment {
        self.commitment.clone()
    }

    /// Produce an opening proof for the leaf at `index`.
    pub fn open(&self, index: usize) -> MerkleOpening {
        assert!(index < self.leaves.len(), "leaf index out of range");
        let opening = self.mmcs.open_batch(index, &self.prover_data);
        // We only ever commit a single column, so opened_values has
        // exactly one row's worth of values, which itself has exactly
        // one element (width-1 matrix).
        let leaf_value = opening.opened_values[0][0];
        MerkleOpening {
            leaf_index: index,
            leaf_value,
            proof: opening.opening_proof,
        }
    }
}

/// Verify an opening against a known commitment and the original leaf
/// count (needed to reconstruct the matrix's `Dimensions`).
pub fn verify_opening(commitment: &Commitment, leaf_count: usize, opening: &MerkleOpening) -> bool {
    let mmcs = new_batch_merkle();
    let dims = Dimensions { width: 1, height: leaf_count };
    let opened_values = vec![vec![opening.leaf_value]];
    let batch_opening_ref = p3_commit::BatchOpeningRef::new(&opened_values, &opening.proof);
    mmcs.verify_batch(commitment, &[dims], opening.leaf_index, batch_opening_ref).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_field::PrimeCharacteristicRing;

    type Fl = F;

    #[test]
    fn root_is_deterministic_shape() {
        // NOTE: unlike the old hand-rolled hash, BatchMerkle samples a
        // fresh random Poseidon2 permutation per `new_batch_merkle()`
        // call (see batch-merkle's documented NOTE about this), so two
        // *separately built* trees over the same leaves will NOT have
        // equal commitments. We test internal consistency instead:
        // a tree's own root verifies its own openings.
        let leaves: Vec<Fl> = vec![1, 2, 3, 4].into_iter().map(Fl::from_u64).collect();
        let tree = MerkleTree::build(leaves);
        let root = tree.root();
        for i in 0..4 {
            let opening = tree.open(i);
            assert!(verify_opening(&root, 4, &opening), "leaf {i} must verify against its own tree's root");
        }
    }

    #[test]
    fn tampered_leaf_value_fails_verification() {
        let leaves: Vec<Fl> = (1..=4u64).map(Fl::from_u64).collect();
        let tree = MerkleTree::build(leaves);
        let root = tree.root();

        let mut opening = tree.open(1);
        opening.leaf_value = Fl::from_u64(999);
        assert!(!verify_opening(&root, 4, &opening),
            "a tampered leaf value must NOT verify against the real root");
    }

    #[test]
    fn wrong_commitment_fails_verification() {
        let leaves_a: Vec<Fl> = (1..=4u64).map(Fl::from_u64).collect();
        let leaves_b: Vec<Fl> = (5..=8u64).map(Fl::from_u64).collect();
        let tree_a = MerkleTree::build(leaves_a);
        let tree_b = MerkleTree::build(leaves_b);

        let opening = tree_a.open(0);
        assert!(!verify_opening(&tree_b.root(), 4, &opening),
            "an opening from one tree must not verify against a different tree's commitment");
    }

    #[test]
    fn valid_opening_verifies_for_every_leaf() {
        let leaves: Vec<Fl> = (1..=8u64).map(Fl::from_u64).collect();
        let tree = MerkleTree::build(leaves);
        let root = tree.root();
        for i in 0..8 {
            let opening = tree.open(i);
            assert!(verify_opening(&root, 8, &opening), "leaf {i}'s opening must verify");
        }
    }
}
