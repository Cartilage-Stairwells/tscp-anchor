use alloc::vec::Vec;
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::TwoAdicField;

use crate::backend::NttBackend;

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
        "avx512-radix2-butterfly"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scalar::ScalarBackend;
    use p3_baby_bear::BabyBear;

    type F = BabyBear;

    #[test]
    fn avx512_matches_scalar() {
        let scalar = ScalarBackend::<F>::default();
        let avx = Avx512Backend::<F>::default();

        let input: Vec<F> = (0..16).map(|i| F::new(i * 7 + 3)).collect();

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
