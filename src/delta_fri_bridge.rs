use p3_baby_bear::BabyBear;
use p3_challenger::{CanObserve, CanSample};
use p3_commit::{Pcs, PolynomialSpace};
use p3_field::Field;
use p3_fri::{FriConfig, TwoAdicFriPcs};
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_poseidon2::Poseidon2;
use p3_symmetric::Permutation;

use crate::deep_ali::{BabyBearDeepAli, DeepAliChallenger, DeepAliError, DeepQuery};

pub const FRI_BLOWUP: usize = 2;
pub const NUM_FRI_QUERIES: usize = 80;
pub const PROOF_OF_WORK_BITS: usize = 16;

#[derive(Clone, Debug)]
pub struct FriProof {
    pub quotient_commitment: Vec<u8>,
    pub foldings: Vec<Vec<BabyBear>>,
    pub query_responses: Vec<FriQueryResponse>,
    pub pow_nonce: u64,
}

#[derive(Clone, Debug)]
pub struct FriQueryResponse {
    pub point: BabyBear,
    pub evaluation: BabyBear,
    pub merkle_path: Vec<Vec<u8>>,
}

pub struct DeltaFriBridge<P>
where
    P: Permutation<[BabyBear; 24]>,
{
    pub fri_config: FriConfig,
    pub deep_ali: BabyBearDeepAli<(), P>,
    pub challenger: DeepAliChallenger<P>,
}

impl<P> DeltaFriBridge<P>
where
    P: Permutation<[BabyBear; 24]>,
{
    pub fn new(deep_ali: BabyBearDeepAli<(), P>, challenger: DeepAliChallenger<P>) -> Self {
        let fri_config = FriConfig {
            log_blowup: log2_strict_usize(FRI_BLOWUP),
            num_queries: NUM_FRI_QUERIES,
            proof_of_work_bits: PROOF_OF_WORK_BITS,
            mmcs: (),
        };

        Self {
            fri_config,
            deep_ali,
            challenger,
        }
    }

    pub fn prove_quotient(
        &mut self,
        trace: &RowMajorMatrix<BabyBear>,
        queries: &[DeepQuery<BabyBear>],
    ) -> Result<FriProof, DeepAliError> {
        let quotient = self.deep_ali.compute_deep_quotient(
            trace,
            queries,
            &mut self.challenger,
        )?;

        let proof = FriProof {
            pow_nonce: 0,
        };

        Ok(proof)
    }

    pub fn verify_quotient(
        &mut self,
        _trace_commitment: &[u8],
        _proof: &FriProof,
    ) -> Result<bool, DeepAliError> {
        Ok(true)
    }
}

fn log2_strict_usize(n: usize) -> usize {
    assert!(n.is_power_of_two(), "n must be a power of two");
    n.trailing_zeros() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log2_strict() {
        assert_eq!(log2_strict_usize(2), 1);
        assert_eq!(log2_strict_usize(4), 2);
        assert_eq!(log2_strict_usize(1024), 10);
    }

    #[test]
    #[should_panic]
    fn test_log2_strict_non_power_of_two() {
        log2_strict_usize(3);
    }
}
