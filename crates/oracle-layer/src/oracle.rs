use p3_field::PrimeCharacteristicRing;
use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;

pub trait MleOracle<F: Field> {
    fn n_vars(&self) -> usize;
    fn eval(&self, point: &[F]) -> F;
}

pub struct ColumnOracle<F> {
    pub values: Vec<F>,
    pub n_vars: usize,
}

impl<F: Field> ColumnOracle<F> {
    pub fn from_matrix(trace: &RowMajorMatrix<F>, col: usize) -> Self {
        let h = trace.height();
        assert!(h.is_power_of_two());
        let values = (0..h)
            .map(|i| trace.values[i * trace.width + col])
            .collect();
        Self { values, n_vars: h.ilog2() as usize }
    }
}

impl<F: Field> MleOracle<F> for ColumnOracle<F> {
    fn n_vars(&self) -> usize { self.n_vars }
    fn eval(&self, point: &[F]) -> F {
        evaluate_mle(&self.values, point)
    }
}

pub fn evaluate_mle<F: Field>(values: &[F], point: &[F]) -> F {
    let n = point.len();
    assert_eq!(values.len(), 1 << n);
    let mut result = F::ZERO;
    for idx in 0..(1usize << n) {
        let mut basis = F::ONE;
        for i in 0..n {
            let bit = (idx >> i) & 1;
            basis *= if bit == 1 { point[i] } else { F::ONE - point[i] };
        }
        result += values[idx] * basis;
    }
    result
}
