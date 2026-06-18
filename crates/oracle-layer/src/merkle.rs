#[allow(unused_imports)]
#[allow(unused_imports)]
use p3_field::PrimeCharacteristicRing;
use p3_field::{Field, PrimeField64};

/// A minimal Merkle tree over field elements, using a simple
/// multiplicative+additive mixing function as a placeholder hash.
///
/// NOTE: this is NOT cryptographically secure — it is a structural
/// scaffold to get tree construction, root computation, and opening
/// verification correct and tested first. The real implementation
/// should replace `hash_pair` with a proper sponge (e.g. Poseidon2
/// via p3_symmetric) before this is used for anything load-bearing.
pub struct MerkleTree<F> {
    pub leaves: Vec<F>,
    pub levels: Vec<Vec<F>>, // levels[0] = leaves, levels.last() = [root]
}

pub struct MerkleOpening<F> {
    pub leaf_index: usize,
    pub leaf_value: F,
    pub siblings: Vec<F>, // one sibling per level, bottom to top
}

fn hash_pair<F: Field + PrimeField64>(a: F, b: F) -> F {
    // Placeholder mixing function: NOT a secure hash. Deterministic,
    // order-sensitive (hash(a,b) != hash(b,a)), and collision-prone in
    // ways a real hash isn't. Replace before any real soundness claim.
    let av = a.as_canonical_u64();
    let bv = b.as_canonical_u64();
    let mixed = av.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(bv).rotate_left(17) ^ bv.wrapping_mul(0xC2B2AE3D27D4EB4F);
    F::from_u64(mixed)
}

impl<F: Field + PrimeField64 + Copy> MerkleTree<F> {
    pub fn build(leaves: Vec<F>) -> Self {
        assert!(leaves.len().is_power_of_two(), "leaf count must be a power of two");
        assert!(!leaves.is_empty(), "cannot build a tree with zero leaves");

        let mut levels = vec![leaves.clone()];
        let mut current = leaves.clone();
        while current.len() > 1 {
            let next: Vec<F> = current
                .chunks(2)
                .map(|pair| hash_pair(pair[0], pair[1]))
                .collect();
            levels.push(next.clone());
            current = next;
        }

        Self { leaves, levels }
    }

    pub fn root(&self) -> F {
        *self.levels.last().unwrap().last().unwrap()
    }

    /// Produce an opening proof for the leaf at `index`: the leaf value
    /// plus the sibling at each level needed to recompute the root.
    pub fn open(&self, index: usize) -> MerkleOpening<F> {
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
pub fn verify_opening<F: Field + PrimeField64 + Copy>(
    root: F,
    opening: &MerkleOpening<F>,
) -> bool {
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
    use p3_baby_bear::BabyBear;

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
