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
    /// applies it. Issue #52 ruling (2026-10-11): REMAINS reserved and
    /// not authoritative. The live interpolation path moved to the
    /// two-adic subgroup in #38 (FFT interpolation), but
    /// interpolation-domain selection and shift semantics are SEPARATE
    /// decisions: #38's domain change does not establish that this
    /// generator should be activated, and the dormant DEEP-ALI shift
    /// machinery still carries the additive convention pending the
    /// unresolved design decision (paper Findings 13/23/32). Zero
    /// references anywhere in the repo. Activation is deferred until
    /// the algebraic shift-semantics contract is ruled.
    pub shift: F, // reserved: shift generator — activation requires the shift-semantics ruling (#36, #38, #52)
}

pub mod lww;
