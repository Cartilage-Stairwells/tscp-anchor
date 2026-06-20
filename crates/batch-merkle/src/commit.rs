//! BatchMerkle: a concrete batch vector-commitment scheme over BabyBear,
//! built on Plonky3's `MerkleTreeMmcs`. Supports committing to multiple
//! matrices/columns of differing heights and widths, opening a single row
//! index across all of them with one combined proof, and verifying that
//! proof against the commitment.

use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_field::Field;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};
use rand::rng;

pub type F = BabyBear;
pub type Perm = Poseidon2BabyBear<16>;
pub type Hasher = PaddingFreeSponge<Perm, 16, 8, 8>;
pub type Compressor = TruncatedPermutation<Perm, 2, 8, 16>;

/// The concrete batch Merkle commitment scheme used across the TSCP stack.
/// `cap_height: 0` commits to the root only (no Merkle-cap optimization).
pub type BatchMerkle =
    MerkleTreeMmcs<<F as Field>::Packing, <F as Field>::Packing, Hasher, Compressor, 2, 8>;

/// Builds a `BatchMerkle` instance with a freshly sampled Poseidon2 permutation.
/// NOTE: for production use (anything that needs reproducible / agreed-upon
/// parameters between prover and verifier), this should take a fixed seed or
/// a permutation constructed from public parameters instead of `rng()`.
pub fn new_batch_merkle() -> BatchMerkle {
    let perm = Perm::new_from_rng_128(&mut rng());
    let hash = Hasher::new(perm.clone());
    let compress = Compressor::new(perm);
    BatchMerkle::new(hash, compress, 0)
}
