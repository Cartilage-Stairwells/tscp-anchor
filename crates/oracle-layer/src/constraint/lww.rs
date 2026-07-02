use super::{CompositeConstraint, Constraint};
use p3_field::Field;
use std::marker::PhantomData;

// ---------- Booleanity: S*(1-S) = 0 ----------
pub struct LwwBooleanity<F>(PhantomData<F>);
impl<F: Field> Constraint<F> for LwwBooleanity<F> {
    fn arity(&self) -> usize {
        7
    }
    fn evaluate(&self, vars: &[F]) -> F {
        let s = vars[6];
        s * (F::ONE - s)
    }
}

// ---------- Value update: S*(v_next - v_event) + (1-S)*(v_next - v_curr) = 0 ----------
pub struct LwwValue<F>(PhantomData<F>);
impl<F: Field> Constraint<F> for LwwValue<F> {
    fn arity(&self) -> usize {
        7
    }
    fn evaluate(&self, vars: &[F]) -> F {
        let v_curr = vars[0];
        let v_event = vars[2];
        let v_next = vars[4];
        let s = vars[6];
        s * (v_next - v_event) + (F::ONE - s) * (v_next - v_curr)
    }
}

// ---------- Timestamp update: S*(ts_next - ts_event) + (1-S)*(ts_next - ts_curr) = 0 ----------
pub struct LwwTimestamp<F>(PhantomData<F>);
impl<F: Field> Constraint<F> for LwwTimestamp<F> {
    fn arity(&self) -> usize {
        7
    }
    fn evaluate(&self, vars: &[F]) -> F {
        let ts_curr = vars[1];
        let ts_event = vars[3];
        let ts_next = vars[5];
        let s = vars[6];
        s * (ts_next - ts_event) + (F::ONE - s) * (ts_next - ts_curr)
    }
}

/// Factory: returns a CompositeConstraint that enforces all three LWW rules.
pub fn lww_constraints<F: Field>() -> CompositeConstraint<F> {
    CompositeConstraint {
        constraints: vec![
            Box::new(LwwBooleanity::<F>(PhantomData)),
            Box::new(LwwValue::<F>(PhantomData)),
            Box::new(LwwTimestamp::<F>(PhantomData)),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeCharacteristicRing; // needed for ZERO

    type F = BabyBear;

    #[test]
    fn lww_constraint_passes_on_valid_trace() {
        // S=1, v_curr=5, v_event=7, v_next=7, ts_curr=10, ts_event=12, ts_next=12
        let vars: Vec<F> = vec![
            F::new(5),  // v_curr
            F::new(10), // ts_curr
            F::new(7),  // v_event
            F::new(12), // ts_event
            F::new(7),  // v_next
            F::new(12), // ts_next
            F::new(1),  // S
        ];
        let composite = lww_constraints::<F>();
        let result = composite.evaluate(&vars);
        assert_eq!(result, F::ZERO, "Valid LWW row should yield zero");
    }

    #[test]
    fn lww_constraint_fails_on_invalid_value() {
        // Same but v_next != v_event (99 instead of 7) -> should fail
        let vars: Vec<F> = vec![
            F::new(5),  // v_curr
            F::new(10), // ts_curr
            F::new(7),  // v_event
            F::new(12), // ts_event
            F::new(99), // v_next -> wrong!
            F::new(12), // ts_next
            F::new(1),  // S
        ];
        let composite = lww_constraints::<F>();
        let result = composite.evaluate(&vars);
        assert_ne!(result, F::ZERO, "Invalid row should NOT yield zero");
    }
}
