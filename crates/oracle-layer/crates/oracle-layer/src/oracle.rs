use p3_field::PrimeField;
use p3_matrix::dense::RowMajorMatrix;
use std::sync::Arc;

pub trait MleOracle<F: PrimeField> {
    fn n_vars(&self) -> usize;
    fn eval(&self, point: &[F]) -> F;
    fn bind_prefix<'a>(&'a self, prefix: &[F]) -> Restricted<'a, F>;
    fn eval_bound(&self, prefix: &[F], suffix: &[F]) -> F {
        self.bind_prefix(prefix).eval(suffix)
    }
}

pub trait Restricted<'a, F: PrimeField> {
    fn remaining_vars(&self) -> usize;
    fn eval(&self, suffix: &[F]) -> F;
}

#[derive(Clone, Debug)]
pub struct ColumnOracle<F> {
    trace: Arc<RowMajorMatrix<F>>,
    col_idx: usize,
    n_vars: usize,
}

impl<F: PrimeField> ColumnOracle<F> {
    pub fn new(trace: Arc<RowMajorMatrix<F>>, col_idx: usize) -> Self {
        let height = trace.height();
        assert!(height.is_power_of_two());
        let n_vars = height.ilog2() as usize;
        assert!(col_idx < trace.width());
        Self { trace, col_idx, n_vars }
    }
}

impl<F: PrimeField> MleOracle<F> for ColumnOracle<F> {
    fn n_vars(&self) -> usize { self.n_vars }
    fn eval(&self, point: &[F]) -> F {
        assert_eq!(point.len(), self.n_vars);
        evaluate_mle(&self.raw_column(), point)
    }
    fn bind_prefix<'a>(&'a self, prefix: &[F]) -> Restricted<'a, F> {
        RestrictedColumn { oracle: self, prefix: prefix.to_vec() }
    }
}

impl<F: PrimeField> ColumnOracle<F> {
    fn raw_column(&self) -> Vec<F> {
        (0..self.trace.height())
            .map(|i| self.trace.values[i * self.trace.width() + self.col_idx])
            .collect()
    }
}

struct RestrictedColumn<'a, F> {
    oracle: &'a ColumnOracle<F>,
    prefix: Vec<F>,
}

impl<'a, F: PrimeField> Restricted<'a, F> for RestrictedColumn<'a, F> {
    fn remaining_vars(&self) -> usize { self.oracle.n_vars - self.prefix.len() }
    fn eval(&self, suffix: &[F]) -> F {
        let mut full = self.prefix.clone();
        full.extend_from_slice(suffix);
        self.oracle.eval(&full)
    }
}

fn evaluate_mle<F: PrimeField>(values: &[F], point: &[F]) -> F {
    let n = point.len();
    let mut result = F::ZERO;
    for idx in 0..(1 << n) {
        let mut basis = F::ONE;
        for i in 0..n {
            let bit = (idx >> i) & 1;
            basis *= if bit == 1 { point[i] } else { F::ONE - point[i] };
        }
        result += values[idx] * basis;
    }
    result
}
