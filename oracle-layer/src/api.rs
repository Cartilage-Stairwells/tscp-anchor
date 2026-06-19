use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofRequest {
    pub transcript_data: Vec<u8>,
    pub public_parameters: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofResponse {
    pub proof: Vec<u8>,
    pub success: bool,
    pub error_message: Option<String>,
}
