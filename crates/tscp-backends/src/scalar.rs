//! ScalarBackend — reference NTT using p3-dft's Radix2Dit.
//!
//! This is the correctness reference. It uses the same scalar
//! radix-2 DFT that the commitment and oracle-layer crates already
//! use directly. The backend trait wraps it so call sites can switch
//! backends without changing their DFT call.

use p3_field::TwoAdicField;
use p3_dft::Radix2Dit;

use crate::backend::NttBackend;

/// Scalar reference backend using p3-dft's Radix2Dit.
///
/// This is the default and the correctness reference. All other
/// backends must produce identical results to this one.
pub struct ScalarBackend<F: TwoAdicField> {
    dft: Radix2Dit<F>,
}

impl<F: TwoAdicField> Default for ScalarBackend<F> {
    fn default() -> Self {
        ScalarBackend {
            dft: Radix2Dit::default(),
        }
    }
}

impl<F: TwoAdicField> NttBackend for ScalarBackend<F> {
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
        "scalar-radix2-dit"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeCharacteristicRing;

    type F = BabyBear;

    #[test]
    fn scalar_forward_inverse_roundtrip() {
        let backend = ScalarBackend::<F>::default();
        let original: Vec<F> = (0..8)
            .map(|i| F::from_canonical_u32(i))
            .collect();
        let mut vals = original.clone();

        backend.forward(&mut vals);
        backend.inverse(&mut vals);

        for (a, b) in original.iter().zip(vals.iter()) {
            assert_eq!(*a, *b, "roundtrip failed");
        }
    }

    #[test]
    fn scalar_backend_name() {
        let backend = ScalarBackend::<F>::default();
        assert_eq!(backend.name(), "scalar-radix2-dit");
    }
}
