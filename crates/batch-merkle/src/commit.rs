//! BatchMerkle: a concrete batch vector-commitment scheme over BabyBear,
//! built on Plonky3's `MerkleTreeMmcs`. Supports committing to multiple
//! matrices/columns of differing heights and widths, opening a single row
//! index across all of them with one combined proof, and verifying that
//! proof against the commitment.

use p3_baby_bear::{default_babybear_poseidon2_16, BabyBear, Poseidon2BabyBear};
use p3_field::Field;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};

pub type F = BabyBear;
pub type Perm = Poseidon2BabyBear<16>;
pub type Hasher = PaddingFreeSponge<Perm, 16, 8, 8>;
pub type Compressor = TruncatedPermutation<Perm, 2, 8, 16>;

/// The concrete batch Merkle commitment scheme used across the TSCP stack.
/// `cap_height: 0` commits to the root only (no Merkle-cap optimization).
pub type BatchMerkle =
    MerkleTreeMmcs<<F as Field>::Packing, <F as Field>::Packing, Hasher, Compressor, 2, 8>;

/// Builds a `BatchMerkle` instance using fixed, deterministic Poseidon2
/// round constants (`default_babybear_poseidon2_16`) -- NOT a freshly
/// sampled random permutation. This is required for soundness: a
/// prover's commitment and a verifier's check of that commitment must
/// use the exact same hash function, or every opening will fail to
/// verify regardless of whether the data is honest. (An earlier version
/// of this function used `Perm::new_from_rng_128(&mut rng())`, which
/// silently broke every cross-call verification because each call
/// produced a different, incompatible hash function.)
pub fn new_batch_merkle() -> BatchMerkle {
    let perm = default_babybear_poseidon2_16();
    let hash = Hasher::new(perm.clone());
    let compress = Compressor::new(perm);
    BatchMerkle::new(hash, compress, 0)
}
