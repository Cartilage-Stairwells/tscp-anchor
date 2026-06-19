use p3_poseidon2::{Poseidon2, Poseidon2Config};
use p3_symmetric::{Absorb, CryptographicSponge, TruncatedPermutation};

/// A simple Fiat-Shamir transcript using Poseidon2.
pub struct Transcript<F: Field, const WIDTH: usize, P: TruncatedPermutation<F, WIDTH>> {
    sponge: Poseidon2<F, Poseidon2Config<F, WIDTH, P>, WIDTH, 16>,
}

impl<F: Field, const WIDTH: usize, P: TruncatedPermutation<F, WIDTH>> Transcript<F, WIDTH, P> {
    pub fn new() -> Self {
        let config = Poseidon2Config::new();
        let sponge = Poseidon2::new(config);
        Self { sponge }
    }

    pub fn absorb_field(&mut self, value: &F) {
        self.sponge.absorb(value);
    }

    pub fn squeeze_field(&mut self) -> F {
        let mut buf = [F::ZERO; 1];
        self.sponge.squeeze_into(&mut buf);
        buf[0]
    }

    pub fn absorb_u64(&mut self, value: u64) {
        // Convert u64 to field element (truncate to 31 bits safely)
        let f = F::from_canonical_u64(value);
        self.absorb_field(&f);
    }

    pub fn squeeze_u64(&mut self) -> u64 {
        self.squeeze_field().as_canonical_u64()
    }
}
