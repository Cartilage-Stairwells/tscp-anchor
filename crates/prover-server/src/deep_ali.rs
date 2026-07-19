use p3_baby_bear::BabyBear;
use p3_challenger::{CanObserve, CanSample};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use p3_symmetric::CryptographicPermutation;
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
    pub numerator: Vec<BabyBear>,
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
        Self {
            bits_consumed: 0,
            bits_remaining: lambda,
            round: 0,
        }
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
    SoundnessDepleted {
        requested: usize,
        remaining: usize,
    },
    InvalidDeepPoint {
        point: String,
        reason: String,
    },
    OracleMismatch {
        expected: String,
        actual: String,
    },
    ConstraintViolation {
        index: usize,
        deep_point: String,
    },
    FriCommitmentMismatch {
        round: usize,
    },
    InterpolationFailure {
        reason: String,
    },
    QuotientRemainderNonZero {
        expected: String,
        actual: String,
    },
    TraceEvaluationFailure {
        column: usize,
        point: String,
        reason: String,
    },
    ConstraintDegreeMismatch {
        expected: usize,
        actual: usize,
    },
    ChallengerError {
        reason: String,
    },
}

impl std::fmt::Display for DeepAliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SoundnessDepleted {
                requested,
                remaining,
            } => {
                write!(
                    f,
                    "DEEP-ALI soundness depleted: requested {} bits, {} remaining",
                    requested, remaining
                )
            }
            Self::InvalidDeepPoint { point, reason } => {
                write!(f, "Invalid DEEP point {}: {}", point, reason)
            }
            Self::OracleMismatch { expected, actual } => {
                write!(f, "Oracle mismatch: expected {}, got {}", expected, actual)
            }
            Self::ConstraintViolation { index, deep_point } => {
                write!(
                    f,
                    "Constraint {} violated at DEEP point {}",
                    index, deep_point
                )
            }
            Self::FriCommitmentMismatch { round } => {
                write!(f, "FRI commitment mismatch at round {}", round)
            }
            Self::InterpolationFailure { reason } => {
                write!(f, "Interpolation failure: {}", reason)
            }
            Self::QuotientRemainderNonZero { expected, actual } => {
                write!(
                    f,
                    "Quotient remainder non-zero: expected {}, got {}",
                    expected, actual
                )
            }
            Self::TraceEvaluationFailure {
                column,
                point,
                reason,
            } => {
                write!(
                    f,
                    "Trace evaluation failed for column {} at point {}: {}",
                    column, point, reason
                )
            }
            Self::ConstraintDegreeMismatch { expected, actual } => {
                write!(
                    f,
                    "Constraint degree mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            Self::ChallengerError { reason } => {
                write!(f, "Challenger error: {}", reason)
            }
        }
    }
}

impl std::error::Error for DeepAliError {}

fn eval_poly_horner(coeffs: &[BabyBear], x: BabyBear) -> BabyBear {
    let mut acc = BabyBear::ZERO;
    for c in coeffs.iter().rev() {
        acc = acc * x + *c;
    }
    acc
}

pub struct TraceEvaluator {
    column_polynomials: Vec<Vec<BabyBear>>,
}

impl TraceEvaluator {
    pub fn from_trace(trace: &RowMajorMatrix<BabyBear>) -> Result<Self, DeepAliError> {
        let height = trace.height();
        let num_cols = trace.width();
        let mut column_polynomials = Vec::with_capacity(num_cols);
        for col in 0..num_cols {
            let evaluations: Vec<BabyBear> =
                (0..height).map(|r| trace.get(r, col).unwrap()).collect();
            let coeffs = interpolate_lagrange_naive(&evaluations)?;
            column_polynomials.push(coeffs);
        }

        Ok(Self { column_polynomials })
    }

    pub fn evaluate_column(&self, col_idx: usize, z: BabyBear) -> Result<BabyBear, DeepAliError> {
        if col_idx >= self.column_polynomials.len() {
            return Err(DeepAliError::TraceEvaluationFailure {
                column: col_idx,
                point: format!("{:?}", z),
                reason: format!(
                    "column index out of bounds (max {})",
                    self.column_polynomials.len()
                ),
            });
        }
        Ok(eval_poly_horner(&self.column_polynomials[col_idx], z))
    }

    pub fn evaluate_columns(
        &self,
        col_indices: &[usize],
        z: BabyBear,
    ) -> Result<Vec<BabyBear>, DeepAliError> {
        col_indices
            .iter()
            .map(|&col| self.evaluate_column(col, z))
            .collect()
    }

    pub fn evaluate_shifted(
        &self,
        col_idx: usize,
        z: BabyBear,
        shift: usize,
    ) -> Result<BabyBear, DeepAliError> {
        let shifted_z = z + BabyBear::new(shift as u32);
        self.evaluate_column(col_idx, shifted_z)
    }
}

pub trait DeepAliVerifier<F: Field + PrimeCharacteristicRing, Comm, C: CanObserve<F> + CanSample<F>>
{
    fn verify_deep_batch(
        &self,
        trace: &p3_matrix::dense::RowMajorMatrix<F>,
        trace_commitment: &Comm,
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
    ) -> Result<Vec<BabyBear>, DeepAliError>;
}

pub type DeepAliChallenger<P> = p3_challenger::DuplexChallenger<BabyBear, P, 24, 7>;

pub struct BabyBearDeepAli<MMCS, P>
where
    P: CryptographicPermutation<[BabyBear; 24]>,
{
    _phantom_mmcs: PhantomData<MMCS>,
    _phantom_perm: PhantomData<P>,
    pub max_degree: usize,
    pub num_columns: usize,
}

impl<MMCS, P> BabyBearDeepAli<MMCS, P>
where
    P: CryptographicPermutation<[BabyBear; 24]>,
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

        let evaluations: Vec<BabyBear> = (0..height)
            .map(|r| trace.get(r, col_idx).unwrap())
            .collect();
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
            local_evaluations
                .iter()
                .copied()
                .fold(BabyBear::ZERO, |a, b| a + b)
        } else {
            eval_poly_horner(&link.numerator, z)
        };

        if link.denominator == BabyBear::ZERO {
            return Err(DeepAliError::InvalidDeepPoint {
                point: format!("{:?}", z),
                reason: "DEEP point lies in trace domain (vanishing polynomial is zero)"
                    .to_string(),
            });
        }

        let scaled_constraint = constraint_value * link.denominator.inverse();
        Ok(scaled_constraint)
    }
}
impl<MMCS, P> DeepAliVerifier<BabyBear, (), DeepAliChallenger<P>> for BabyBearDeepAli<MMCS, P>
where
    MMCS: p3_commit::Mmcs<BabyBear>,
    P: CryptographicPermutation<[BabyBear; 24]>,
{
    fn verify_deep_batch(
        &self,
        trace: &RowMajorMatrix<BabyBear>,
        _trace_commitment: &(),
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
                    reason: format!(
                        "column index {} out of bounds (max {})",
                        resp.query.column_index, self.num_columns
                    ),
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
                reason: "Observation Window Safety Loop flagged CRITICAL — verification aborted"
                    .to_string(),
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
                challenger.observe(trace.get(row, col).unwrap());
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

        for quotient in quotients.iter() {
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

fn interpolate_lagrange_naive(evaluations: &[BabyBear]) -> Result<Vec<BabyBear>, DeepAliError> {
    let n = evaluations.len();

    if n == 0 {
        return Err(DeepAliError::InterpolationFailure {
            reason: "empty evaluation vector".to_string(),
        });
    }

    let mut coeffs = vec![BabyBear::ZERO; n];

    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        let mut basis = vec![BabyBear::ONE];
        let mut denom = BabyBear::ONE;

        for j in 0..n {
            if i != j {
                let xj = BabyBear::new(j as u32);
                let xi = BabyBear::new(i as u32);

                let mut next = vec![BabyBear::ZERO; basis.len() + 1];

                for k in 0..basis.len() {
                    next[k] -= basis[k] * xj;
                    next[k + 1] += basis[k];
                }

                basis = next;
                denom *= xi - xj;
            }
        }

        let scale = denom.inverse();

        for k in 0..basis.len() {
            coeffs[k] += basis[k] * evaluations[i] * scale;
        }
    }

    Ok(coeffs)
}
