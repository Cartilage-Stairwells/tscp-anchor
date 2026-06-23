use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix;
use p3_commit::PolynomialSpace;
use p3_challenger::{CanObserve, CanSample};
use p3_symmetric::Permutation;
use std::marker::PhantomData;

pub const DEEP_ALI_LAMBDA: usize = 128;
pub const MAX_DEEP_POINTS: usize = 16;
pub const FRI_FOLDING_FACTOR: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeepQuery<F: Field> {
    pub point: F,
    pub trace_index: usize,
    pub column_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeepResponse<F: Field> {
    pub value: F,
    pub query: DeepQuery<F>,
}

#[derive(Clone, Debug)]
pub struct AlgebraicLink<F: Field> {
    pub constraint_index: usize,
    pub numerator: Vec<F>,
    pub denominator: F,
    pub deep_point: F,
    pub involved_columns: Vec<usize>,
    pub shifts: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct SoundnessAccumulator {
    pub bits_consumed: usize,
    pub bits_remaining: usize,
    pub round: usize,
}

impl SoundnessAccumulator {
    pub fn new(lambda: usize) -> Self {
        Self { bits_consumed: 0, bits_remaining: lambda, round: 0 }
    }

    pub fn consume(&mut self, bits: usize) -> Result<(), DeepAliError> {
        if bits > self.bits_remaining {
            return Err(DeepAliError::SoundnessDepleted {
                requested: bits,
                remaining: self.bits_remaining,
            });
        }
        self.bits_consumed += bits;
        self.bits_remaining -= bits;
        self.round += 1;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeepAliError {
    SoundnessDepleted { requested: usize, remaining: usize },
    InvalidDeepPoint { point: String, reason: String },
    OracleMismatch { expected: String, actual: String },
    ConstraintViolation { index: usize, deep_point: String },
    FriCommitmentMismatch { round: usize },
    InterpolationFailure { reason: String },
    QuotientRemainderNonZero { expected: String, actual: String },
    TraceEvaluationFailure { column: usize, point: String, reason: String },
    ConstraintDegreeMismatch { expected: usize, actual: usize },
    ChallengerError { reason: String },
}

impl std::fmt::Display for DeepAliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SoundnessDepleted { requested, remaining } => {
                write!(f, "DEEP-ALI soundness depleted: requested {} bits, {} remaining", requested, remaining)
            }
            Self::InvalidDeepPoint { point, reason } => {
                write!(f, "Invalid DEEP point {}: {}", point, reason)
            }
            Self::OracleMismatch { expected, actual } => {
                write!(f, "Oracle mismatch: expected {}, got {}", expected, actual)
            }
            Self::ConstraintViolation { index, deep_point } => {
                write!(f, "Constraint {} violated at DEEP point {}", index, deep_point)
            }
            Self::FriCommitmentMismatch { round } => {
                write!(f, "FRI commitment mismatch at round {}", round)
            }
            Self::InterpolationFailure { reason } => {
                write!(f, "Interpolation failure: {}", reason)
            }
            Self::QuotientRemainderNonZero { expected, actual } => {
                write!(f, "Quotient remainder non-zero: expected {}, got {}", expected, actual)
            }
            Self::TraceEvaluationFailure { column, point, reason } => {
                write!(f, "Trace evaluation failed for column {} at point {}: {}", column, point, reason)
            }
            Self::ConstraintDegreeMismatch { expected, actual } => {
                write!(f, "Constraint degree mismatch: expected {}, got {}", expected, actual)
            }
            Self::ChallengerError { reason } => {
                write!(f, "Challenger error: {}", reason)
            }
        }
    }
}

impl std::error::Error for DeepAliError {}

pub struct TraceEvaluator<F: Field> {
    pub column_polynomials: Vec<Vec<F>>,
    pub trace_height: usize,
}

impl<F: Field> TraceEvaluator<F> {
    pub fn from_trace(trace: &RowMajorMatrix<F>) -> Result<Self, DeepAliError> {
        let height = trace.height();
        let num_cols = trace.width();
        let mut column_polynomials = Vec::with_capacity(num_cols);

        for col in 0..num_cols {
            let evaluations: Vec<F> = (0..height).map(|r| trace.get(r, col)).collect();
            let coeffs = interpolate_lagrange_naive(&evaluations)?;
            column_polynomials.push(coeffs);
        }

        Ok(Self { column_polynomials, trace_height: height })
    }

    pub fn evaluate_column(&self, col_idx: usize, z: F) -> Result<F, DeepAliError> {
        if col_idx >= self.column_polynomials.len() {
            return Err(DeepAliError::TraceEvaluationFailure {
                column: col_idx,
                point: format!("{:?}", z),
                reason: format!("column index out of bounds (max {})", self.column_polynomials.len()),
            });
        }
        Ok(eval_poly_horner(&self.column_polynomials[col_idx], z))
    }

    pub fn evaluate_columns(&self, col_indices: &[usize], z: F) -> Result<Vec<F>, DeepAliError> {
        col_indices.iter().map(|&col| self.evaluate_column(col, z)).collect()
    }

    pub fn evaluate_shifted(&self, col_idx: usize, z: F, shift: usize) -> Result<F, DeepAliError> {
        let shifted_z = z + F::from_canonical_usize(shift);
        self.evaluate_column(col_idx, shifted_z)
    }
}

pub trait DeepAliVerifier<F: Field, S: PolynomialSpace, C: CanObserve<F> + CanSample<F>> {
    fn verify_deep_batch(
        &self,
        trace: &p3_matrix::dense::RowMajorMatrix<F>,
        trace_commitment: &S::Commitment,
        queries: &[DeepResponse<F>],
        links: &[AlgebraicLink<F>],
        accumulator: &mut SoundnessAccumulator,
        challenger: &mut C,
    ) -> Result<(), DeepAliError>;

    fn compute_deep_quotient(
        &self,
        trace: &RowMajorMatrix<F>,
        queries: &[DeepQuery<F>],
        challenger: &mut C,
    ) -> Result<Vec<F>, DeepAliError>;
}

use p3_baby_bear::BabyBear;
use p3_merkle_tree::MerkleTreeMmcs;

pub type DeepAliChallenger<P> = p3_challenger::DuplexChallenger<BabyBear, P, 24, 7>;

pub struct BabyBearDeepAli<MMCS, P>
where
    P: Permutation<[BabyBear; 24]>,
{
    _phantom_mmcs: PhantomData<MMCS>,
    _phantom_perm: PhantomData<P>,
    pub max_degree: usize,
    pub num_columns: usize,
}

impl<MMCS, P> BabyBearDeepAli<MMCS, P>
where
    P: Permutation<[BabyBear; 24]>,
{
    pub fn new(max_degree: usize, num_columns: usize) -> Self {
        Self {
            _phantom_mmcs: PhantomData,
            _phantom_perm: PhantomData,
            max_degree,
            num_columns,
        }
    }

    fn compute_single_quotient(
        &self,
        trace: &RowMajorMatrix<BabyBear>,
        query: &DeepQuery<BabyBear>,
    ) -> Result<Vec<BabyBear>, DeepAliError> {
        let height = trace.height();
        let col_idx = query.column_index;
        let z = query.point;

        let evaluations: Vec<BabyBear> = (0..height).map(|r| trace.get(r, col_idx)).collect();
        let coeffs = interpolate_lagrange_naive(&evaluations)?;
        let fz = eval_poly_horner(&coeffs, z);

        let n = coeffs.len();
        if n == 0 {
            return Err(DeepAliError::InterpolationFailure {
                reason: "empty polynomial".to_string(),
            });
        }

        let mut quotient = vec![BabyBear::ZERO; n - 1];
        if n >= 2 {
            quotient[n - 2] = coeffs[n - 1];
        }

        for k in (0..n - 2).rev() {
            quotient[k] = coeffs[k + 1] + z * quotient[k + 1];
        }

        let remainder = coeffs[0] + z * quotient[0];
        if remainder != fz {
            return Err(DeepAliError::QuotientRemainderNonZero {
                expected: format!("{:?}", fz),
                actual: format!("{:?}", remainder),
            });
        }

        Ok(quotient)
    }

    fn evaluate_air_constraint(
        &self,
        trace: &RowMajorMatrix<BabyBear>,
        link: &AlgebraicLink<BabyBear>,
    ) -> Result<BabyBear, DeepAliError> {
        let evaluator = TraceEvaluator::from_trace(trace)?;
        let z = link.deep_point;

        let mut local_evaluations: Vec<BabyBear> = Vec::new();

        for (col_idx, shift) in link.involved_columns.iter().zip(link.shifts.iter()) {
            if *col_idx >= self.num_columns {
                return Err(DeepAliError::InvalidDeepPoint {
                    point: format!("{:?}", z),
                    reason: format!("constraint references column {} out of bounds", col_idx),
                });
            }

            let eval = if *shift == 0 {
                evaluator.evaluate_column(*col_idx, z)?
            } else {
                evaluator.evaluate_shifted(*col_idx, z, *shift)?
            };
            local_evaluations.push(eval);
        }

        let constraint_value = if link.numerator.is_empty() {
            local_evaluations.iter().copied().fold(BabyBear::ZERO, |a, b| a + b)
        } else {
            eval_poly_horner(&link.numerator, z)
        };

        if link.denominator == BabyBear::ZERO {
            return Err(DeepAliError::InvalidDeepPoint {
                point: format!("{:?}", z),
                reason: "DEEP point lies in trace domain (vanishing polynomial is zero)".to_string(),
            });
        }

        let scaled_constraint = constraint_value * link.denominator.inverse();
        Ok(scaled_constraint)
    }
}

impl<MMCS, P> DeepAliVerifier<BabyBear, MerkleTreeMmcs<MMCS>, DeepAliChallenger<P>>
    for BabyBearDeepAli<MMCS, P>
where
    MMCS: p3_commit::Mmcs<BabyBear>,
    P: Permutation<[BabyBear; 24]>,
{
    fn verify_deep_batch(
        &self,
        trace: &RowMajorMatrix<BabyBear>,
        _trace_commitment: &<MerkleTreeMmcs<MMCS> as p3_commit::PolynomialSpace>::Commitment,
        queries: &[DeepResponse<BabyBear>],
        links: &[AlgebraicLink<BabyBear>],
        accumulator: &mut SoundnessAccumulator,
        challenger: &mut DeepAliChallenger<P>,
    ) -> Result<(), DeepAliError> {
        let bits_per_query = DEEP_ALI_LAMBDA / queries.len().max(1);
        accumulator.consume(bits_per_query * queries.len())?;

        for resp in queries {
            if resp.query.column_index >= self.num_columns {
                return Err(DeepAliError::InvalidDeepPoint {
                    point: format!("{:?}", resp.query.point),
                    reason: format!("column index {} out of bounds (max {})", resp.query.column_index, self.num_columns),
                });
            }
            challenger.observe(resp.value);
        }

        for link in links {
            // Use the real trace — not a zeroed dummy — so constraint
            // evaluation is actually meaningful.
            let constraint_eval = self.evaluate_air_constraint(trace, link)?;

            if constraint_eval != BabyBear::ZERO {
                return Err(DeepAliError::ConstraintViolation {
                    index: link.constraint_index,
                    deep_point: format!("{:?}", link.deep_point),
                });
            }
        }

        if !crate::owsl_bridge::owsl_permits_verification() {
            return Err(DeepAliError::InvalidDeepPoint {
                point: "OWSL_HALT".to_string(),
                reason: "Observation Window Safety Loop flagged CRITICAL — verification aborted".to_string(),
            });
        }

        Ok(())
    }

    fn compute_deep_quotient(
        &self,
        trace: &RowMajorMatrix<BabyBear>,
        queries: &[DeepQuery<BabyBear>],
        challenger: &mut DeepAliChallenger<P>,
    ) -> Result<Vec<BabyBear>, DeepAliError> {
        if !crate::owsl_bridge::owsl_permits_verification() {
            return Err(DeepAliError::InvalidDeepPoint {
                point: "OWSL_HALT".to_string(),
                reason: "OWSL CRITICAL — cannot compute quotient over corrupted trace".to_string(),
            });
        }

        if queries.is_empty() {
            return Ok(vec![]);
        }

        for row in 0..trace.height() {
            for col in 0..trace.width() {
                challenger.observe(trace.get(row, col));
            }
        }

        let mut quotients: Vec<Vec<BabyBear>> = Vec::with_capacity(queries.len());
        let mut max_degree = 0;

        for query in queries {
            if query.column_index >= self.num_columns {
                return Err(DeepAliError::InvalidDeepPoint {
                    point: format!("{:?}", query.point),
                    reason: format!("column index {} out of bounds", query.column_index),
                });
            }

            let q = self.compute_single_quotient(trace, query)?;
            max_degree = max_degree.max(q.len());
            quotients.push(q);
        }

        let mut combined = vec![BabyBear::ZERO; max_degree];

        for (_i, quotient) in quotients.iter().enumerate() {
            let weight: BabyBear = challenger.sample();
            for (j, &coeff) in quotient.iter().enumerate() {
                combined[j] += weight * coeff;
            }
        }

        while combined.last() == Some(&BabyBear::ZERO) {
            combined.pop();
        }

        let actual_degree = combined.len().saturating_sub(1);
        if actual_degree >= self.max_degree {
            return Err(DeepAliError::InvalidDeepPoint {
                point: "COMBINED_QUOTIENT".to_string(),
                reason: format!("degree {} exceeds bound {}", actual_degree, self.max_degree),
            });
        }

        Ok(combined)
    }
}

fn interpolate_lagrange_naive<F: Field>(evaluations: &[F]) -> Result<Vec<F>, DeepAliError> {
    let n = evaluations.len();
    if n == 0 {
        return Err(DeepAliError::InterpolationFailure {
            reason: "empty evaluation vector".to_string(),
        });
    }

    let mut coeffs = vec![F::ZERO; n];

    for i in 0..n {
        let yi = evaluations[i];
        if yi == F::ZERO {
            continue;
        }

        let mut basis = vec![F::ZERO; n];
        basis[0] = F::ONE;

        let mut denom = F::ONE;
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut new_basis = vec![F::ZERO; n];
            for k in 0..n {
                if k < n {
                    new_basis[k] += basis[k] * (-F::from_canonical_usize(j));
                }
                if k > 0 {
                    new_basis[k] += basis[k - 1];
                }
            }
            basis = new_basis;

            let diff = F::from_canonical_usize(i) - F::from_canonical_usize(j);
            if diff == F::ZERO {
                return Err(DeepAliError::InterpolationFailure {
                    reason: format!("duplicate evaluation point at index {}", i),
                });
            }
            denom *= diff;
        }

        let scale = yi * denom.inverse();
        for k in 0..n {
            coeffs[k] += scale * basis[k];
        }
    }

    Ok(coeffs)
}

fn eval_poly_horner<F: Field>(coeffs: &[F], x: F) -> F {
    let mut result = F::ZERO;
    for i in (0..coeffs.len()).rev() {
        result = result * x + coeffs[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soundness_accumulator_basic() {
        let mut acc = SoundnessAccumulator::new(128);
        assert_eq!(acc.bits_remaining, 128);
        acc.consume(32).unwrap();
        assert_eq!(acc.bits_consumed, 32);
        assert_eq!(acc.bits_remaining, 96);
        assert_eq!(acc.round, 1);
    }

    #[test]
    fn test_interpolate_and_evaluate_identity() {
        let evals = vec![
            BabyBear::from_canonical_u32(1),
            BabyBear::from_canonical_u32(6),
            BabyBear::from_canonical_u32(17),
        ];
        let coeffs = interpolate_lagrange_naive(&evals).unwrap();
        assert_eq!(coeffs[0], BabyBear::from_canonical_u32(1));
        assert_eq!(coeffs[1], BabyBear::from_canonical_u32(2));
        assert_eq!(coeffs[2], BabyBear::from_canonical_u32(3));

        let z = BabyBear::from_canonical_u32(4);
        let fz = eval_poly_horner(&coeffs, z);
        assert_eq!(fz, BabyBear::from_canonical_u32(57));
    }

    #[test]
    fn test_trace_evaluator_basic() {
        let trace = RowMajorMatrix::new(
            vec![
                BabyBear::from_canonical_u32(1), BabyBear::from_canonical_u32(4),
                BabyBear::from_canonical_u32(2), BabyBear::from_canonical_u32(9),
                BabyBear::from_canonical_u32(3), BabyBear::from_canonical_u32(16),
            ],
            2,
        );

        let evaluator = TraceEvaluator::from_trace(&trace).unwrap();
        let z = BabyBear::from_canonical_u32(4);
        let col0_eval = evaluator.evaluate_column(0, z).unwrap();
        assert_eq!(col0_eval, BabyBear::from_canonical_u32(5));
    }
}
