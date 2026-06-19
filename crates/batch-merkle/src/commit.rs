use p3_baby_bear::{BabyBear, Poseidon2BabyBear, default_babybear_poseidon2_16};
use p3_field::Field;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};

pub type F = BabyBear;
pub type Perm = Poseidon2BabyBear<16>;
pub type Hasher = PaddingFreeSponge<Perm, 16, 8, 8>;
pub type Compressor = TruncatedPermutation<Perm, 2, 8, 16>;

pub type BatchMerkle =
    MerkleTreeMmcs<<F as Field>::Packing, <F as Field>::Packing, Hasher, Compressor, 2, 8>;

pub fn new_batch_merkle() -> BatchMerkle {
    let perm = default_babybear_poseidon2_16();
    let hash = Hasher::new(perm.clone());
    let compress = Compressor::new(perm);
    BatchMerkle::new(hash, compress, 1)
}
