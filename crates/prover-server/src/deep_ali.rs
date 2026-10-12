use p3_baby_bear::BabyBear;
use p3_challenger::{CanObserve, CanSample};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use p3_symmetric::CryptographicPermutation;
use std::marker::PhantomData;

use crate::deep_ali_fri::primitive_root;
use oracle_layer::fft::Radix2Interpolator;

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

/// Reserved verifier scaffold (issue #52): part of the dormant DEEP-ALI
/// verification path. Zero constructors anywhere in the repo, including
/// tests. Its shift semantics (additive vs multiplicative) remain an
/// open design decision (paper Findings 13/23/32). Not active.
#[derive(Clone, Debug)]
pub struct AlgebraicLink<F: Field> {
    pub constraint_index: usize,
    pub numerator: Vec<BabyBear>,
    pub denominator: F,
    pub deep_point: F,
    pub involved_columns: Vec<usize>,
    pub shifts: Vec<usize>,
}

/// Reserved verifier scaffold (issue #52 ruling, 2026-10-11): retained
/// as intended, INCOMPLETE verifier infrastructure — not a working
/// security boundary. Status: zero production constructors (all ::new
/// call sites are #[cfg(test)]); the F35 `consume_and_bind` correction
/// is implemented but LATENT, because its only live-code caller
/// (`verify_deep_batch`) is itself unreachable. Activation is deferred
/// until the end-to-end DEEP-ALI verifier's cryptographic contract is
/// defined. Do not wire into the serving path.
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

/// Reserved verifier scaffold (issue #52): dormant shift machinery of
/// the unreachable DEEP-ALI verification path — its only production
/// callers are `evaluate_air_constraint`, which is only called by
/// `verify_deep_batch`, which has no callers at all. Evaluates with an
/// ADDITIVE shift convention (z + k) over the integer domain; whether
/// that convention is the intended one is unresolved (Findings
/// 13/23/32). Issue #38 moved the live interpolation path to the
/// two-adic subgroup, but interpolation-domain selection and shift
/// semantics are separate decisions — that change neither activates
/// this machinery nor approves its convention. Not active.
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
    /// Reserved verifier scaffold (issue #52 ruling, 2026-10-11): the
    /// intended DEEP-ALI batch verifier, INCOMPLETE and INACTIVE. As
    /// implemented it: has no callers anywhere, including tests; takes
    /// the trace as prover-supplied trusted input; uses `()` for the
    /// trace commitment (no commitment binding); performs no quotient
    /// verification (paper Finding 17); and relies on unresolved shift
    /// semantics (Findings 13/23/32). The F35 accumulator binding
    /// inside it is implemented but latent for the same reason. Wiring
    /// this into the serving path would give unbound trace data the
    /// appearance of cryptographic verification. Activation is
    /// deferred until the cryptographic contract is defined.
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
        // Issue #38: the column polynomial is interpolated over the
        // two-adic subgroup {omega^i} via inverse FFT (O(h log h)),
        // replacing the integer-domain {0..h-1} naive Lagrange basis
        // (O(h^2)). The DEEP quotient identity below is
        // domain-independent: only the definition of the column
        // polynomial changes, not the quotient construction.
        let coeffs = interpolate_subgroup(&evaluations)?;
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

        // Issue #38 ruling 8: the synthetic division above satisfies
        // t(x) = (x - z) * q(x) + remainder identically in the field,
        // so evaluating at x = z gives remainder = t(z) = fz for every
        // honest input — this branch is mathematically unreachable.
        // It is retained deliberately as an implementation-error
        // tripwire: it fires only if the division, the interpolation,
        // or the Horner evaluation are misimplemented.
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
    /// Issue #52 ruling: retained as reserved, incomplete verifier
    /// infrastructure. The parameter named `_trace_commitment` is `()`:
    /// this impl binds no commitment. See the trait method's doc for the
    /// full inactive-scaffold limitations record.
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

/// Integer-domain Lagrange interpolation over the points {0..n-1}.
///
/// Issue #38: no longer used on the live DEEP-quotient path (the live
/// path interpolates over the two-adic subgroup; see
/// `interpolate_subgroup`). Retained for two reasons: (1) it remains
/// the basis of the dead-path `TraceEvaluator` (zero callers; the
/// shift machinery that consumes it is deferred to the #52
/// wire-or-remove decision, per the #38 ruling), and (2) it serves as
/// the independent O(n^2) test oracle for the FFT interpolation and
/// for documenting the convention change (quotients differ between
/// bases; identities hold under each).
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

/// Two-adic subgroup interpolation: treat the column evaluations as
/// the values of the column polynomial t at the points
/// {omega^0, omega^1, ..., omega^(h-1)}, where omega is the primitive
/// h-th root of unity (generator-31 convention, shared with
/// deep_ali_fri's FRI domain), and recover the coefficient form of t
/// via inverse FFT (Issue #38: O(h log h), replacing the O(h^2)
/// integer-domain Lagrange basis).
///
/// Height requirement (Issue #38 ruling 6): h must be a power of two
/// and at least 2 — radix-2 inverse FFT requires a power-of-two
/// length, and the FRI side pads to a minimum domain of 2. A trace
/// whose height is not a power of two is a loud error, never silent
/// padding.
fn interpolate_subgroup(evaluations: &[BabyBear]) -> Result<Vec<BabyBear>, DeepAliError> {
    let h = evaluations.len();
    if h == 0 {
        return Err(DeepAliError::InterpolationFailure {
            reason: "empty evaluation vector".to_string(),
        });
    }
    if h < 2 || !h.is_power_of_two() {
        return Err(DeepAliError::InterpolationFailure {
            reason: format!(
                "two-adic subgroup interpolation requires a power-of-two height >= 2 (radix-2 inverse FFT); got height {h}"
            ),
        });
    }
    let omega = primitive_root(h);
    Ok(Radix2Interpolator::interpolate_column(evaluations, omega))
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

#[cfg(test)]
mod deep_quotient_subgroup_tests {
    use super::*;
    use p3_baby_bear::{default_babybear_poseidon2_24, Poseidon2BabyBear};
    use p3_challenger::DuplexChallenger;
    use p3_field::PrimeField64;

    type DeepPermutation = Poseidon2BabyBear<24>;

    fn fresh_deep_challenger() -> DeepAliChallenger<DeepPermutation> {
        DuplexChallenger::new(default_babybear_poseidon2_24())
    }

    /// Deterministic LCG point generator (same convention as the
    /// deep_ali_fri trace fill): independent of the trace values.
    struct LcgPoint {
        state: u64,
    }

    impl LcgPoint {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }
        fn next(&mut self) -> BabyBear {
            self.state = (1103515245 * self.state + 12345) % BabyBear::ORDER_U64;
            BabyBear::new(self.state as u32)
        }
    }

    fn lcg_trace(rows: usize, cols: usize, seed: u64) -> RowMajorMatrix<BabyBear> {
        let mut st = LcgPoint::new(seed);
        let vals: Vec<BabyBear> = (0..rows * cols).map(|_| st.next()).collect();
        RowMajorMatrix::new(vals, cols)
    }

    /// Independent O(n^2) oracle: the value of the subgroup-interpolated
    /// polynomial at an arbitrary point x, computed directly from the
    /// Lagrange basis over {omega^i} WITHOUT going through any
    /// interpolation routine. Any agreement with the FFT path is
    /// evidence the FFT interpolation is faithful.
    #[allow(clippy::needless_range_loop)]
    fn lagrange_value_oracle(evals: &[BabyBear], omega: BabyBear, x: BabyBear) -> BabyBear {
        let n = evals.len();
        let mut acc = BabyBear::ZERO;
        for i in 0..n {
            let xi = omega.exp_u64(i as u64);
            let mut num = BabyBear::ONE;
            let mut den = BabyBear::ONE;
            for j in 0..n {
                if i != j {
                    let xj = omega.exp_u64(j as u64);
                    num *= x - xj;
                    den *= xi - xj;
                }
            }
            acc += evals[i] * num * den.inverse();
        }
        acc
    }

    /// Synthetic division of coeffs by (x - z), the exact construction
    /// used on the live path — replicated here so the old integer-domain
    /// basis can be carried through the same division for comparison.
    fn divide_by_x_minus_z(coeffs: &[BabyBear], z: BabyBear) -> (Vec<BabyBear>, BabyBear) {
        let n = coeffs.len();
        let mut quotient = vec![BabyBear::ZERO; n - 1];
        if n >= 2 {
            quotient[n - 2] = coeffs[n - 1];
        }
        for k in (0..n - 2).rev() {
            quotient[k] = coeffs[k + 1] + z * quotient[k + 1];
        }
        let remainder = coeffs[0] + z * quotient[0];
        (quotient, remainder)
    }

    /// Issue #38 gate 1: the FFT interpolation must be faithful to the
    /// subgroup polynomial it claims to produce — checked against the
    /// independent Lagrange value oracle at randomized points, not
    /// merely round-tripped.
    #[test]
    fn subgroup_fft_interpolation_matches_lagrange_value_oracle() {
        for (h, seed) in [(4usize, 0xA1), (8, 0xB2), (16, 0xC3), (32, 0xD4)] {
            let trace = lcg_trace(h, 2, seed);
            let omega = primitive_root(h);
            let mut pts = LcgPoint::new(seed + 7);
            for col in 0..2 {
                let evals: Vec<BabyBear> = (0..h).map(|r| trace.get(r, col).unwrap()).collect();
                let coeffs = interpolate_subgroup(&evals).expect("pow2 height interpolates");
                for _ in 0..8 {
                    let x = pts.next();
                    let via_fft = eval_poly_horner(&coeffs, x);
                    let via_oracle = lagrange_value_oracle(&evals, omega, x);
                    assert_eq!(
                        via_fft, via_oracle,
                        "height {h} col {col}: FFT coefficients must define the subgroup polynomial"
                    );
                }
            }
        }
    }

    /// Issue #38 gate 2 (acceptance condition): the DEEP quotient
    /// identity t(x) = (x - z) * q(x) + t(z) must hold at randomized
    /// evaluation points for the subgroup-basis quotient produced by
    /// the live path — the identity itself, not just end-to-end tests.
    #[test]
    fn deep_identity_holds_at_randomized_points() {
        for (h, seed) in [(8usize, 0x51), (16, 0x62), (32, 0x73)] {
            let trace = lcg_trace(h, 2, seed);
            let prover: BabyBearDeepAli<(), DeepPermutation> = BabyBearDeepAli::new(64, 2);
            let mut z_gen = LcgPoint::new(seed * 3 + 1);
            let mut x_gen = LcgPoint::new(seed * 5 + 11);
            for col in 0..2 {
                // a deterministic, non-domain query point
                let z = z_gen.next();
                let query = DeepQuery {
                    point: z,
                    trace_index: 0,
                    column_index: col,
                };
                let q = prover
                    .compute_single_quotient(&trace, &query)
                    .expect("pow2-height trace interpolates and divides");
                let evals: Vec<BabyBear> = (0..h).map(|r| trace.get(r, col).unwrap()).collect();
                let t_coeffs = interpolate_subgroup(&evals).unwrap();
                let fz = eval_poly_horner(&t_coeffs, z);
                assert_eq!(q.len(), h - 1, "quotient degree must be h-2");
                for _ in 0..10 {
                    let x = x_gen.next();
                    let lhs = eval_poly_horner(&q, x) * (x - z) + fz;
                    let rhs = eval_poly_horner(&t_coeffs, x);
                    assert_eq!(
                        lhs, rhs,
                        "DEEP identity must hold at randomized x (h={h}, col={col})"
                    );
                }
            }
        }
        let _ = fresh_deep_challenger(); // permutation import stays exercised
    }

    /// Issue #38 gate 3: the convention change is real — the subgroup
    /// basis and the integer basis define DIFFERENT column polynomials,
    /// so their quotients differ, while each satisfies its own DEEP
    /// identity. Predicted before implementation; the naive path is the
    /// retained oracle.
    #[test]
    fn subgroup_basis_quotient_differs_from_integer_basis() {
        let h = 16usize;
        let trace = lcg_trace(h, 1, 0x99);
        let prover: BabyBearDeepAli<(), DeepPermutation> = BabyBearDeepAli::new(64, 2);
        let z = LcgPoint::new(0x99).next();
        let query = DeepQuery {
            point: z,
            trace_index: 0,
            column_index: 0,
        };
        let q_sub = prover
            .compute_single_quotient(&trace, &query)
            .expect("subgroup path");
        let evals: Vec<BabyBear> = (0..h).map(|r| trace.get(r, 0).unwrap()).collect();
        let t_int = interpolate_lagrange_naive(&evals).expect("oracle interpolates");
        let fz_int = eval_poly_horner(&t_int, z);
        let (q_int, remainder_int) = divide_by_x_minus_z(&t_int, z);
        assert_eq!(
            remainder_int, fz_int,
            "oracle basis must satisfy its own tripwire identity"
        );
        assert_ne!(
            q_sub, q_int,
            "the two bases define different column polynomials; quotients must differ"
        );
        // each basis satisfies its own DEEP identity at a random point
        let x = LcgPoint::new(0x77).next();
        let lhs_int = eval_poly_horner(&q_int, x) * (x - z) + fz_int;
        let rhs_int = eval_poly_horner(&t_int, x);
        assert_eq!(lhs_int, rhs_int, "integer basis identity holds");
        let t_sub = interpolate_subgroup(&evals).unwrap();
        let lhs_sub = eval_poly_horner(&q_sub, x) * (x - z) + eval_poly_horner(&t_sub, z);
        let rhs_sub = eval_poly_horner(&t_sub, x);
        assert_eq!(lhs_sub, rhs_sub, "subgroup basis identity holds");
    }

    /// Issue #38 ruling 6: non-power-of-two (and sub-2) heights are a
    /// loud, explicit error — never silent padding, never a panic.
    #[test]
    fn non_power_of_two_height_is_a_loud_error() {
        let prover: BabyBearDeepAli<(), DeepPermutation> = BabyBearDeepAli::new(64, 2);
        for bad_h in [0usize, 1, 3, 12, 100] {
            let trace = lcg_trace(bad_h.max(1), 1, 0xF0);
            let query = DeepQuery {
                point: BabyBear::new(5),
                trace_index: 0,
                column_index: 0,
            };
            let r = prover.compute_single_quotient(&trace, &query);
            match r {
                Err(DeepAliError::InterpolationFailure { reason }) => {
                    assert!(
                        reason.contains("power-of-two"),
                        "height {bad_h}: reason must name the power-of-two requirement, got: {reason}"
                    );
                }
                Err(e) => panic!("height {bad_h}: unexpected error variant {e:?}"),
                Ok(v) => panic!(
                    "height {bad_h}: non-power-of-two height must not produce a quotient (got len {})",
                    v.len()
                ),
            }
        }
    }
}
