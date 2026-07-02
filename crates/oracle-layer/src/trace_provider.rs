use p3_field::Field;

/// The TOPO-LOCK compliant storage abstraction.
/// Decouples the trace representation from the evaluation logic.
pub trait TraceProvider<F: Field> {
    fn row(&self, index: usize) -> &[F];
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

/// The legacy storage provider for the current TSCP architecture.
pub struct VecTraceProvider<'a, F: Field> {
    trace: &'a [Vec<F>],
}

impl<'a, F: Field> VecTraceProvider<'a, F> {
    pub fn new(trace: &'a [Vec<F>]) -> Self {
        Self { trace }
    }
}

impl<'a, F: Field> TraceProvider<F> for VecTraceProvider<'a, F> {
    fn row(&self, index: usize) -> &[F] {
        &self.trace[index]
    }
    fn width(&self) -> usize {
        self.trace.first().map(|r| r.len()).unwrap_or(0)
    }
    fn height(&self) -> usize {
        self.trace.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;

    #[test]
    fn vec_trace_provider_preserves_layout_and_consistency() {
        let trace = vec![
            vec![BabyBear::ONE, BabyBear::ZERO],
            vec![BabyBear::ZERO, BabyBear::ONE],
        ];

        let provider = VecTraceProvider::new(&trace);

        assert_eq!(provider.row(0), trace[0]);
        assert_eq!(provider.row(1), trace[1]);
        assert_eq!(provider.width(), 2);
        assert_eq!(provider.height(), 2);
        assert!((0..provider.height()).all(|i| provider.row(i).len() == provider.width()));
    }
}
