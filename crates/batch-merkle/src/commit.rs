use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};

use rand::rngs::StdRng;
use rand::SeedableRng;

pub type F = BabyBear;
pub type Perm = Poseidon2BabyBear<16>;

pub type Hasher =
    PaddingFreeSponge<Perm, 16, 8, 8>;

pub type Compressor =
    TruncatedPermutation<Perm, 2, 8, 16>;

pub type BatchMerkle =
    MerkleTreeMmcs<F, F, Hasher, Compressor, 2, 8>;

pub fn new_batch_merkle() -> BatchMerkle {
    let perm =
        Perm::new_from_rng_128(
            &mut StdRng::seed_from_u64(0)
        );

    let hash =
        Hasher::new(perm.clone());

    let compress =
        Compressor::new(perm);

    BatchMerkle::new(
        hash,
        compress,
        0
    )
}
