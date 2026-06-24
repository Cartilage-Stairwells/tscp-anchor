//! TSCP Transition Kernel v0.1
//! Deterministic state-transition machine for verifiable evolution

pub mod types;
pub mod state;
pub mod event;
pub mod transition;
pub mod serialization;
pub mod replay;

pub use types::*;
pub use transition::dispatch_event;
