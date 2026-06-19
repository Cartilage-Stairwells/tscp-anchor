use p3_field::Field;

/// A multivariate constraint evaluated on a trace row.
pub trait Constraint<F: Field> {
    /// Number of variables this constraint expects.
    fn arity(&self) -> usize;
    /// Evaluate the constraint on a slice of field elements (the row).
    fn evaluate(&self, vars: &[F]) -> F;
}

/// A composite constraint that sums the squares of several sub-constraints.
pub struct CompositeConstraint<F: Field> {
    pub constraints: Vec<Box<dyn Constraint<F>>>,
}

impl<F: Field> Constraint<F> for CompositeConstraint<F> {
    fn arity(&self) -> usize {
        // assume all constraints have same arity
        self.constraints.first().map(|c| c.arity()).unwrap_or(0)
    }

    fn evaluate(&self, vars: &[F]) -> F {
        let mut sum = F::ZERO;
        for c in &self.constraints {
            let val = c.evaluate(vars);
            sum = sum + val * val; // degree 2 each -> squared -> degree 4
        }
        sum
    }
}

/// AIR morphism: maps symbolic predicates to field constraints with shift.
pub struct AIRMorphism<F: Field> {
    pub name: String,
    pub constraint: Box<dyn Constraint<F>>,
    pub shift: F, // multiplicative generator for subgroup
}

pub mod lww;
