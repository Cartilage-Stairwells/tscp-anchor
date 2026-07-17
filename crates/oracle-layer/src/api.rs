use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofRequest {
    pub job_id: String,
    pub public_inputs: Vec<u64>,
    pub claim: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofResponse {
    pub job_id: String,
    pub proof: Vec<u8>,
    pub status: String,
}
