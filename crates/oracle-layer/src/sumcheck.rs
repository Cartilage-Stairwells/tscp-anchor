use p3_field::Field;
use crate::oracle::MleOracle;

pub struct SumcheckClaim<F: Field> {
    pub claimed_sum: F,
    pub n_vars: usize,
}

/// One round of sumcheck: prover sends g(X) = sum over residual hypercube
/// Returns the degree-1 round polynomial [g(0), g(1)]
pub fn sumcheck_round<F: Field>(
    oracle: &impl MleOracle<F>,
    prefix: &[F],
) -> [F; 2] {
    let remaining = oracle.n_vars() - prefix.len();
    assert!(remaining > 0);

    let sum_size = 1usize << (remaining - 1);

    let eval_at = |bit: F| -> F {
        let mut pre = prefix.to_vec();
        pre.push(bit);
        (0..sum_size).map(|idx| {
            let mut full = pre.clone();
            for i in 0..(remaining - 1) {
                full.push(if (idx >> i) & 1 == 1 { F::ONE } else { F::ZERO });
            }
            oracle.eval(&full)
        }).fold(F::ZERO, |a, b| a + b)
    };

    [eval_at(F::ZERO), eval_at(F::ONE)]
}
