use std::cell::RefCell;
use tscp_polyir_verification::{Transformation, TransformResult, Trace, WitnessDiff, TransformError};

pub struct EngineAdapter<'a, E> {
    pub engine: RefCell<&'a mut E>,
}

impl<'a, E, R> Transformation for EngineAdapter<'a, E>
where
    E: FnMut(&crate::EventEnvelope) -> anyhow::Result<R>,
{
    type InputIR = crate::EventEnvelope;
    type OutputIR = crate::EventEnvelope;

    fn apply(&self, input: Self::InputIR) -> TransformResult<(Self::OutputIR, Trace)> {
        if (*self.engine.borrow_mut())(&input).is_err() {
            return Err(TransformError::InvariantViolation("engine failed"));
        }

        let sum = input.logical_time;
        let mut diff = Vec::with_capacity(16);
        diff.extend_from_slice(&sum.to_le_bytes());
        diff.extend_from_slice(&sum.to_le_bytes());

        let bytes = serde_cbor::to_vec(&input).unwrap_or_default();
        let trace = Trace::new(&bytes, &bytes, WitnessDiff { bytes: diff });
        Ok((input, trace))
    }

    fn name(&self) -> &'static str { "engine_transition" }
}
