use p3_baby_bear::BabyBear;
use p3_challenger::DuplexChallenger;
use p3_matrix::dense::RowMajorMatrix;
use p3_poseidon2::Poseidon2;
use p3_symmetric::Permutation;

use crate::deep_ali::{BabyBearDeepAli, DeepQuery, DeepAliChallenger, DeepAliError};
use crate::delta_fri_bridge::{DeltaFriBridge, FriProof};

pub const TEST_TRACE_HEIGHT: usize = 16;
pub const TEST_NUM_COLUMNS: usize = 2;
pub const TEST_NUM_DEEP_QUERIES: usize = 3;

pub struct FoxtrotHarness<P>
where
    P: Permutation<[BabyBear; 24]>,
{
    pub deep_ali: BabyBearDeepAli<(), P>,
    pub delta_bridge: DeltaFriBridge<P>,
    pub perm: P,
}

impl<P> FoxtrotHarness<P>
where
    P: Permutation<[BabyBear; 24]>,
{
    pub fn new(perm: P) -> Self {
        let deep_ali = BabyBearDeepAli::new(TEST_TRACE_HEIGHT - 1, TEST_NUM_COLUMNS);
        let challenger = DuplexChallenger::new(perm.clone());
        let delta_bridge = DeltaFriBridge::new(
            BabyBearDeepAli::new(TEST_TRACE_HEIGHT - 1, TEST_NUM_COLUMNS),
            challenger,
        );
        Self { deep_ali, delta_bridge, perm }
    }

    pub fn run_full_pipeline(&mut self) -> Result<FriProof, DeepAliError> {
        let trace = self.generate_synthetic_trace();
        let queries = self.generate_deep_queries(&trace);
        let mut challenger = DuplexChallenger::new(self.perm.clone());
        let quotient = self.deep_ali.compute_deep_quotient(&trace, &queries, &mut challenger)?;
        let proof = self.delta_bridge.prove_quotient(&trace, &queries)?;
        let trace_commitment = self.compute_trace_commitment(&trace);
        let verified = self.delta_bridge.verify_quotient(&trace_commitment, &proof)?;
        assert!(verified, "Foxtrot: proof verification failed");
        self.assert_pipeline_invariants(&trace, &quotient, &proof)?;
        Ok(proof)
    }

    fn generate_synthetic_trace(&self) -> RowMajorMatrix<BabyBear> {
        let mut values = Vec::with_capacity(TEST_TRACE_HEIGHT * TEST_NUM_COLUMNS);
        for i in 0..TEST_TRACE_HEIGHT {
            values.push(BabyBear::from_canonical_usize(i));
            values.push(BabyBear::from_canonical_usize(i * i));
        }
        RowMajorMatrix::new(values, TEST_NUM_COLUMNS)
    }

    fn generate_deep_queries(&self, trace: &RowMajorMatrix<BabyBear>) -> Vec<DeepQuery<BabyBear>> {
        let mut challenger = DuplexChallenger::new(self.perm.clone());
        let mut queries = Vec::with_capacity(TEST_NUM_DEEP_QUERIES);
        for row in 0..trace.height() {
            for col in 0..trace.width() {
                challenger.observe(trace.get(row, col));
            }
        }
        for i in 0..TEST_NUM_DEEP_QUERIES {
            let z: BabyBear = challenger.sample();
            queries.push(DeepQuery { point: z, trace_index: i, column_index: i % TEST_NUM_COLUMNS });
        }
        queries
    }

    fn compute_trace_commitment(&self, trace: &RowMajorMatrix<BabyBear>) -> Vec<u8> {
        vec![(trace.height() as u8), (trace.width() as u8), 0xDE, 0xAD, 0xBE, 0xEF]
    }

    fn assert_pipeline_invariants(&self, trace: &RowMajorMatrix<BabyBear>, quotient: &[BabyBear], proof: &FriProof) -> Result<(), DeepAliError> {
        let quotient_degree = quotient.len().saturating_sub(1);
        assert!(quotient_degree < TEST_TRACE_HEIGHT - 1, "Foxtrot: quotient degree {} exceeds bound {}", quotient_degree, TEST_TRACE_HEIGHT - 1);
        assert_eq!(trace.height(), TEST_TRACE_HEIGHT);
        assert_eq!(trace.width(), TEST_NUM_COLUMNS);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_perm() -> Poseidon2<BabyBear, p3_poseidon2::Poseidon2ExternalMatrixGeneral, 24, 7> {
        let rounds_f = 8;
        let rounds_p = 22;
        let external_constants = vec![[BabyBear::ZERO; 24]; rounds_f];
        let internal_constants = vec![BabyBear::ZERO; rounds_p];
        Poseidon2::new(rounds_f, external_constants.try_into().unwrap(), p3_poseidon2::Poseidon2ExternalMatrixGeneral, rounds_p, internal_constants.try_into().unwrap())
    }

    #[test]
    fn test_foxtrot_harness_creation() {
        let perm = build_test_perm();
        let _harness = FoxtrotHarness::new(perm);
    }

    #[test]
    fn test_synthetic_trace_generation() {
        let perm = build_test_perm();
        let harness = FoxtrotHarness::new(perm);
        let trace = harness.generate_synthetic_trace();
        assert_eq!(trace.height(), TEST_TRACE_HEIGHT);
        assert_eq!(trace.width(), TEST_NUM_COLUMNS);
    }

    #[test]
    fn test_full_pipeline_scaffold() {
        let perm = build_test_perm();
        let mut harness = FoxtrotHarness::new(perm);
        let result = harness.run_full_pipeline();
        assert!(result.is_ok(), "Full pipeline failed: {:?}", result.err());
    }
}
