use serde::{Deserialize, Serialize};

pub type StateHash = [u8; 32];
pub type EventHash = [u8; 32];
pub type TransitionHash = [u8; 32];
pub type EventId = [u8; 16];
pub type RulesetVersion = u16;
pub type KernelVersion = RulesetVersion;

pub const GENESIS_STATE: StateHash = [0u8; 32];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionKind { ClaimCreated = 1, ClaimVerified = 2 }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransitionReceipt {
    pub parent_state_hash: StateHash,
    pub event_hash: EventHash,
    pub child_state_hash: StateHash,
    pub kernel_version: KernelVersion,
    pub kind: TransitionKind,
}

impl TransitionReceipt {
    pub fn hash(&self) -> TransitionHash {
        use blake3::Hasher;
        let mut h = Hasher::new();
        h.update(b"TSCP_TRANSITION_RECEIPT_V1");
        h.update(&self.kernel_version.to_le_bytes());
        h.update(&(self.kind as u16).to_le_bytes());
        h.update(&self.parent_state_hash);
        h.update(&self.event_hash);
        h.update(&self.child_state_hash);
        let mut out = [0u8;32]; out.copy_from_slice(h.finalize().as_bytes()); out
    }
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum TransitionError {
    #[error("Invalid parent")] InvalidParent,
    #[error("Precondition: {0}")] PreconditionFailed(String),
}
