//! TSCP NTT Backend Abstraction
//!
//! Provides a uniform trait over NTT backends so the proving stack
//! can select between scalar and SIMD-accelerated implementations
//! without changing call sites.
//!
//! The trait wraps p3-dft's `TwoAdicSubgroupDft` interface, which is
//! the same interface the commitment and oracle-layer crates already
//! use. The difference is backend selection: `Radix2Dit` (scalar)
//! vs. a future AVX-512-accelerated butterfly.
//!
//! Feature flags:
//!   `scalar` (default) — reference backend using p3-dft's Radix2Dit
//!   `avx512`            — AVX-512-accelerated backend (x86_64 only)

#![cfg_attr(not(feature = "std"), no_std)]

pub mod backend;
pub mod scalar;

#[cfg(feature = "avx512")]
pub mod avx512;

// Re-export the primary types.
pub use backend::NttBackend;
pub use scalar::ScalarBackend;

#[cfg(feature = "avx512")]
pub use avx512::Avx512Backend;
