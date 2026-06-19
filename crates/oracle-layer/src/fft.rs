use p3_field::Field;
#[allow(unused_imports)]
#[allow(unused_imports)]
use p3_field::PrimeCharacteristicRing;

/// Radix-2 DIT FFT/IFFT over a field with a known root of unity.
/// `omega` must be a primitive n-th root of unity, where n = values.len()
/// is a power of two. The caller supplies omega (BabyBear's multiplicative
/// group has order p-1 = 2^27 * 15 * ... so suitable roots exist for
/// power-of-two sizes up to 2^27).
pub struct Radix2Interpolator;

impl Radix2Interpolator {
    /// Forward FFT: coefficient form -> evaluation form (evaluations at
    /// omega^0, omega^1, ..., omega^(n-1)).
    pub fn fft<F: Field>(coeffs: &[F], omega: F) -> Vec<F> {
        let n = coeffs.len();
        assert!(n.is_power_of_two(), "fft size must be a power of two");
        let mut a = coeffs.to_vec();
        Self::fft_in_place(&mut a, omega);
        a
    }

    /// Inverse FFT: evaluation form -> coefficient form.
    /// Requires omega_inv = omega^-1 and divides by n at the end.
    pub fn ifft<F: Field>(evals: &[F], omega: F) -> Vec<F> {
        let n = evals.len();
        assert!(n.is_power_of_two(), "ifft size must be a power of two");
        let omega_inv = omega.inverse();
        let mut a = evals.to_vec();
        Self::fft_in_place(&mut a, omega_inv);
        let n_inv = F::from_u64(n as u64).inverse();
        for x in a.iter_mut() {
            *x *= n_inv;
        }
        a
    }

    /// In-place iterative radix-2 DIT FFT (Cooley-Tukey), bit-reversal
    /// permutation followed by butterfly stages.
    fn fft_in_place<F: Field>(a: &mut [F], omega: F) {
        let n = a.len();
        if n <= 1 {
            return;
        }

        // Bit-reversal permutation.
        let log_n = n.trailing_zeros();
        for i in 0..n {
            let j = Self::reverse_bits(i, log_n);
            if j > i {
                a.swap(i, j);
            }
        }

        // Butterfly stages.
        let mut len = 2;
        while len <= n {
            // omega_len is a primitive `len`-th root of unity, obtained
            // from the n-th root by exponentiating to n/len.
            let exp = (n / len) as u64;
            let omega_len = omega.exp_u64(exp);
            let half = len / 2;
            let mut start = 0;
            while start < n {
                let mut w = F::ONE;
                for k in 0..half {
                    let u = a[start + k];
                    let v = a[start + k + half] * w;
                    a[start + k] = u + v;
                    a[start + k + half] = u - v;
                    w *= omega_len;
                }
                start += len;
            }
            len <<= 1;
        }
    }

    fn reverse_bits(mut x: usize, bits: u32) -> usize {
        let mut r = 0usize;
        for _ in 0..bits {
            r = (r << 1) | (x & 1);
            x >>= 1;
        }
        r
    }

    /// Interpolate a single column: given evaluations on the domain
    /// {omega^0, ..., omega^(n-1)}, recover the coefficient-form
    /// polynomial via IFFT.
    pub fn interpolate_column<F: Field>(evals: &[F], omega: F) -> Vec<F> {
        Self::ifft(evals, omega)
    }

    /// Interpolate every column of a matrix (rows = evaluation points,
    /// cols = independent polynomials), returning one coefficient vector
    /// per column.
    pub fn interpolate_matrix<F: Field>(matrix: &[Vec<F>], omega: F) -> Vec<Vec<F>> {
        let n_cols = matrix.first().map(|r| r.len()).unwrap_or(0);
        let n_rows = matrix.len();
        (0..n_cols)
            .map(|c| {
                let col: Vec<F> = (0..n_rows).map(|r| matrix[r][c]).collect();
                Self::interpolate_column(&col, omega)
            })
            .collect()
    }

    /// Evaluate a coefficient-form polynomial at an arbitrary point
    /// (in-domain or out-of-domain), via Horner's method. Used for
    /// DEEP-style out-of-domain sampling once a quotient/witness
    /// polynomial is in coefficient form.
    pub fn evaluate_coeffs<F: Field>(coeffs: &[F], point: F) -> F {
        let mut result = F::ZERO;
        for &c in coeffs.iter().rev() {
            result = result * point + c;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;
    use p3_field::{PrimeField64};

    type F = BabyBear;

    // A known primitive 4th root of unity in BabyBear: BabyBear's
    // multiplicative group has order p-1 = 2^27 * 15. The canonical
    // generator g=31 (a known BabyBear primitive root) raised to the
    // power (p-1)/4 gives a primitive 4th root.
    fn omega4() -> F {
        let g = F::from_u64(31);
        let exp = (F::ORDER_U64 - 1) / 4;
        g.exp_u64(exp)
    }

    #[test]
    fn omega4_has_order_4() {
        let w = omega4();
        let w2 = w * w;
        let w4 = w2 * w2;
        assert_eq!(w4, F::ONE, "omega^4 must be 1");
        assert_ne!(w2, F::ONE, "omega^2 must not be 1 (omega must have exact order 4)");
    }

    #[test]
    fn fft_then_ifft_is_identity() {
        let w = omega4();
        let coeffs: Vec<F> = vec![1, 2, 3, 4].into_iter().map(F::from_u64).collect();
        let evals = Radix2Interpolator::fft(&coeffs, w);
        let back = Radix2Interpolator::ifft(&evals, w);
        assert_eq!(back, coeffs, "ifft(fft(x)) must recover x exactly");
    }

    #[test]
    fn fft_matches_direct_evaluation() {
        // p(x) = 1 + 2x + 3x^2 + 4x^3
        // Evaluating directly at omega^0, omega^1, omega^2, omega^3 by
        // Horner's method must match the FFT's output exactly.
        let w = omega4();
        let coeffs: Vec<F> = vec![1, 2, 3, 4].into_iter().map(F::from_u64).collect();
        let evals = Radix2Interpolator::fft(&coeffs, w);

        for i in 0..4u64 {
            let point = w.exp_u64(i);
            let direct = Radix2Interpolator::evaluate_coeffs(&coeffs, point);
            assert_eq!(evals[i as usize], direct,
                "fft output at index {i} must match direct Horner evaluation");
        }
    }

    #[test]
    fn interpolate_column_recovers_known_polynomial() {
        // Build evaluations of p(x) = 5 + 6x directly, then confirm
        // interpolation recovers [5, 6, 0, 0] in coefficient form
        // (degree < 2 padded to length 4).
        let w = omega4();
        let coeffs: Vec<F> = vec![5, 6, 0, 0].into_iter().map(F::from_u64).collect();
        let evals: Vec<F> = (0..4u64)
            .map(|i| Radix2Interpolator::evaluate_coeffs(&coeffs, w.exp_u64(i)))
            .collect();

        let recovered = Radix2Interpolator::interpolate_column(&evals, w);
        assert_eq!(recovered, coeffs);
    }

    #[test]
    fn interpolate_matrix_handles_multiple_columns_independently() {
        let w = omega4();
        let col_a: Vec<F> = vec![1, 0, 0, 0].into_iter().map(F::from_u64).collect(); // p(x)=1
        let col_b: Vec<F> = vec![0, 1, 0, 0].into_iter().map(F::from_u64).collect(); // p(x)=x

        let evals_a = Radix2Interpolator::fft(&col_a, w);
        let evals_b = Radix2Interpolator::fft(&col_b, w);

        let matrix: Vec<Vec<F>> = (0..4)
            .map(|i| vec![evals_a[i], evals_b[i]])
            .collect();

        let recovered = Radix2Interpolator::interpolate_matrix(&matrix, w);
        assert_eq!(recovered[0], col_a, "column 0 must recover independently");
        assert_eq!(recovered[1], col_b, "column 1 must recover independently");
    }
}
