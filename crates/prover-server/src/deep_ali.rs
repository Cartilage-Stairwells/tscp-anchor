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

    /// ARCHER Finding 35 (recorded; resolved by consume_and_bind below):
    /// bare consume() does not bind the budget to the Fiat-Shamir
    /// transcript — a malicious prover could reset the accumulator
    /// without affecting the challenge sequence. Callers that must bind
    /// use consume_and_bind(), which absorbs the accumulator state into
    /// the challenger after each consume.
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

    /// ARCHER Finding 35 (fix, Issue #39): consume AND bind. After a
    /// successful consume(), the accumulator state — round,
    /// bits_consumed, bits_remaining — is absorbed into the challenger
    /// as canonical field elements, so any reset or divergence of the
    /// accumulator between prover and verifier changes the challenge
    /// sequence and invalidates the proof. A failed consume binds
    /// nothing: the challenger is left byte-identical.
    ///
    /// Scope (per the #39 ruling, stated here so it cannot drift):
    /// this binding makes the reported budget part of the Fiat-Shamir
    /// statement. It does NOT make the error terms correct or the
    /// combined bound tight — green tests prove the binding, not the
    /// soundness analysis.
    pub fn consume_and_bind<C: CanObserve<BabyBear>>(
        &mut self,
        bits: usize,
        challenger: &mut C,
    ) -> Result<(), DeepAliError> {
        self.consume(bits)?;
        let round = u32::try_from(self.round).map_err(|_| DeepAliError::AccumulatorEncoding {
            reason: format!("round {} does not fit in u32", self.round),
        })?;
        let consumed =
            u32::try_from(self.bits_consumed).map_err(|_| DeepAliError::AccumulatorEncoding {
                reason: format!("bits_consumed {} does not fit in u32", self.bits_consumed),
            })?;
        let remaining =
            u32::try_from(self.bits_remaining).map_err(|_| DeepAliError::AccumulatorEncoding {
                reason: format!("bits_remaining {} does not fit in u32", self.bits_remaining),
            })?;
        challenger.observe(BabyBear::from_u32(round));
        challenger.observe(BabyBear::from_u32(consumed));
        challenger.observe(BabyBear::from_u32(remaining));
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeepAliError {
    SoundnessDepleted {
        requested: usize,
        remaining: usize,
    },
    /// Issue #39: accumulator state that cannot be canonically encoded
    /// for transcript binding — a loud error, never a silent skip.
    AccumulatorEncoding {
        reason: String,
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
            Self::AccumulatorEncoding { reason } => {
                write!(
                    f,
                    "Soundness accumulator state cannot be encoded for transcript binding: {}",
                    reason
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
        // Issue #36 (2026-10-08, caller ruling "Option A"): the shift
        // convention is INTEGER-DOMAIN — the trace is interpolated over
        // the integer points {0..n-1} (see interpolate_lagrange_naive),
        // so "k rows later" lives at z + k: the additive form is the
        // correct arithmetic for this domain, not a defect. The
        // multiplicative form (z*g^k) becomes correct only when the
        // domain moves to the two-adic subgroup {g^i} — recorded with
        // #38 (FFT interpolation); swapping it in before that pass
        // would sample a point with no row meaning.
        //
        // ARCHER Finding 32 (cast half), FIXED: the usize->field cast
        // is now checked — a shift that does not fit in u32 is a loud
        // error instead of a silent wrap.
        let shift_val = u32::try_from(shift).map_err(|_| DeepAliError::InvalidDeepPoint {
            point: format!("{:?}", z),
            reason: format!("shift {} does not fit in u32", shift),
        })?;
        let shifted_z = z + BabyBear::new(shift_val);
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
    /// Issue #47 injection point: when set, this prover's OWSL gate
    /// checks the status file at this path instead of the process-
    /// global default. Production leaves it None.
    pub owsl_status_path: Option<String>,
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
            owsl_status_path: None,
        }
    }

    /// Issue #47 injection point: gate this prover's OWSL checks on an
    /// explicit status-file path instead of the process-global default.
    /// Test suites that own their status file construct their prover
    /// with it; production leaves it unset.
    pub fn with_owsl_status_path(mut self, path: &str) -> Self {
        self.owsl_status_path = Some(path.to_string());
        self
    }

    fn owsl_gate_permits(&self) -> bool {
        match &self.owsl_status_path {
            Some(p) => crate::owsl_bridge::owsl_permits_verification_at_path(p),
            None => crate::owsl_bridge::owsl_permits_verification(),
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
        // Issue #39 / ARCHER F35: the budget consume is now transcript-
        // bound — the accumulator state enters the challenge sequence.
        let bits_per_query = DEEP_ALI_LAMBDA / queries.len().max(1);
        accumulator.consume_and_bind(bits_per_query * queries.len(), challenger)?;

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

        if !self.owsl_gate_permits() {
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
        if !self.owsl_gate_permits() {
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

#[cfg(test)]
mod shift_semantics_tests {
    use super::*;
    use p3_matrix::dense::RowMajorMatrix;

    fn single_column_trace(values: &[u32]) -> RowMajorMatrix<BabyBear> {
        let vals: Vec<BabyBear> = values.iter().map(|&v| BabyBear::from_u32(v)).collect();
        RowMajorMatrix::new(vals, 1)
    }

    /// Issue #36: under the integer-domain convention, evaluating the
    /// column polynomial at z + k returns the value at row k when z
    /// names a row and the shift steps rows — a round-trip over real
    /// trace data, not a mocked evaluation.
    #[test]
    fn evaluate_shifted_round_trips_row_steps() {
        let trace = single_column_trace(&[10, 21, 32, 43]);
        let evaluator = TraceEvaluator::from_trace(&trace).unwrap();
        // z = 0, shift 2 -> row 2.
        let v = evaluator
            .evaluate_shifted(0, BabyBear::from_u32(0), 2)
            .unwrap();
        assert_eq!(v, BabyBear::from_u32(32));
        // z = 1 (row 1), shift 2 -> row 3.
        let v = evaluator
            .evaluate_shifted(0, BabyBear::from_u32(1), 2)
            .unwrap();
        assert_eq!(v, BabyBear::from_u32(43));
        // shift 0 is evaluate_column at z.
        let v = evaluator
            .evaluate_shifted(0, BabyBear::from_u32(3), 0)
            .unwrap();
        assert_eq!(v, BabyBear::from_u32(43));
    }

    /// Issue #36: the shifted evaluation must agree exactly with an
    /// unshifted evaluation at the arithmetically shifted point.
    #[test]
    fn evaluate_shifted_agrees_with_column_evaluation() {
        let trace = single_column_trace(&[7, 11, 13, 17, 19, 23]);
        let evaluator = TraceEvaluator::from_trace(&trace).unwrap();
        for (z, k) in [(2u32, 1usize), (0, 5), (3, 2)] {
            let shifted = evaluator
                .evaluate_shifted(0, BabyBear::from_u32(z), k)
                .unwrap();
            let direct = evaluator
                .evaluate_column(0, BabyBear::from_u32(z) + BabyBear::from_u32(k as u32))
                .unwrap();
            assert_eq!(shifted, direct, "z={}, k={}", z, k);
        }
    }

    /// ARCHER Finding 32 (cast half): a shift that does not fit in u32
    /// must be a loud error, not a silent wrap.
    #[test]
    fn evaluate_shifted_rejects_shift_beyond_u32() {
        let trace = single_column_trace(&[1, 2, 3, 4]);
        let evaluator = TraceEvaluator::from_trace(&trace).unwrap();
        let oversized = u32::MAX as usize + 1;
        let err = evaluator
            .evaluate_shifted(0, BabyBear::from_u32(0), oversized)
            .unwrap_err();
        match err {
            DeepAliError::InvalidDeepPoint { reason, .. } => {
                assert!(
                    reason.contains("u32"),
                    "reason must name the u32 bound: {}",
                    reason
                );
            }
            other => panic!("expected InvalidDeepPoint, got {:?}", other),
        }
    }
}

#[cfg(test)]
mod accumulator_binding_tests {
    use super::*;
    use p3_baby_bear::{default_babybear_poseidon2_24, Poseidon2BabyBear};
    use p3_challenger::DuplexChallenger;

    type DeepPermutation = Poseidon2BabyBear<24>;

    fn fresh_deep_challenger() -> DeepAliChallenger<DeepPermutation> {
        DuplexChallenger::new(default_babybear_poseidon2_24())
    }

    /// ARCHER F35 fix, test 1: a bound consume changes the challenge
    /// sequence — a challenger that absorbed accumulator state and one
    /// that did not must sample different elements, and two
    /// challengers that absorbed identical state must agree.
    #[test]
    fn bound_consume_changes_and_determines_challenges() {
        let mut acc = SoundnessAccumulator::new(DEEP_ALI_LAMBDA);
        let mut bound = fresh_deep_challenger();
        acc.consume_and_bind(32, &mut bound).unwrap();
        let mut unbound = fresh_deep_challenger();
        let b: BabyBear = bound.sample();
        let u: BabyBear = unbound.sample();
        assert_ne!(b, u);

        let mut a1 = SoundnessAccumulator::new(DEEP_ALI_LAMBDA);
        let mut a2 = SoundnessAccumulator::new(DEEP_ALI_LAMBDA);
        let mut c1 = fresh_deep_challenger();
        let mut c2 = fresh_deep_challenger();
        a1.consume_and_bind(48, &mut c1).unwrap();
        a2.consume_and_bind(48, &mut c2).unwrap();
        let s1: BabyBear = c1.sample();
        let s2: BabyBear = c2.sample();
        assert_eq!(s1, s2);
    }

    /// ARCHER F35 fix, test 2: a failed consume binds nothing — the
    /// challenger state is unchanged after a rejected consume_and_bind.
    #[test]
    fn failed_consume_binds_nothing() {
        let mut acc = SoundnessAccumulator::new(64);
        let mut bound = fresh_deep_challenger();
        let err = acc.consume_and_bind(65, &mut bound).unwrap_err();
        match err {
            DeepAliError::SoundnessDepleted {
                requested,
                remaining,
            } => {
                assert_eq!((requested, remaining), (65, 64));
            }
            other => panic!("expected SoundnessDepleted, got {:?}", other),
        }
        let mut unbound = fresh_deep_challenger();
        let b: BabyBear = bound.sample();
        let u: BabyBear = unbound.sample();
        assert_eq!(b, u);
    }

    /// ARCHER F35 fix, test 3: the adversarial reset the finding names.
    /// A prover that silently resets its accumulator mid-stream absorbs
    /// different state, so the two sides' challenge sequences diverge.
    /// Under bare consume() the reset was invisible; bound, it is fatal
    /// to the proof.
    #[test]
    fn reset_diverges_the_challenge_sequence() {
        let mut honest_acc = SoundnessAccumulator::new(DEEP_ALI_LAMBDA);
        let mut honest = fresh_deep_challenger();
        honest_acc.consume_and_bind(32, &mut honest).unwrap();
        honest_acc.consume_and_bind(32, &mut honest).unwrap();

        let mut malicious_acc = SoundnessAccumulator::new(DEEP_ALI_LAMBDA);
        let mut malicious = fresh_deep_challenger();
        malicious_acc.consume_and_bind(32, &mut malicious).unwrap();
        malicious_acc.bits_consumed = 0;
        malicious_acc.bits_remaining = DEEP_ALI_LAMBDA;
        malicious_acc.round = 0;
        malicious_acc.consume_and_bind(32, &mut malicious).unwrap();

        let h: BabyBear = honest.sample();
        let m: BabyBear = malicious.sample();
        assert_ne!(h, m);
    }

    /// Checked encoding (#36 discipline): state that does not fit in u32
    /// is a loud AccumulatorEncoding error, and it binds nothing.
    #[test]
    fn oversized_state_is_a_loud_error() {
        let mut acc = SoundnessAccumulator::new(DEEP_ALI_LAMBDA);
        acc.bits_consumed = u32::MAX as usize + 1;
        let mut bound = fresh_deep_challenger();
        let err = acc.consume_and_bind(0, &mut bound).unwrap_err();
        match err {
            DeepAliError::AccumulatorEncoding { reason } => {
                assert!(
                    reason.contains("u32"),
                    "reason must name the u32 bound: {}",
                    reason
                );
            }
            other => panic!("expected AccumulatorEncoding, got {:?}", other),
        }
        let mut unbound = fresh_deep_challenger();
        let b: BabyBear = bound.sample();
        let u: BabyBear = unbound.sample();
        assert_eq!(b, u);
    }
}
