//! NttBackend trait — the uniform interface over NTT implementations.
//!
//! The proving stack (commitment, oracle-layer) already consumes
//! `p3_dft::TwoAdicSubgroupDft`. This trait wraps that interface and
//! adds backend identification, so the proving stack can record which
//! backend produced a given proof — critical for evidence provenance.
//!
//! The trait is deliberately minimal. It does not know about:
//!   - TSCP custody or evidence protocol
//!   - AVX-512 specifics or CPU features
//!   - Montgomery arithmetic or field representation
//!
//! It knows: forward NTT, inverse NTT, and who am I.

use p3_field::TwoAdicField;

/// A backend for Number-Theoretic Transform operations.
///
/// Implementations include the scalar reference (`ScalarBackend`)
/// and the AVX-512-accelerated path (`Avx512Backend`, feature-gated).
pub trait NttBackend {
    /// The field this backend operates on.
    type Field: TwoAdicField;

    /// Forward NTT: coefficient form -> evaluation form.
    fn forward(&self, vals: &mut [Self::Field]);

    /// Inverse NTT: evaluation form -> coefficient form.
    fn inverse(&self, vals: &mut [Self::Field]);

    /// Backend identity string for provenance recording.
    /// e.g. "scalar-radix2-dit", "avx512-radix2-butterfly"
    fn name(&self) -> &'static str;
}
