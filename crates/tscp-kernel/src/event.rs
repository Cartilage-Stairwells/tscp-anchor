use crate::types::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub protocol_version: u16,
    pub event_id: EventId,
    pub parent_state_hash: StateHash,
    pub actor: ActorId,
    pub timestamp: u64,
    pub payload_hash: PayloadHash,
    pub transition_id: TransitionId,
    pub payload: Vec<u8>,
}

impl EventEnvelope {
    pub fn new(
        parent_state_hash: StateHash,
        actor: ActorId,
        timestamp: u64,
        transition_id: TransitionId,
        payload: Vec<u8>,
    ) -> Self {
        let payload_hash = Sha256::digest(&payload).into();
        let mut envelope = Self {
            protocol_version: PROTOCOL_VERSION,
            event_id: [0u8; 32],
            parent_state_hash,
            actor,
            timestamp,
            payload_hash,
            transition_id,
            payload,
        };
        envelope.event_id = envelope.compute_id();
        envelope
    }

    fn compute_id(&self) -> EventId {
        let mut hasher = Sha256::new();
        hasher.update(self.protocol_version.to_le_bytes());
        hasher.update(self.parent_state_hash);
        hasher.update(self.actor.0.as_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.payload_hash);
        hasher.update([self.transition_id as u8]);
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimCreatedPayload {
    pub claim_id: String,
    pub content_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimVerifiedPayload {
    pub claim_id: String,
    pub evidence_hash: [u8; 32],
    pub score: u8,
}
