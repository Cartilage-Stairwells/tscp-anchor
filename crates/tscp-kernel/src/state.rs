use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use blake3::Hasher;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub version: u64,
    pub claims: BTreeMap<String, ClaimState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimState {
    pub content_hash: [u8; 32],
    pub status: ClaimStatus,
    pub verified_by: Option<String>,
    pub score: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimStatus {
    Created,
    Verified,
}

impl State {
    pub fn hash(&self) -> StateHash {
        let bytes = crate::serialization::to_canonical_cbor(self)
           .expect("state serialization must succeed");
        let mut hasher = Hasher::new();
        hasher.update(&bytes);
        *hasher.finalize().as_bytes()
    }

    pub fn apply_claim_created(&mut self, claim_id: String, content_hash: [u8; 32]) {
        self.claims.insert(claim_id, ClaimState {
            content_hash,
            status: ClaimStatus::Created,
            verified_by: None,
            score: None,
        });
        self.version += 1;
    }

    pub fn apply_claim_verified(&mut self, claim_id: String, verifier: String, score: u8) {
        if let Some(claim) = self.claims.get_mut(&claim_id) {
            claim.status = ClaimStatus::Verified;
            claim.verified_by = Some(verifier);
            claim.score = Some(score);
            self.version += 1;
        }
    }
}
