//! Placeholder NTT backend (not yet AVX-512 accelerated).
//!
//! This backend delegates to `p3_dft::Radix2Dit` — the same generic scalar
//! implementation used by `ScalarBackend`. It exists as a structural
//! placeholder so that the proving stack can reference a second backend
//! identity for evidence provenance recording.
//!
//! IMPORTANT: This is NOT an AVX-512 implementation. It does not use any
//! SIMD intrinsics. The test `placeholder_backend_matches_scalar` is a
//! tautology — it compares two wrappers around the same `Radix2Dit` and
//! cannot establish AVX-512 correctness. It exists only as a regression
//! guard to confirm the placeholder still produces correct NTT results.
//!
//! When a real AVX-512 backend is implemented, it must:
//! 1. Use actual `_mm512_*` intrinsics for vectorized butterfly operations
//! 2. Be tested against the scalar reference with a differential test
//! 3. Have that differential test explicitly labeled as an AVX-512
//!    equivalence proof, not a placeholder regression
//!
//! See GitHub Issue #27 for the full analysis of why the previous
//! tautological test was a false positive.

use alloc::vec::Vec;
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::TwoAdicField;

use crate::backend::NttBackend;

/// Placeholder NTT backend that delegates to Plonky3's scalar `Radix2Dit`.
///
/// Despite the historical name `Avx512Backend`, this struct does not
/// perform any AVX-512 computation. See the module documentation for details.
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
        let owned: Vec<F> = self.dft.dft(vals.to_vec());
        vals.copy_from_slice(&owned);
    }

    fn inverse(&self, vals: &mut [Self::Field]) {
        let owned: Vec<F> = self.dft.idft(vals.to_vec());
        vals.copy_from_slice(&owned);
    }

    fn name(&self) -> &'static str {
        "placeholder-radix2-dit"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scalar::ScalarBackend;
    use p3_baby_bear::BabyBear;

    type F = BabyBear;

    /// Placeholder regression test: confirms the placeholder backend produces
    /// correct NTT results by comparing against the scalar reference.
    ///
    /// This is NOT an AVX-512 equivalence proof. Both backends delegate to
    /// the same `Radix2Dit` implementation, so this test is tautological —
    /// it compares a function against itself. It serves only as a regression
    /// guard for the placeholder's correctness, not as evidence that an
    /// AVX-512 path is correct.
    ///
    /// When a real AVX-512 backend is implemented, a separate differential
    /// test must be added that compares actual AVX-512 intrinsics output
    /// against the scalar reference.
    #[test]
    fn placeholder_backend_matches_scalar() {
        let scalar = ScalarBackend::<F>::default();
        let placeholder = Avx512Backend::<F>::default();

        let input: Vec<F> = (0..16).map(|i| F::new(i * 7 + 3)).collect();

        let mut s_vals = input.clone();
        let mut p_vals = input.clone();

        scalar.forward(&mut s_vals);
        placeholder.forward(&mut p_vals);

        assert_eq!(
            s_vals, p_vals,
            "placeholder backend must match scalar (regression guard only)"
        );

        scalar.inverse(&mut s_vals);
        placeholder.inverse(&mut p_vals);

        assert_eq!(
            s_vals, p_vals,
            "inverse must also match (regression guard only)"
        );
    }

    #[test]
    fn placeholder_backend_name() {
        let backend = Avx512Backend::<F>::default();
        assert_eq!(backend.name(), "placeholder-radix2-dit");
    }
}
