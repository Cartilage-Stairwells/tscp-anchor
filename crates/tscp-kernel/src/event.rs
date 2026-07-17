use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventEnvelope {
    pub event_id: EventId,
    pub parent_state_hash: StateHash,
    pub payload_hash: EventHash,
    pub logical_time: u64,
}

impl EventEnvelope {
    pub fn hash(&self) -> EventHash {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&self.event_id);
        hasher.update(&self.parent_state_hash);
        hasher.update(&self.payload_hash);
        hasher.update(&self.logical_time.to_le_bytes());
        let mut out = [0u8; 32];
        out.copy_from_slice(hasher.finalize().as_bytes());
        out
    }
}
