use crate::oracle::ColumnOracle;
use p3_field::PrimeField;
use p3_matrix::dense::RowMajorMatrix;
use std::sync::Arc;

pub struct AdversarialLoadGenerator {
    pub max_rows: usize,
    pub max_arity: usize,
}

impl AdversarialLoadGenerator {
    pub fn new(max_rows: usize, max_arity: usize) -> Self {
        Self { max_rows, max_arity }
    }

    pub fn high_density_trace<F: PrimeField>(&self, rows: usize) -> Arc<RowMajorMatrix<F>> {
        let rows = rows.min(self.max_rows);
        let mut data = vec![F::zero(); rows * 4];
        for i in 0..rows {
            data[i*4]     = F::from_canonical_u64(i as u64);
            data[i*4 + 1] = F::from_canonical_u64((i * 3) as u64);
            data[i*4 + 2] = F::from_canonical_u64((i * 7) as u64);
            data[i*4 + 3] = F::from_canonical_u64((i * 11) as u64);
        }
        Arc::new(RowMajorMatrix::new(data, 4))
    }

    pub fn high_arity_columns<F: PrimeField>(&self, trace: Arc<RowMajorMatrix<F>>) -> Vec<ColumnOracle<F>> {
        (0..self.max_arity.min(32))
            .map(|i| ColumnOracle::new(trace.clone(), i % trace.width()))
            .collect()
    }
}
