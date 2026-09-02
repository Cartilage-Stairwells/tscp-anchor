use crate::constraint::Constraint;
use crate::oracle::{evaluate_mle, MleOracle};
use p3_field::Field;

/// An oracle that evaluates a constraint over a trace of rows.
/// The trace is a hypercube: each row is a vector of field elements.
/// The oracle evaluates the multilinear extension (MLE) of the
/// constraint evaluations at all binary points.
///
/// ARCHER Finding 10 & 11 fix:
/// - n_vars now uses integer log2 (trailing_zeros) instead of f64::log2().ceil()
/// - eval now uses evaluate_mle for correct MLE evaluation at arbitrary points
///   (previously used binary index lookup which only worked at corner points)
pub struct ConstraintOracle<F: Field, C: Constraint<F>> {
    /// Precomputed constraint evaluations at all binary points (rows of trace)
    constraint_evals: Vec<F>,
    n_vars: usize,
    #[allow(dead_code)]
    constraint: C,
}

impl<F: Field, C: Constraint<F>> ConstraintOracle<F, C> {
    pub fn new(trace: Vec<Vec<F>>, constraint: C) -> Self {
        let rows = trace.len();
        assert!(rows.is_power_of_two(), "trace length must be a power of two");
        assert!(rows > 0, "trace must not be empty");

        let n_vars = rows.trailing_zeros() as usize;

        // Precompute constraint evaluations at all binary points
        let constraint_evals: Vec<F> = trace
            .iter()
            .map(|row| constraint.evaluate(row))
            .collect();

        Self {
            constraint_evals,
            n_vars,
            constraint,
        }
    }
}

impl<F: Field, C: Constraint<F>> MleOracle<F> for ConstraintOracle<F, C> {
    fn n_vars(&self) -> usize {
        self.n_vars
    }

    fn eval(&self, point: &[F]) -> F {
        evaluate_mle(&self.constraint_evals, point)
    }
}
