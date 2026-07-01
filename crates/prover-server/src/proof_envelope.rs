use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum ProofVersion {
    V0_6_1 = 1,
    V0_6_2 = 2,
}

impl ProofVersion {
    pub const CURRENT: Self = Self::V0_6_2;
}

#[derive(Serialize, Deserialize)]
pub struct ProofEnvelope {
    pub version: ProofVersion,
    pub plonky3_semver: [u8; 3],
    pub claim: u64,
    pub payload: Vec<u8>,
}

impl ProofEnvelope {
    pub fn seal_061(claim: u64, payload: Vec<u8>) -> Self {
        Self { version: ProofVersion::V0_6_1, plonky3_semver: [0,6,1], claim, payload }
    }
    pub fn seal_062(claim: u64, payload: Vec<u8>) -> Self {
        Self { version: ProofVersion::V0_6_2, plonky3_semver: [0,6,2], claim, payload }
    }
    pub fn open(&self) -> Result<&[u8], ProofError> {
        match self.version {
            ProofVersion::V0_6_1 => Ok(&self.payload),
            ProofVersion::V0_6_2 => Err(ProofError::UnsupportedVersion {
                got: ProofVersion::V0_6_2,
                expected: ProofVersion::V0_6_1,
                reason: "V0_6_2 verification not yet implemented",
            }),
        }
    }
    pub fn verify_golden(&self) -> Result<(), ProofError> {
        if self.claim!= 294373 {
            return Err(ProofError::GoldenMismatch { expected: 294373, got: self.claim });
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ProofError {
    #[error("Unsupported proof version: got {got:?}, expected {expected:?} - {reason}")]
    UnsupportedVersion { got: ProofVersion, expected: ProofVersion, reason: &'static str },
    #[error("Golden corpus mismatch: expected claim {expected}, got {got}")]
    GoldenMismatch { expected: u64, got: u64 },
}
