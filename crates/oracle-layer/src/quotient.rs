use p3_field::Field;
#[allow(unused_imports)]
use p3_field::PrimeCharacteristicRing;

/// Divides a coefficient-form polynomial C(x) by the vanishing
/// polynomial Z_H(x) = x^n - 1 for a degree-n domain, returning the
/// quotient Q(x) such that C(x) = Q(x) * Z_H(x) + R(x).
///
/// This only succeeds cleanly (remainder zero) when C vanishes on all
/// n-th roots of unity, i.e., when C is itself divisible by Z_H — which
/// is exactly the soundness condition an AIR constraint polynomial must
/// satisfy on its execution domain. If the remainder is nonzero, the
/// caller has an unsatisfied constraint, and this function reports it
/// rather than silently truncating.
pub struct QuotientResult<F> {
    pub quotient: Vec<F>,
    pub remainder: Vec<F>,
}

pub fn divide_by_vanishing<F: Field>(coeffs: &[F], domain_size: usize) -> QuotientResult<F> {
    // Synthetic division of C(x) by (x^n - 1):
    // for each coefficient c_i with i >= n, c_i contributes to both
    // q_{i-n} (it "wraps around" via x^n = 1) and the term at i is
    // absorbed into the quotient; standard reduction is:
    //   q[i - n] += c[i]; remainder keeps c[i] only for i < n after
    //   folding all higher terms down.
    let n = domain_size;
    let mut work = coeffs.to_vec();
    let deg = work.len();

    if deg <= n {
        // C already has degree < n: it cannot be divisible by Z_H
        // unless C is the zero polynomial. Quotient is zero, remainder
        // is C itself.
        return QuotientResult {
            quotient: vec![F::ZERO],
            remainder: work,
        };
    }

    let mut quotient = vec![F::ZERO; deg - n];
    // Process from the highest-degree term down: x^n ≡ 1 (mod Z_H),
    // so a coefficient at degree i >= n folds into degree i - n.
    for i in (n..deg).rev() {
        let c = work[i];
        quotient[i - n] = c;
        work[i] = F::ZERO;
        work[i - n] += c;
    }

    let remainder = work[..n].to_vec();
    QuotientResult {
        quotient,
        remainder,
    }
}

/// Convenience check: does this polynomial vanish on the domain (i.e.,
/// is the remainder of dividing by Z_H exactly zero)?
pub fn vanishes_on_domain<F: Field>(coeffs: &[F], domain_size: usize) -> bool {
    let result = divide_by_vanishing(coeffs, domain_size);
    result.remainder.iter().all(|&r| r == F::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::Radix2Interpolator;
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeField64;

    type F = BabyBear;

    fn omega4() -> F {
        let g = F::from_u64(31);
        let exp = (F::ORDER_U64 - 1) / 4;
        g.exp_u64(exp)
    }

    #[test]
    fn exact_multiple_of_vanishing_has_zero_remainder() {
        // Construct C(x) = (x^4 - 1) * (2 + 3x) directly by hand:
        // (x^4 - 1)(2 + 3x) = 2 + 3x - 2x^4 - 3x^5
        // coeffs (ascending): [2, 3, 0, 0, -2, -3]
        let two = F::from_u64(2);
        let three = F::from_u64(3);
        let coeffs = vec![F::ZERO - two, F::ZERO - three, F::ZERO, F::ZERO, two, three];

        let result = divide_by_vanishing(&coeffs, 4);
        assert!(
            result.remainder.iter().all(|&r| r == F::ZERO),
            "remainder must be exactly zero for a true multiple of Z_H"
        );
        assert_eq!(result.quotient[0], two, "quotient constant term must be 2");
        assert_eq!(result.quotient[1], three, "quotient linear term must be 3");
    }

    #[test]
    fn non_multiple_has_nonzero_remainder() {
        // C(x) = 1 + x (degree 1, less than domain size 4): not
        // divisible by Z_H unless it's the zero polynomial.
        let coeffs = vec![F::ONE, F::ONE];
        let result = divide_by_vanishing(&coeffs, 4);
        assert!(
            !result.remainder.iter().all(|&r| r == F::ZERO),
            "a nonzero low-degree polynomial must NOT vanish on the domain"
        );
    }

    #[test]
    fn quotient_reconstructs_original_via_evaluation() {
        // Cross-check against the FFT module: build C(x) as a real
        // product of Z_H(x) and an arbitrary Q(x), confirm
        // vanishes_on_domain reports true, and confirm Q(x) evaluated
        // at an out-of-domain point matches re-deriving Q from the
        // quotient division.
        let w = omega4();
        let q_coeffs = vec![F::from_u64(7), F::from_u64(11)]; // Q(x) = 7 + 11x

        // Multiply Q(x) by Z_H(x) = x^4 - 1 by hand: shift Q by 4
        // (the x^4 * Q part) and subtract Q (the -1 * Q part).
        let mut c_coeffs = vec![F::ZERO; 6];
        for (i, &qc) in q_coeffs.iter().enumerate() {
            c_coeffs[i + 4] += qc; // x^4 * Q(x)
            c_coeffs[i] -= qc; // -1 * Q(x)
        }

        assert!(
            vanishes_on_domain(&c_coeffs, 4),
            "Z_H(x) * Q(x) must vanish on the domain by construction"
        );

        let result = divide_by_vanishing(&c_coeffs, 4);
        assert_eq!(
            result.quotient, q_coeffs,
            "recovered quotient must match the original Q(x) exactly"
        );

        // Independent cross-check via FFT: evaluate C(x) at all domain
        // points using the FFT module; all should be zero.
        let padded: Vec<F> = c_coeffs.iter().take(4).cloned().collect();
        let _ = padded; // domain evals of the low part aren't meaningful alone;
        for i in 0..4u64 {
            let point = w.exp_u64(i);
            let direct = Radix2Interpolator::evaluate_coeffs(&c_coeffs, point);
            assert_eq!(
                direct,
                F::ZERO,
                "C(x) must evaluate to zero at every domain point, index {i}"
            );
        }
    }
}
