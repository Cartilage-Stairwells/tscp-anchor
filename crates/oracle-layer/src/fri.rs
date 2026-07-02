use p3_field::{Field, PrimeCharacteristicRing};

pub fn fri_fold_step<F: Field + PrimeCharacteristicRing>(
    evals: &[F],
    domain: &[F],
    beta: F,
) -> Vec<F> {
    let n = evals.len();
    assert_eq!(
        domain.len(),
        n,
        "domain and evaluations must have the same length"
    );
    assert!(
        n.is_power_of_two() && n >= 2,
        "fold input size must be a power of two >= 2"
    );
    let half = n / 2;

    let two_inv = F::from_u64(2).inverse();

    (0..half)
        .map(|i| {
            let x = domain[i];
            let p_x = evals[i];
            let p_neg_x = evals[i + half];

            let even_part = (p_x + p_neg_x) * two_inv;
            let odd_part = (p_x - p_neg_x) * two_inv * x.inverse();

            even_part + beta * odd_part
        })
        .collect()
}

pub fn fold_domain<F: Field>(domain: &[F]) -> Vec<F> {
    let half = domain.len() / 2;
    domain[..half].iter().map(|&x| x * x).collect()
}
