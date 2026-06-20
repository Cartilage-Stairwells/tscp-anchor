use std::sync::OnceLock;

use p3_baby_bear::{default_babybear_poseidon2_16, BabyBear, Poseidon2BabyBear};
use p3_field::PrimeCharacteristicRing;
use p3_symmetric::{PseudoCompressionFunction, TruncatedPermutation};

/// A Merkle tree over BabyBear field elements, using a real
/// Poseidon2-based 2-to-1 compression function for hashing.
///
/// The compressor is built once from fixed, deterministic Poseidon2
/// round constants (`default_babybear_poseidon2_16`) — not randomized
/// per-call — so the same inputs always produce the same root, which
/// is required for a prover and verifier to ever agree.
///
/// NOTE: this is specialized to `BabyBear` rather than generic over
/// any field, since the Poseidon2 permutation's round constants are
/// themselves field-specific. The earlier placeholder hash was field-
/// generic only because it used raw integer arithmetic, not a real
/// cryptographic primitive.
pub struct MerkleTree {
    pub leaves: Vec<BabyBear>,
    pub levels: Vec<Vec<BabyBear>>, // levels[0] = leaves, levels.last() = [root]
}

pub struct MerkleOpening {
    pub leaf_index: usize,
    pub leaf_value: BabyBear,
    pub siblings: Vec<BabyBear>, // one sibling per level, bottom to top
}

type Perm = Poseidon2BabyBear<16>;
type Compressor = TruncatedPermutation<Perm, 2, 8, 16>;

fn compressor() -> &'static Compressor {
    static COMPRESSOR: OnceLock<Compressor> = OnceLock::new();
    COMPRESSOR.get_or_init(|| Compressor::new(default_babybear_poseidon2_16()))
}

fn hash_pair(a: BabyBear, b: BabyBear) -> BabyBear {
    // Pad each scalar into an 8-element chunk (the compressor's CHUNK
    // width), compress the two chunks via the real Poseidon2
    // permutation, and take the first output element as the result.
    // Padding with zeros is fine here: a/b occupy a fixed position
    // (index 0) in an otherwise-zero chunk, so distinct (a, b) pairs
    // map to distinct compressor inputs.
    let mut chunk_a = [BabyBear::ZERO; 8];
    let mut chunk_b = [BabyBear::ZERO; 8];
    chunk_a[0] = a;
    chunk_b[0] = b;
    let out = compressor().compress([chunk_a, chunk_b]);
    out[0]
}

impl MerkleTree {
    pub fn build(leaves: Vec<BabyBear>) -> Self {
        assert!(leaves.len().is_power_of_two(), "leaf count must be a power of two");
        assert!(!leaves.is_empty(), "cannot build a tree with zero leaves");

        let mut levels = vec![leaves.clone()];
        let mut current = leaves.clone();
        while current.len() > 1 {
            let next: Vec<BabyBear> = current
                .chunks(2)
                .map(|pair| hash_pair(pair[0], pair[1]))
                .collect();
            levels.push(next.clone());
            current = next;
        }

        Self { leaves, levels }
    }

    pub fn root(&self) -> BabyBear {
        *self.levels.last().unwrap().last().unwrap()
    }

    /// Produce an opening proof for the leaf at `index`: the leaf value
    /// plus the sibling at each level needed to recompute the root.
    pub fn open(&self, index: usize) -> MerkleOpening {
        assert!(index < self.leaves.len(), "leaf index out of range");
        let mut siblings = Vec::new();
        let mut idx = index;
        for level in &self.levels[..self.levels.len() - 1] {
            let sibling_idx = idx ^ 1; // flip last bit to get the pair partner
            siblings.push(level[sibling_idx]);
            idx /= 2;
        }
        MerkleOpening {
            leaf_index: index,
            leaf_value: self.leaves[index],
            siblings,
        }
    }
}

/// Verify an opening against a known root, without needing the full
/// tree — this is what a real verifier would call.
pub fn verify_opening(root: BabyBear, opening: &MerkleOpening) -> bool {
    let mut current = opening.leaf_value;
    let mut idx = opening.leaf_index;
    for &sibling in &opening.siblings {
        current = if idx % 2 == 0 {
            hash_pair(current, sibling)
        } else {
            hash_pair(sibling, current)
        };
        idx /= 2;
    }
    current == root
}

#[cfg(test)]
mod tests {
    use super::*;

    type F = BabyBear;

    #[test]
    fn root_is_deterministic() {
        let leaves: Vec<F> = vec![1, 2, 3, 4].into_iter().map(F::from_u64).collect();
        let tree_a = MerkleTree::build(leaves.clone());
        let tree_b = MerkleTree::build(leaves);
        assert_eq!(tree_a.root(), tree_b.root(), "same leaves must produce the same root");
    }

    #[test]
    fn changing_one_leaf_changes_the_root() {
        let leaves_a: Vec<F> = vec![1, 2, 3, 4].into_iter().map(F::from_u64).collect();
        let mut leaves_b = leaves_a.clone();
        leaves_b[2] = F::from_u64(999);

        let tree_a = MerkleTree::build(leaves_a);
        let tree_b = MerkleTree::build(leaves_b);
        assert_ne!(tree_a.root(), tree_b.root(), "changing any leaf must change the root");
    }

    #[test]
    fn valid_opening_verifies_for_every_leaf() {
        let leaves: Vec<F> = (1..=8u64).map(F::from_u64).collect();
        let tree = MerkleTree::build(leaves);
        let root = tree.root();

        for i in 0..8 {
            let opening = tree.open(i);
            assert!(verify_opening(root, &opening),
                "leaf {i}'s opening must verify against the real root");
        }
    }

    #[test]
    fn tampered_leaf_value_fails_verification() {
        let leaves: Vec<F> = (1..=4u64).map(F::from_u64).collect();
        let tree = MerkleTree::build(leaves);
        let root = tree.root();

        let mut opening = tree.open(1);
        opening.leaf_value = F::from_u64(999); // tamper with the claimed leaf
        assert!(!verify_opening(root, &opening),
            "a tampered leaf value must NOT verify against the real root");
    }

    #[test]
    fn tampered_sibling_fails_verification() {
        let leaves: Vec<F> = (1..=4u64).map(F::from_u64).collect();
        let tree = MerkleTree::build(leaves);
        let root = tree.root();

        let mut opening = tree.open(0);
        opening.siblings[0] = F::from_u64(12345); // tamper with a sibling
        assert!(!verify_opening(root, &opening),
            "a tampered sibling must NOT verify against the real root");
    }

    #[test]
    fn wrong_root_fails_verification() {
        let leaves: Vec<F> = (1..=4u64).map(F::from_u64).collect();
        let tree = MerkleTree::build(leaves);
        let opening = tree.open(0);

        let wrong_root = F::from_u64(42);
        assert!(!verify_opening(wrong_root, &opening),
            "opening must not verify against an unrelated root");
    }
}
