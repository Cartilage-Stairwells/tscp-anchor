#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod backend;
pub mod scalar;

#[cfg(feature = "avx512")]
pub mod avx512;

pub use backend::NttBackend;
pub use scalar::ScalarBackend;

#[cfg(feature = "avx512")]
pub use avx512::Avx512Backend;
