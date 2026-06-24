use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type StateHash = [u8; 32];
pub type EventId = [u8; 32];
pub type PayloadHash = [u8; 32];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RulesetVersion(pub u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionId {
    ClaimCreated = 1,
    ClaimVerified = 2,
}

#[derive(Debug, Error)]
pub enum TransitionError {
    #[error("Invalid parent state: expected {expected:?}, got {actual:?}")]
    InvalidParentState { expected: StateHash, actual: StateHash },
    #[error("Unsupported transition: {0:?}")]
    UnsupportedTransition(TransitionId),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("State mutation violated determinism")]
    NonDeterministic,
}

pub const PROTOCOL_VERSION: u16 = 1;
pub const GENESIS_STATE: StateHash = [0u8; 32];
