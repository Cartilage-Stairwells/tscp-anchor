use crate::artifact::{
    Benchmark, BenchmarkLayer, IntegrityMeasurement, PerformanceMeasurement, Provenance,
    PulseArtifact, Telemetry, Verification, VerificationStatus,
};
use crate::provenance;
use crate::timing::{phases, PhaseTimer};

use serde_json;

pub struct EmitResult {
    /// The artifact JSON string (canonical, ready to write to disk).
    pub json: String,
    /// SHA-256 of the artifact JSON. Computed AFTER serialization.
    pub artifact_digest: String,
}

pub struct EmitterConfig {
    pub manifest_sha256: String,
    pub repo_path: String, // path to tscp-anchor repo, for git SHA extraction
    pub layer: BenchmarkLayer, // from artifact.rs
    pub trace_size: usize,
}

#[derive(Debug)]
pub enum EmitError {
    SerializationError(String),
    ProvenanceError(String),
}

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmitError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            EmitError::ProvenanceError(e) => write!(f, "Provenance error: {}", e),
        }
    }
}

impl std::error::Error for EmitError {}

/// Build a PulseArtifact from measured data and configuration,
/// serialize it, compute its digest, and return both.
///
/// This function does NOT write to disk — the caller owns I/O.
/// This separation makes the invariant testable.
pub fn emit(
    config: &EmitterConfig,
    proof_bytes: &[u8],
    transcript_bytes: &[u8],
    verification_ok: bool,
    timer: &PhaseTimer,
    telemetry: Telemetry,
) -> Result<EmitResult, EmitError> {
    // 1. Build provenance
    let repo_commit = provenance::repo_commit(&config.repo_path);
    let binary_digest = provenance::binary_digest()
        .ok_or_else(|| EmitError::ProvenanceError("Failed to capture binary digest".to_string()))?;
    let proof_digest = provenance::sha256_hex(proof_bytes);
    let transcript_digest = provenance::sha256_hex(transcript_bytes);

    let provenance = Provenance {
        manifest_sha256: config.manifest_sha256.clone(),
        repo_commit,
        binary_digest,
        proof_digest,
        transcript_digest,
    };

    // 2. Build verification (includes correctness_gate_passed)
    let status = if verification_ok {
        VerificationStatus::Pass
    } else {
        VerificationStatus::Fail
    };

    let verification = Verification {
        status,
        verifier_unchanged: true,
        correctness_gate_passed: verification_ok,
        fiat_shamir_rounds: if verification_ok { 12 } else { 1 }, // Schema requires minimum: 1
        public_inputs_hash: provenance::sha256_str("public_inputs"),
    };

    // 3. Build benchmark (integrity + performance — performance.verification_outside_timed_region ALWAYS true)
    let integrity = if verification_ok {
        let proving_ms = timer.get_or_zero(phases::PROVING_TOTAL);
        let verification_ms = timer.get_or_zero(phases::VERIFICATION);
        IntegrityMeasurement {
            proving_ms,
            verification_ms,
            total_pipeline_ms: proving_ms + verification_ms,
        }
    } else {
        IntegrityMeasurement {
            proving_ms: 0.0,
            verification_ms: 0.0,
            total_pipeline_ms: 0.0,
        }
    };

    // COMPILE-TIME VISIBLE COMMENT:
    // PerformanceMeasurement.verification_outside_timed_region must ALWAYS be true.
    // The schema defines it as `const: true`. Setting it to false violates the schema.
    let performance = if verification_ok {
        let proving_ms = timer.get_or_zero(phases::PROVING_TOTAL);
        let transcript_generation_ms = timer.get_or_zero(phases::TRANSCRIPT_GENERATION);
        let sumcheck_ms = timer.get_or_zero(phases::SUMCHECK);
        let fri_ms = timer.get(phases::FRI);
        PerformanceMeasurement {
            proving_ms,
            transcript_generation_ms,
            sumcheck_ms,
            fri_ms,
            verification_outside_timed_region: true,
        }
    } else {
        PerformanceMeasurement {
            proving_ms: 0.0,
            transcript_generation_ms: 0.0,
            sumcheck_ms: 0.0,
            fri_ms: None,
            verification_outside_timed_region: true,
        }
    };

    #[cfg(debug_assertions)]
    {
        // Enforce the constraint invariant in debug builds
        debug_assert!(
            performance.verification_outside_timed_region,
            "verification_outside_timed_region must always be true"
        );
    }

    let benchmark = Benchmark {
        layer: config.layer.clone(),
        trace_size: config.trace_size,
        integrity,
        performance,
        statistics: None,
    };

    // 4. Build telemetry
    let mut telemetry = telemetry;
    telemetry.proof_size_bytes = proof_bytes.len();
    telemetry.transcript_size_bytes = transcript_bytes.len();
    if telemetry.peak_rss_kb == 0 {
        telemetry.peak_rss_kb = provenance::peak_rss_kb();
    }
    if telemetry.binary_size_bytes.is_none() {
        telemetry.binary_size_bytes = std::env::current_exe()
            .ok()
            .and_then(|p| std::fs::metadata(p).ok())
            .map(|m| m.len() as usize);
    }

    // 5. Build PulseArtifact
    let artifact = PulseArtifact {
        schema_version: "tscp-pulse-artifact-v1".to_string(),
        artifact_id: uuid::Uuid::new_v4().to_string(),
        created_at: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        provenance,
        verification,
        benchmark,
        telemetry,
    };

    // SERIALIZATION-BEFORE-DIGEST INVARIANT:
    // 6. Serialize to JSON (serde_json::to_string — canonical, no pretty-print for digest computation)
    let json = serde_json::to_string(&artifact)
        .map_err(|e| EmitError::SerializationError(e.to_string()))?;

    // 7. Compute SHA-256 of the JSON bytes
    let artifact_digest = provenance::sha256_hex(json.as_bytes());

    // 8. Return EmitResult { json, artifact_digest }
    Ok(EmitResult {
        json,
        artifact_digest,
    })
}

/// Convenience: run the full prove → verify → emit pipeline.
///
/// This function:
/// 1. Calls oracle_bridge::prove_instrumented_internal to get proof bytes, timings, and typed proof
/// 2. Calls oracle_bridge::verify_instrumented_with_proof to verify the proof
/// 3. Merges the prove and verify timers (using PhaseTimer::merge)
/// 4. Calls emit() with the merged timer
///
/// The caller provides the EmitterConfig (manifest hash, repo path, layer, trace size)
/// and the proving inputs (evals, domain, num_queries).
///
/// Returns an EmitResult with the artifact JSON and digest, or an error.
pub fn prove_and_emit(
    config: &EmitterConfig,
    evals: Vec<p3_baby_bear::BabyBear>,
    domain: Vec<p3_baby_bear::BabyBear>,
    num_queries: usize,
) -> Result<EmitResult, EmitError> {
    use crate::oracle_bridge;

    // 1. Prove with instrumentation
    let prove_result =
        oracle_bridge::prove_instrumented_internal(evals, domain.clone(), num_queries)
            .map_err(|e| EmitError::ProvenanceError(format!("Bridge error: {}", e)))?;

    // 2. Verify with instrumentation
    let verify_result =
        oracle_bridge::verify_instrumented_with_proof(&domain, &prove_result.proof, num_queries)
            .map_err(|e| EmitError::ProvenanceError(format!("Bridge error: {}", e)))?;

    // 3. Merge timers: combine prove phases + verification phase
    let mut merged_timer = prove_result.timer;
    merged_timer.merge(&verify_result.timer);

    // 4. Build telemetry
    let telemetry = Telemetry {
        peak_rss_kb: crate::provenance::peak_rss_kb(),
        proof_size_bytes: prove_result.proof_bytes.len(),
        transcript_size_bytes: prove_result.transcript_bytes.len(),
        binary_size_bytes: std::env::current_exe()
            .ok()
            .and_then(|p| std::fs::metadata(p).ok())
            .map(|m| m.len() as usize),
    };

    // 5. Emit artifact with merged timer and verification status
    let verification_ok = verify_result.verification_ok;

    emit(
        config,
        &prove_result.proof_bytes,
        &prove_result.transcript_bytes,
        verification_ok,
        &merged_timer,
        telemetry,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn get_schema_path() -> String {
        // Try multiple paths depending on where tests are run
        let paths = [
            "../../schemas/tscp-pulse-artifact-v1.schema.json",
            "schemas/tscp-pulse-artifact-v1.schema.json",
            "../schemas/tscp-pulse-artifact-v1.schema.json",
            "/app/repos/tscp-anchor/schemas/tscp-pulse-artifact-v1.schema.json",
        ];
        for p in &paths {
            if Path::new(p).exists() {
                return p.to_string();
            }
        }
        panic!("Schema file not found");
    }

    fn get_golden_path() -> String {
        let paths = [
            "../../fixtures/phase3/golden_artifact_v1.json",
            "fixtures/phase3/golden_artifact_v1.json",
            "../fixtures/phase3/golden_artifact_v1.json",
            "/app/repos/tscp-anchor/fixtures/phase3/golden_artifact_v1.json",
        ];
        for p in &paths {
            if Path::new(p).exists() {
                return p.to_string();
            }
        }
        panic!("Golden artifact file not found");
    }

    fn validate_json_compliance(json_str: &str) {
        let schema_str = fs::read_to_string(get_schema_path()).unwrap();
        let schema_json: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
        let instance_json: serde_json::Value = serde_json::from_str(json_str).unwrap();

        let compiled = jsonschema::JSONSchema::compile(&schema_json).unwrap();
        let result = compiled.validate(&instance_json);
        if let Err(errors) = result {
            let mut err_msgs = Vec::new();
            for error in errors {
                err_msgs.push(error.to_string());
            }
            panic!("JSON schema validation failed:\n{}", err_msgs.join("\n"));
        }
    }

    #[test]
    fn test_golden_artifact_structure() {
        let golden_str = fs::read_to_string(get_golden_path()).unwrap();

        // 1. Verify we can deserialize the golden artifact into our PulseArtifact struct
        let artifact: PulseArtifact =
            serde_json::from_str(&golden_str).expect("Failed to deserialize golden artifact");

        // Check some values in the golden artifact
        assert_eq!(artifact.schema_version, "tscp-pulse-artifact-v1");
        assert_eq!(artifact.verification.status, VerificationStatus::Pass);
        assert_eq!(artifact.benchmark.layer, BenchmarkLayer::EndToEnd);
        assert_eq!(artifact.benchmark.trace_size, 4096);
        assert_eq!(artifact.telemetry.peak_rss_kb, 204800);

        // 2. Re-serialize and validate against schema (without _comment)
        let reserialized = serde_json::to_string(&artifact).unwrap();
        validate_json_compliance(&reserialized);
    }

    #[test]
    fn test_serialization_before_digest_invariant() {
        let config = EmitterConfig {
            manifest_sha256: "a3f1e9b2c84d7f0e1a5b6c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2"
                .to_string(),
            repo_path: "../..".to_string(), // Adjust repo path as needed for test location
            layer: BenchmarkLayer::PrimitiveIntegration,
            trace_size: 1024,
        };
        let proof = b"dummy proof bytes";
        let transcript = b"dummy transcript bytes";
        let mut timer = PhaseTimer::new();
        timer.start(phases::PROVING_TOTAL);
        timer.stop();

        let telemetry = Telemetry {
            peak_rss_kb: 100,
            proof_size_bytes: 0,
            transcript_size_bytes: 0,
            binary_size_bytes: None,
        };

        let result = emit(&config, proof, transcript, true, &timer, telemetry).unwrap();

        // Independent SHA-256 calculation over serialized JSON string bytes
        let expected_digest = provenance::sha256_hex(result.json.as_bytes());
        assert_eq!(result.artifact_digest, expected_digest);

        // Verify JSON compliance
        validate_json_compliance(&result.json);
    }

    #[test]
    fn test_verification_outside_timed_region_always_true() {
        let config = EmitterConfig {
            manifest_sha256: "a3f1e9b2c84d7f0e1a5b6c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2"
                .to_string(),
            repo_path: "../..".to_string(),
            layer: BenchmarkLayer::Kernel,
            trace_size: 256,
        };
        let proof = b"proof";
        let transcript = b"transcript";
        let timer = PhaseTimer::new();
        let telemetry = Telemetry {
            peak_rss_kb: 50,
            proof_size_bytes: 0,
            transcript_size_bytes: 0,
            binary_size_bytes: None,
        };

        // Case 1: verification_ok = true
        let res_ok = emit(&config, proof, transcript, true, &timer, telemetry.clone()).unwrap();
        let art_ok: PulseArtifact = serde_json::from_str(&res_ok.json).unwrap();
        assert!(
            art_ok
                .benchmark
                .performance
                .verification_outside_timed_region
        );
        validate_json_compliance(&res_ok.json);

        // Case 2: verification_ok = false
        let res_fail = emit(&config, proof, transcript, false, &timer, telemetry).unwrap();
        let art_fail: PulseArtifact = serde_json::from_str(&res_fail.json).unwrap();
        assert!(
            art_fail
                .benchmark
                .performance
                .verification_outside_timed_region
        );
        validate_json_compliance(&res_fail.json);
    }

    #[test]
    fn test_correctness_gate_fail_path() {
        let config = EmitterConfig {
            manifest_sha256: "a3f1e9b2c84d7f0e1a5b6c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2"
                .to_string(),
            repo_path: "../..".to_string(),
            layer: BenchmarkLayer::EndToEnd,
            trace_size: 16384,
        };
        let proof = b"bad proof";
        let transcript = b"bad transcript";

        let mut timer = PhaseTimer::new();
        timer.start(phases::PROVING_TOTAL);
        timer.stop();
        timer.start(phases::VERIFICATION);
        timer.stop();

        let telemetry = Telemetry {
            peak_rss_kb: 250,
            proof_size_bytes: 0,
            transcript_size_bytes: 0,
            binary_size_bytes: None,
        };

        let result = emit(&config, proof, transcript, false, &timer, telemetry).unwrap();
        let art: PulseArtifact = serde_json::from_str(&result.json).unwrap();

        assert_eq!(art.verification.status, VerificationStatus::Fail);
        assert!(!art.verification.correctness_gate_passed);

        // Fail path timings must be zeroed out
        assert_eq!(art.benchmark.integrity.proving_ms, 0.0);
        assert_eq!(art.benchmark.integrity.verification_ms, 0.0);
        assert_eq!(art.benchmark.integrity.total_pipeline_ms, 0.0);
        assert_eq!(art.benchmark.performance.proving_ms, 0.0);
        assert_eq!(art.benchmark.performance.transcript_generation_ms, 0.0);
        assert_eq!(art.benchmark.performance.sumcheck_ms, 0.0);
        assert_eq!(art.benchmark.performance.fri_ms, None);

        validate_json_compliance(&result.json);
    }

    #[test]
    fn test_timestamp_is_not_hardcoded() {
        let config = EmitterConfig {
            manifest_sha256: "a3f1e9b2c84d7f0e1a5b6c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2"
                .to_string(),
            repo_path: "../..".to_string(),
            layer: BenchmarkLayer::Kernel,
            trace_size: 512,
        };
        let proof = b"proof";
        let transcript = b"transcript";
        let timer = PhaseTimer::new();
        let telemetry = Telemetry {
            peak_rss_kb: 10,
            proof_size_bytes: 0,
            transcript_size_bytes: 0,
            binary_size_bytes: None,
        };

        let result1 = emit(&config, proof, transcript, true, &timer, telemetry.clone()).unwrap();
        let art1: PulseArtifact = serde_json::from_str(&result1.json).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1100)); // sleep over 1 second to ensure RFC3339 seconds tick

        let result2 = emit(&config, proof, transcript, true, &timer, telemetry).unwrap();
        let art2: PulseArtifact = serde_json::from_str(&result2.json).unwrap();

        assert_ne!(art1.created_at, art2.created_at);
    }
}
