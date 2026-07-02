use p3_field::{Field, TwoAdicField};
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};

/// Compute vanishing polynomial Z_H(x) = x^n - 1 for domain of size n.
pub fn vanishing_poly<F: Field>(n: usize) -> Vec<F> {
    let mut coeffs = vec![F::ZERO; n + 1];
    coeffs[0] = -F::ONE;
    coeffs[n] = F::ONE;
    coeffs
}

/// Polynomial long division: numerator / denominator (den must be monic).
/// If numerator degree < denominator degree, quotient is zero polynomial.
pub fn poly_div<F: Field>(num: &[F], den: &[F]) -> Vec<F> {
    // If numerator degree < denominator degree, quotient is zero.
    if num.len() < den.len() {
        return vec![F::ZERO; 0]; // empty quotient
    }
    let mut q = vec![F::ZERO; num.len() - den.len() + 1];
    let mut r = num.to_vec();
    let den_lead = den.last().unwrap();
    for k in (0..=r.len() - den.len()).rev() {
        let coeff = r[k + den.len() - 1] * den_lead.inverse();
        q[k] = coeff;
        for j in 0..den.len() {
            r[k + j] -= coeff * den[j];
        }
    }
    q
}

/// Build quotient Q(x) = C(x) / Z_H(x) where C is the constraint polynomial.
pub fn build_quotient<F: Field>(constraint_poly: &[F], domain_size: usize) -> Vec<F> {
    let van = vanishing_poly(domain_size);
    poly_div(constraint_poly, &van)
}

/// Interpolate evaluations (on roots of unity) to coefficients using inverse DFT.
/// Assumes evals length is a power of two and F is a TwoAdicField.
pub fn interpolate<F: TwoAdicField>(evals: &[F]) -> Vec<F> {
    let n = evals.len();
    assert!(n.is_power_of_two(), "domain size must be power of two");
    let dft = Radix2Dit::default();
    dft.idft(evals.to_vec())
}

/// Forward DFT: coefficients -> evaluations.
pub fn evaluate<F: TwoAdicField>(coeffs: &[F]) -> Vec<F> {
    let n = coeffs.len();
    assert!(n.is_power_of_two(), "domain size must be power of two");
    let dft = Radix2Dit::default();
    dft.dft(coeffs.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeCharacteristicRing;
    type F = BabyBear;

    #[test]
    fn test_vanishing_poly() {
        let n = 4;
        let van = vanishing_poly::<F>(n);
        assert_eq!(van.len(), n + 1);
        assert_eq!(van[0], -F::ONE);
        assert_eq!(van[n], F::ONE);
        let omega = F::two_adic_generator(n.trailing_zeros() as usize);
        let mut x = F::ONE;
        for _ in 0..n {
            let mut val = F::ZERO;
            for (i, &c) in van.iter().enumerate() {
                val += c * x.exp_u64(i as u64);
            }
            assert_eq!(val, F::ZERO);
            x *= omega;
        }
    }

    #[test]
    fn test_poly_div() {
        let num = vec![F::ZERO, F::ZERO, F::ONE]; // x^2
        let den = vec![-F::ONE, F::ONE];          // x - 1
        let q = poly_div(&num, &den);
        assert_eq!(q, vec![F::ONE, F::ONE]);      // x + 1
    }

    #[test]
    fn test_poly_div_zero_quotient() {
        // numerator degree < denominator degree -> quotient should be empty
        let num = vec![F::ONE];                    // degree 0
        let den = vec![F::ZERO, F::ONE];           // x (degree 1)
        let q = poly_div(&num, &den);
        assert_eq!(q, vec![F::ZERO; 0]);           // empty quotient
    }

    #[test]
    fn test_interpolation_roundtrip() {
        let coeffs = vec![F::ONE, F::new(2), F::new(3), F::new(4)];
        let evals = evaluate(&coeffs);
        let roundtrip = interpolate(&evals);
        assert_eq!(roundtrip, coeffs);
    }
}
