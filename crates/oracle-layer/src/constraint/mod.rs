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
            sum += val * val; // degree 2 each -> squared -> degree 4
        }
        sum
    }
}

/// AIR morphism: maps symbolic predicates to field constraints with shift.
pub struct AIRMorphism<F: Field> {
    pub name: String,
    pub constraint: Box<dyn Constraint<F>>,
    /// Issue #36 record: declared but unconsumed — no oracle-layer code
    /// applies it. The DEEP-ALI evaluator's live shift convention is
    /// additive over the integer-domain interpolation {0..n-1}
    /// (shifted point z + k); this generator becomes the shift factor
    /// only when the domain moves to the two-adic subgroup (#38, FFT
    /// interpolation). Until then it is reserved, not authoritative.
    pub shift: F, // reserved: two-adic subgroup shift generator (#36, #38)
}

pub mod lww;
