use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_field::Field;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};

pub type F = BabyBear;
pub type Perm = Poseidon2BabyBear<16>;
pub type Hasher = PaddingFreeSponge<Perm, 16, 8, 8>;
pub type Compressor = TruncatedPermutation<Perm, 2, 2, 16>;

/// N=8: leaves per internal node. DIGEST_ELEMS=8: matches Hasher output width.
pub type BatchMerkle =
    MerkleTreeMmcs<<F as Field>::Packing, <F as Field>::Packing, Hasher, Compressor, 8, 8>;

pub fn new_batch_merkle() -> BatchMerkle {
    let perm = Perm::new_from_rng_128(&mut rand::rng());
    let hash = Hasher::new(perm.clone());
    let compress = Compressor::new(perm);
    BatchMerkle::new(hash, compress, 8)
}
