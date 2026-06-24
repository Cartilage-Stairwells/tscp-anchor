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

// === TransitionReceipt (frozen protocol boundary) ===
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransitionReceipt {
    pub parent_state_hash: StateHash,
    pub event_hash: EventHash,
    pub child_state_hash: StateHash,
    pub kernel_version: KernelVersion,
}

impl TransitionReceipt {
    /// Domain-separated hash of the receipt.
    /// H("TSCP_TRANSITION_RECEIPT_V1", kernel_version, parent, event, child)
    pub fn hash(&self) -> TransitionHash {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"TSCP_TRANSITION_RECEIPT_V1");
        hasher.update(&self.kernel_version.to_le_bytes());
        hasher.update(&self.parent_state_hash);
        hasher.update(&self.event_hash);
        hasher.update(&self.child_state_hash);
        let hash = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(hash.as_bytes());
        out
    }
}
