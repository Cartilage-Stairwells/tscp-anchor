use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PulseArtifact {
    pub schema_version: String,
    pub artifact_id: String,
    pub created_at: String,
    pub provenance: Provenance,
    pub verification: Verification,
    pub benchmark: Benchmark,
    pub telemetry: Telemetry,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Provenance {
    pub manifest_sha256: String,
    pub repo_commit: String,
    pub binary_digest: String,
    pub proof_digest: String,
    pub transcript_digest: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pass,
    Fail,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Verification {
    pub status: VerificationStatus,
    pub verifier_unchanged: bool,
    pub correctness_gate_passed: bool,
    pub fiat_shamir_rounds: u32,
    pub public_inputs_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkLayer {
    Kernel,
    PrimitiveIntegration,
    EndToEnd,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Benchmark {
    pub layer: BenchmarkLayer,
    pub trace_size: usize,
    pub integrity: IntegrityMeasurement,
    pub performance: PerformanceMeasurement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntegrityMeasurement {
    pub proving_ms: f64,
    pub verification_ms: f64,
    pub total_pipeline_ms: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PerformanceMeasurement {
    pub proving_ms: f64,
    pub transcript_generation_ms: f64,
    pub sumcheck_ms: f64,
    pub fri_ms: Option<f64>,
    pub verification_outside_timed_region: bool, // ALWAYS true — the emitter must never set it to false.
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Statistics {
    pub repetitions: u32,
    pub median_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub stddev_ms: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Telemetry {
    pub peak_rss_kb: usize,
    pub proof_size_bytes: usize,
    pub transcript_size_bytes: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_size_bytes: Option<usize>,
}
