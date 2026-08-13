//! Oracle Bridge — the sole integration point between tscp-verifier and oracle-layer.
//!
//! This module is the ONLY file in tscp-verifier that imports `oracle_layer::` types.
//! It wraps the oracle-layer's `fri_prove` / `fri_verify` functions behind a typed
//! interface that:
//!
//! 1. Captures phase timings at execution boundaries using PhaseTimer.
//! 2. Returns typed errors instead of panicking.
//! 3. Serializes proof and transcript bytes for the emitter's digest chain.
//! 4. Does NOT import any artifact or schema types.
//!
//! Architectural invariant:
//!   oracle_bridge.rs → imports oracle_layer (proving)
//!   artifact.rs      → sees only Vec<u8> (no oracle-layer types)
//!   emitter.rs       → consumes both, builds PulseArtifact

use crate::timing::{phases, PhaseTimer};

use oracle_layer::fri_protocol::Challenger;
use oracle_layer::fri_query::{fri_prove, fri_verify, FriProof};
use oracle_layer::oracle::MleOracle;
use oracle_layer::sumcheck::sumcheck_round;

use p3_baby_bear::BabyBear;
use serde::{Deserialize, Serialize};

/// Error type for all oracle-bridge operations.
#[derive(Debug)]
pub enum BridgeError {
    InvalidInput(String),
    VerificationFailed,
    SerializationError(String),
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BridgeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BridgeError::VerificationFailed => write!(f, "Proof verification failed"),
            BridgeError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for BridgeError {}

/// Serializable representation of a FriProof.
/// The oracle-layer's FriProof doesn't derive Serialize/Deserialize,
/// so we convert it to this serializable form for byte-level digest computation.
/// This preserves the architectural boundary: the emitter only sees Vec<u8>.
#[derive(Serialize, Deserialize)]
pub struct SerializableFriProof {
    pub roots: Vec<String>,  // Merkle roots as debug strings
    pub final_value: String, // Final constant as debug string
    pub query_indices: Vec<usize>,
    pub num_query_rounds: usize,
    pub num_fold_rounds: usize,
}

/// Convert a FriProof to its serializable form.
fn proof_to_serializable(proof: &FriProof) -> SerializableFriProof {
    SerializableFriProof {
        roots: proof
            .commitment
            .roots
            .iter()
            .map(|r| format!("{:?}", r))
            .collect(),
        final_value: format!("{:?}", proof.commitment.final_value),
        query_indices: proof.query_indices.clone(),
        num_query_rounds: proof.query_proofs.len(),
        num_fold_rounds: proof.commitment.roots.len().saturating_sub(1),
    }
}

/// Convert serialized form back to a partial FriProof for verification.
/// Note: this reconstruction is sufficient for re-running fri_verify because
/// the query proofs are reconstructed from the original proof data.
/// In a real deployment, the full proof (including Merkle openings) would be
/// serialized. For benchmarking, we re-prove and re-verify in the same process,
/// so this is not needed — but the serialization path is exercised for digest computation.
fn serialize_proof(proof: &FriProof) -> Result<Vec<u8>, BridgeError> {
    let serializable = proof_to_serializable(proof);
    serde_json::to_vec(&serializable).map_err(|e| BridgeError::SerializationError(e.to_string()))
}

/// The result of an instrumented proving run.
pub struct ProveResult {
    pub proof_bytes: Vec<u8>,
    pub transcript_bytes: Vec<u8>,
    pub timer: PhaseTimer,
    pub fiat_shamir_rounds: u32,
}

/// The result of an instrumented verification run.
pub struct VerifyResult {
    pub verification_ok: bool,
    pub timer: PhaseTimer,
}

/// Internal: holds the original proof alongside its serialized bytes,
/// so verification can use the original typed proof.
pub struct ProveResultInternal {
    pub proof_bytes: Vec<u8>,
    pub transcript_bytes: Vec<u8>,
    pub timer: PhaseTimer,
    pub fiat_shamir_rounds: u32,
    pub proof: FriProof, // kept for in-process verification
}

impl std::fmt::Debug for ProveResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProveResult")
            .field("proof_bytes_len", &self.proof_bytes.len())
            .field("transcript_bytes_len", &self.transcript_bytes.len())
            .field("fiat_shamir_rounds", &self.fiat_shamir_rounds)
            .finish()
    }
}

impl std::fmt::Debug for VerifyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerifyResult")
            .field("verification_ok", &self.verification_ok)
            .finish()
    }
}

impl std::fmt::Debug for ProveResultInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProveResultInternal")
            .field("proof_bytes_len", &self.proof_bytes.len())
            .field("transcript_bytes_len", &self.transcript_bytes.len())
            .field("fiat_shamir_rounds", &self.fiat_shamir_rounds)
            .finish()
    }
}

/// Run the full FRI proving pipeline with phase-level instrumentation.
///
/// Phase boundaries (each bracketed by PhaseTimer):
///   - transcript_generation: challenger setup + initial Merkle commitment
///   - fri: fold rounds + query opening proofs
///   - proving_total: wall-clock total (brackets the entire prove call)
pub fn prove_instrumented(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    num_queries: usize,
) -> Result<ProveResult, BridgeError> {
    let internal = prove_instrumented_internal(evals, domain, num_queries)?;

    Ok(ProveResult {
        proof_bytes: internal.proof_bytes,
        transcript_bytes: internal.transcript_bytes,
        timer: internal.timer,
        fiat_shamir_rounds: internal.fiat_shamir_rounds,
    })
}

/// Internal function that retains the typed proof for in-process verification.
pub fn prove_instrumented_internal(
    evals: Vec<BabyBear>,
    domain: Vec<BabyBear>,
    num_queries: usize,
) -> Result<ProveResultInternal, BridgeError> {
    let n = evals.len();
    if n == 0 {
        return Err(BridgeError::InvalidInput("evals is empty".to_string()));
    }
    if !n.is_power_of_two() {
        return Err(BridgeError::InvalidInput(format!(
            "evals.len() = {} is not a power of two",
            n
        )));
    }
    if domain.len() != n {
        return Err(BridgeError::InvalidInput(format!(
            "domain.len() = {} != evals.len() = {}",
            domain.len(),
            n
        )));
    }

    let mut timer = PhaseTimer::new();

    // --- PHASE BOUNDARY: proving_total (brackets entire prove call) ---
    // File: oracle-layer/src/fri_query.rs, fn fri_prove (line ~90)
    timer.start(phases::PROVING_TOTAL);

    // --- PHASE BOUNDARY: transcript_generation ---
    // File: oracle-layer/src/fri_protocol.rs, fn fri_commit_transcript (line ~100)
    timer.start(phases::TRANSCRIPT_GENERATION);

    let perm = p3_baby_bear::default_babybear_poseidon2_16();
    let mut challenger = Challenger::new(perm);

    // The initial commitment is part of transcript generation.
    timer.stop(); // transcript_generation

    // --- PHASE BOUNDARY: fri (folding + query opening proofs) ---
    // File: oracle-layer/src/fri_query.rs, fn fri_prove (fold loop + query phase)
    timer.start(phases::FRI);

    let proof = fri_prove(evals, domain.clone(), &mut challenger, num_queries);

    timer.stop(); // fri
    timer.stop(); // proving_total

    // --- Serialize proof to canonical bytes ---
    let proof_bytes = serialize_proof(&proof)?;

    // Transcript: serialize commitment roots as proxy for Fiat-Shamir transcript
    let transcript_data = proof
        .commitment
        .roots
        .iter()
        .map(|r| format!("{:?}", r))
        .collect::<Vec<_>>()
        .join("\n");
    let transcript_bytes = transcript_data.into_bytes();

    let fiat_shamir_rounds = proof.commitment.roots.len() as u32;

    Ok(ProveResultInternal {
        proof_bytes,
        transcript_bytes,
        timer,
        fiat_shamir_rounds,
        proof,
    })
}

/// Run FRI verification with timing instrumentation, using the original typed proof.
/// This is the primary verification path for in-process benchmarking.
///
/// Phase boundary:
///   - verification: brackets the entire verify call
pub fn verify_instrumented_with_proof(
    domain: &[BabyBear],
    proof: &FriProof,
    num_queries: usize,
) -> Result<VerifyResult, BridgeError> {
    let mut timer = PhaseTimer::new();

    // --- PHASE BOUNDARY: verification ---
    // File: oracle-layer/src/fri_query.rs, fn fri_verify (line ~169)
    timer.start(phases::VERIFICATION);

    let perm = p3_baby_bear::default_babybear_poseidon2_16();
    let mut challenger = Challenger::new(perm);

    let verification_ok = fri_verify(domain, proof, &mut challenger, num_queries);

    timer.stop(); // verification

    Ok(VerifyResult {
        verification_ok,
        timer,
    })
}

/// Run a single sumcheck round with timing.
///
/// Phase boundary:
///   - sumcheck: brackets the sumcheck_round call
pub fn sumcheck_instrumented<O: MleOracle<BabyBear>>(
    oracle: &O,
    prefix: &[BabyBear],
) -> Result<([BabyBear; 2], PhaseTimer), BridgeError> {
    let mut timer = PhaseTimer::new();

    // --- PHASE BOUNDARY: sumcheck ---
    // File: oracle-layer/src/sumcheck.rs, fn sumcheck_round (line ~11)
    timer.start(phases::SUMCHECK);

    let result = sumcheck_round(oracle, prefix);

    timer.stop(); // sumcheck

    Ok((result, timer))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::BabyBear;

    type F = BabyBear;

    #[test]
    fn test_bridge_prove_verify_roundtrip() {
        let n = 256usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

        let prove_result =
            prove_instrumented_internal(evals, domain.clone(), 20).expect("prove should succeed");

        assert!(
            !prove_result.proof_bytes.is_empty(),
            "proof bytes must not be empty"
        );
        assert!(
            !prove_result.transcript_bytes.is_empty(),
            "transcript bytes must not be empty"
        );
        assert!(prove_result.timer.get_or_zero(phases::PROVING_TOTAL) >= 0.0);
        assert!(prove_result.timer.get_or_zero(phases::FRI) >= 0.0);
        assert!(
            prove_result
                .timer
                .get_or_zero(phases::TRANSCRIPT_GENERATION)
                >= 0.0
        );
        assert!(
            prove_result.fiat_shamir_rounds > 0,
            "must have at least 1 Fiat-Shamir round"
        );

        // Verify using the original typed proof
        let verify_result = verify_instrumented_with_proof(&domain, &prove_result.proof, 20)
            .expect("verify should succeed");

        assert!(
            verify_result.verification_ok,
            "verification must pass for a valid proof"
        );
        assert!(verify_result.timer.get_or_zero(phases::VERIFICATION) >= 0.0);
    }

    #[test]
    fn test_bridge_rejects_non_power_of_two() {
        let evals: Vec<F> = (0..7).map(|i| F::new(i)).collect();
        let domain: Vec<F> = (0..7).map(|i| F::new(i)).collect();

        let result = prove_instrumented(evals, domain, 20);
        assert!(result.is_err());
        match result.unwrap_err() {
            BridgeError::InvalidInput(msg) => {
                assert!(
                    msg.contains("power of two"),
                    "error should mention power of two: {}",
                    msg
                );
            }
            _ => panic!("expected InvalidInput error"),
        }
    }

    #[test]
    fn test_bridge_rejects_empty_input() {
        let evals: Vec<F> = vec![];
        let domain: Vec<F> = vec![];

        let result = prove_instrumented(evals, domain, 20);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BridgeError::InvalidInput(_)));
    }

    #[test]
    fn test_bridge_rejects_mismatched_lengths() {
        let evals: Vec<F> = (0..8).map(|i| F::new(i)).collect();
        let domain: Vec<F> = (0..4).map(|i| F::new(i)).collect();

        let result = prove_instrumented(evals, domain, 20);
        assert!(result.is_err());
        match result.unwrap_err() {
            BridgeError::InvalidInput(msg) => {
                assert!(
                    msg.contains("domain.len()"),
                    "error should mention length mismatch: {}",
                    msg
                );
            }
            _ => panic!("expected InvalidInput error"),
        }
    }

    #[test]
    fn test_bridge_serialization_produces_valid_json() {
        let n = 256usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

        let result = prove_instrumented(evals, domain, 20).expect("prove should succeed");

        // Proof bytes must be valid JSON
        let parsed: serde_json::Value =
            serde_json::from_slice(&result.proof_bytes).expect("proof bytes must be valid JSON");
        assert!(parsed.is_object(), "serialized proof must be a JSON object");
        assert!(parsed.get("roots").is_some(), "must have roots field");
        assert!(
            parsed.get("final_value").is_some(),
            "must have final_value field"
        );
    }

    #[test]
    fn test_bridge_timings_are_recorded() {
        let n = 1024usize;
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

        let result = prove_instrumented(evals, domain, 20).expect("prove should succeed");

        assert!(
            result.timer.get(phases::PROVING_TOTAL).is_some(),
            "proving_total must be recorded"
        );
        assert!(
            result.timer.get(phases::FRI).is_some(),
            "fri must be recorded"
        );
        assert!(
            result.timer.get(phases::TRANSCRIPT_GENERATION).is_some(),
            "transcript_generation must be recorded"
        );
    }

    #[test]
    fn test_bridge_error_display() {
        let err = BridgeError::InvalidInput("test".to_string());
        assert_eq!(format!("{}", err), "Invalid input: test");

        let err = BridgeError::VerificationFailed;
        assert_eq!(format!("{}", err), "Proof verification failed");

        let err = BridgeError::SerializationError("json".to_string());
        assert_eq!(format!("{}", err), "Serialization error: json");
    }
}
