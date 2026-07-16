//! AVX-512 Backend — SIMD-accelerated NTT.
//!
//! Feature-gated behind `avx512`. Only compiled for x86_64 targets
//! with AVX-512F and AVX-512DQ support.
//!
//! Currently delegates to ScalarBackend because the AVX-512 butterfly
//! implementation lives in the separate avx512-butterfly repository.
//! This module exists to:
//!   1. Establish the feature-gated entry point
//!   2. Verify the trait is implementable behind a feature gate
//!   3. Provide the compile-time plumbing for when the real
//!      AVX-512 butterfly is wired in
//!
//! When the avx512-butterfly butterfly is integrated, this module
//! will use std::arch::x86_64 intrinsics directly. Until then,
//! correctness is preserved by delegation to the scalar reference.

use p3_field::TwoAdicField;
use p3_dft::Radix2Dit;

use crate::backend::NttBackend;

/// AVX-512-accelerated NTT backend.
///
/// Currently delegates to scalar. When the real SIMD butterfly is
/// wired in, this will use __m512i intrinsics for the radix-2
/// butterfly with Montgomery reduction.
pub struct Avx512Backend<F: TwoAdicField> {
    dft: Radix2Dit<F>,
}

impl<F: TwoAdicField> Default for Avx512Backend<F> {
    fn default() -> Self {
        Avx512Backend {
            dft: Radix2Dit::default(),
        }
    }
}

impl<F: TwoAdicField> NttBackend for Avx512Backend<F> {
    type Field = F;

    fn forward(&self, vals: &mut [Self::Field]) {
        // TODO: Replace with AVX-512 butterfly when avx512-butterfly is integrated.
        // Until then, delegate to scalar to preserve correctness.
        let owned: Vec<F> = self.dft.dft(vals.to_vec());
        vals.copy_from_slice(&owned);
    }

    fn inverse(&self, vals: &mut [Self::Field]) {
        let owned: Vec<F> = self.dft.idft(vals.to_vec());
        vals.copy_from_slice(&owned);
    }

    fn name(&self) -> &'static str {
        "avx512-radix2-butterfly"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeCharacteristicRing;
    use crate::scalar::ScalarBackend;

    type F = BabyBear;

    #[test]
    fn avx512_matches_scalar() {
        let scalar = ScalarBackend::<F>::default();
        let avx = Avx512Backend::<F>::default();

        let input: Vec<F> = (0..16)
            .map(|i| F::from_canonical_u32(i * 7 + 3))
            .collect();

        let mut s_vals = input.clone();
        let mut a_vals = input.clone();

        scalar.forward(&mut s_vals);
        avx.forward(&mut a_vals);

        assert_eq!(s_vals, a_vals, "AVX-512 backend must match scalar");

        scalar.inverse(&mut s_vals);
        avx.inverse(&mut a_vals);

        assert_eq!(s_vals, a_vals, "inverse must also match");
    }

    #[test]
    fn avx512_name() {
        let backend = Avx512Backend::<F>::default();
        assert_eq!(backend.name(), "avx512-radix2-butterfly");
    }
}
