use crate::constraint::Constraint;
use crate::oracle::MleOracle;
use p3_field::Field;

/// An oracle that evaluates a constraint over a trace of rows.
/// The trace is a hypercube: each row is a vector of field elements.
/// The oracle maps a binary index (as a field vector) to the constraint evaluation at that row.
pub struct ConstraintOracle<F: Field, C: Constraint<F>> {
    trace: Vec<Vec<F>>, // rows x columns
    constraint: C,
}

impl<F: Field, C: Constraint<F>> ConstraintOracle<F, C> {
    pub fn new(trace: Vec<Vec<F>>, constraint: C) -> Self {
        Self { trace, constraint }
    }
}

impl<F: Field, C: Constraint<F>> MleOracle<F> for ConstraintOracle<F, C> {
    fn n_vars(&self) -> usize {
        let rows = self.trace.len();
        if rows == 0 {
            return 0;
        }
        // number of bits needed to address rows (power of two)
        (rows as f64).log2().ceil() as usize
    }

    fn eval(&self, point: &[F]) -> F {
        // interpret point as binary index
        let mut index = 0usize;
        for (i, &bit) in point.iter().enumerate() {
            if bit == F::ONE {
                index |= 1 << i;
            }
        }
        // assume trace length is a power of two
        let row = &self.trace[index];
        self.constraint.evaluate(row)
    }
}
