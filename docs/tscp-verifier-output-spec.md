# TSCP Verifier Output Specification v1

**Phase 3 — Producer Contract**
**Status: SEALED**
**Date: 2026-07-23**

---

## Purpose

This document defines the producer responsibilities and artifact shape for the TSCP verifier
output pipeline. It is the contract between the verifier emitter (producer) and the dashboard
adapter (consumer).

The artifact — not the dashboard, not the benchmark harness, not the implementation language —
is the trust boundary. Both sides of the seam are bound to the schema. Neither can drift
without a schema version bump.

```
TSCP Verifier
      │
      ▼
Reference Emitter          ← producer (bound by this spec)
      │
      ▼
Pulse Artifact (.json)     ← the trust boundary
      │
      ▼
JSON Schema Validation     ← schemas/tscp-pulse-artifact-v1.schema.json
      │
      ▼
TscpAdapter                ← consumer (bound by this spec)
      │
      ▼
Dashboard / PulseView
```

---

## Schema Reference

**Location:** `schemas/tscp-pulse-artifact-v1.schema.json`
**Schema ID:** `tscp-pulse-artifact-v1`
**JSON Schema draft:** draft-07

The schema is the authoritative definition. This document explains intent and constraints
that schema validation cannot enforce alone.

---

## Acceptance Criteria

A conformant artifact must pass all four gates:

```
✓  Validates against schemas/tscp-pulse-artifact-v1.schema.json
✓  Reference emitter output matches golden artifact fixture structure
     (fixtures/phase3/golden_artifact_v1.json)
✓  TscpAdapter accepts the golden artifact without modification
✓  Schema validation rejects any artifact that violates an invariant
     (wrong layer claim, verification_outside_timed_region=false, etc.)
```

This creates the full producer parity loop:

```
Emitter → Golden Artifact → Schema → Adapter
```

Any drift breaks at least one gate.

---

## Producer Responsibilities

### 1. Timestamps must be real

```rust
// CORRECT — use real UTC creation time
fn current_timestamp() -> String {
    chrono::Utc::now()
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

// WRONG — fixed timestamp creates "reproducible but false" artifacts
fn current_timestamp() -> String {
    "2026-07-23T15:00:00Z".to_string()  // do not do this
}
```

`created_at` is a runtime field. It records when this specific artifact was emitted.
A hardcoded timestamp makes the artifact appear to be evidence of a specific moment
when it is not.

Deterministic fields (commit SHAs, digests, schema version) must be stable.
Runtime fields (timestamps, durations) must be real.

Cargo.toml addition required:
```toml
chrono = { version = "0.4", features = ["serde"] }
```

### 2. Provenance chain must be complete

Every artifact must carry the full backward chain:

```
manifest_sha256       ← frozen benchmark environment
  └─ repo_commit      ← exact code version
       └─ binary_digest    ← exact compiled binary
            └─ proof_digest      ← exact proof output
                 └─ transcript_digest   ← exact transcript
```

An independent verifier given this chain can:
1. Check `manifest_sha256` → verify the measurement environment
2. Checkout `repo_commit` → rebuild from source
3. Compare `binary_digest` → confirm build reproducibility
4. Rerun → compare `proof_digest` and `transcript_digest`

If any link is missing, the artifact is not self-contained evidence.

### 3. Correctness gate before timing

The measurement sequence is not negotiable:

```
prove(trace)
    │
    ▼
verify(proof)     ← must return true, abort if false
    │
    ▼
sha256(proof)     → proof_digest
sha256(transcript) → transcript_digest
    │
    ▼
record timing     ← only reached if gate passes
```

`correctness_gate_passed` must be `true` before any performance numbers are accepted.
If the gate fails, the artifact must be emitted with `status: "fail"` and no performance
numbers should be cited.

### 4. Two measurement types, never conflated

```
benchmark.integrity    = prove() + verify() in timed region
                         ← answers: is it correct, full pipeline cost?
                         ← NOT for speedup claims

benchmark.performance  = prove() only in timed region
                         verify() outside, but must pass
                         ← USE THIS for CPU→GPU speedup claims
```

`performance.verification_outside_timed_region` must always be `true`. The schema
enforces this as `const: true`. An artifact emitted with this `false` is schema-invalid
and will be rejected.

### 5. One layer per artifact

```
benchmark.layer  ∈  { kernel, primitive_integration, end_to_end }
```

One artifact covers exactly one layer. Do not aggregate kernel results into an
end-to-end artifact. The claim boundary must be preserved:

| Layer | Valid claim | Invalid cross-claim |
|---|---|---|
| `kernel` | "AVX-512 butterfly is 9.15× faster than scalar" | ~~"The prover is 9.15× faster"~~ |
| `primitive_integration` | "sumcheck improved 3.2× after AVX-512 integration" | ~~"End-to-end improved 3.2×"~~ |
| `end_to_end` | "Full prove+verify pipeline: 820ms → 610ms (manifest abc123)" | — |

---

## Reference Emitter Stub (Rust)

The reference emitter is the canonical producer implementation. It must emit artifacts
that validate against the schema and match the golden fixture structure.

```rust
// src/emitter.rs  (reference stub — replace TODOs before production use)

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub struct PulseArtifact {
    pub schema_version: String,
    pub artifact_id: String,
    pub created_at: String,
    pub provenance: Provenance,
    pub verification: Verification,
    pub benchmark: Benchmark,
    pub telemetry: Telemetry,
}

#[derive(Serialize, Deserialize)]
pub struct Provenance {
    pub manifest_sha256: String,
    pub repo_commit: String,
    pub binary_digest: String,
    pub proof_digest: String,
    pub transcript_digest: String,
}

#[derive(Serialize, Deserialize)]
pub struct Verification {
    pub status: String,                  // "pass" | "fail" | "error"
    pub verifier_unchanged: bool,
    pub correctness_gate_passed: bool,
    pub fiat_shamir_rounds: u32,
    pub public_inputs_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Benchmark {
    pub layer: String,                   // "kernel" | "primitive_integration" | "end_to_end"
    pub trace_size: usize,
    pub integrity: IntegrityMeasurement,
    pub performance: PerformanceMeasurement,
    pub statistics: Option<Statistics>,
}

#[derive(Serialize, Deserialize)]
pub struct IntegrityMeasurement {
    pub proving_ms: f64,
    pub verification_ms: f64,
    pub total_pipeline_ms: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceMeasurement {
    pub proving_ms: f64,
    pub transcript_generation_ms: f64,
    pub sumcheck_ms: f64,
    pub fri_ms: Option<f64>,
    pub verification_outside_timed_region: bool,  // always true
}

#[derive(Serialize, Deserialize)]
pub struct Statistics {
    pub repetitions: u32,
    pub median_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub stddev_ms: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Telemetry {
    pub peak_rss_kb: usize,
    pub proof_size_bytes: usize,
    pub transcript_size_bytes: usize,
    pub binary_size_bytes: usize,
}

/// SHA-256 a byte slice, return lowercase hex string.
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Real UTC timestamp. Do not replace with a fixed string.
fn current_timestamp() -> String {
    chrono::Utc::now()
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

pub fn emit_artifact(
    trace_size: usize,
    manifest_sha256: &str,
    repo_commit: &str,
    binary_digest: &str,
) -> Result<PulseArtifact, Box<dyn std::error::Error>> {

    // ── 1. PROVE ──────────────────────────────────────────────────────────
    // TODO: replace with oracle_layer::prove(trace_size)
    let (proof_bytes, transcript_bytes) = {
        let proof = vec![0u8; trace_size * 32];       // placeholder
        let transcript = vec![0u8; trace_size * 8];   // placeholder
        (proof, transcript)
    };

    // ── 2. VERIFY (correctness gate — before any timing is accepted) ──────
    // TODO: replace with oracle_layer::verify(&proof_bytes)
    let verification_ok = true;  // placeholder
    if !verification_ok {
        return Ok(PulseArtifact {
            schema_version: "tscp-pulse-artifact-v1".into(),
            artifact_id: uuid_v4(),
            created_at: current_timestamp(),
            provenance: Provenance {
                manifest_sha256: manifest_sha256.into(),
                repo_commit: repo_commit.into(),
                binary_digest: binary_digest.into(),
                proof_digest: sha256_hex(&proof_bytes),
                transcript_digest: sha256_hex(&transcript_bytes),
            },
            verification: Verification {
                status: "fail".into(),
                verifier_unchanged: true,
                correctness_gate_passed: false,
                fiat_shamir_rounds: 0,
                public_inputs_hash: String::new(),
            },
            benchmark: Benchmark {
                layer: "end_to_end".into(),
                trace_size,
                integrity: IntegrityMeasurement { proving_ms: 0.0, verification_ms: 0.0, total_pipeline_ms: 0.0 },
                performance: PerformanceMeasurement {
                    proving_ms: 0.0,
                    transcript_generation_ms: 0.0,
                    sumcheck_ms: 0.0,
                    fri_ms: None,
                    verification_outside_timed_region: true,
                },
                statistics: None,
            },
            telemetry: Telemetry { peak_rss_kb: 0, proof_size_bytes: 0, transcript_size_bytes: 0, binary_size_bytes: 0 },
        });
    }

    // ── 3. DIGEST ─────────────────────────────────────────────────────────
    let proof_digest = sha256_hex(&proof_bytes);
    let transcript_digest = sha256_hex(&transcript_bytes);

    // ── 4. INTEGRITY timing (prove + verify, both timed) ──────────────────
    let t_integrity = Instant::now();
    // TODO: oracle_layer::prove(trace_size)  (retimed for integrity measurement)
    let integrity_proving_ms = t_integrity.elapsed().as_secs_f64() * 1000.0;

    let t_verify = Instant::now();
    // TODO: oracle_layer::verify(&proof_bytes)
    let verification_ms = t_verify.elapsed().as_secs_f64() * 1000.0;

    // ── 5. PERFORMANCE timing (prove ONLY in timed region) ────────────────
    let t_perf = Instant::now();
    // TODO: oracle_layer::prove(trace_size)  (retimed for performance)
    let performance_proving_ms = t_perf.elapsed().as_secs_f64() * 1000.0;

    // verify() outside timed region — must pass (already checked above)

    // ── 6. EMIT ───────────────────────────────────────────────────────────
    Ok(PulseArtifact {
        schema_version: "tscp-pulse-artifact-v1".into(),
        artifact_id: uuid_v4(),
        created_at: current_timestamp(),
        provenance: Provenance {
            manifest_sha256: manifest_sha256.into(),
            repo_commit: repo_commit.into(),
            binary_digest: binary_digest.into(),
            proof_digest,
            transcript_digest,
        },
        verification: Verification {
            status: "pass".into(),
            verifier_unchanged: true,
            correctness_gate_passed: true,
            fiat_shamir_rounds: 12,  // TODO: get from transcript
            public_inputs_hash: String::new(),  // TODO: sha256_hex(&public_inputs_bytes)
        },
        benchmark: Benchmark {
            layer: "end_to_end".into(),
            trace_size,
            integrity: IntegrityMeasurement {
                proving_ms: integrity_proving_ms,
                verification_ms,
                total_pipeline_ms: integrity_proving_ms + verification_ms,
            },
            performance: PerformanceMeasurement {
                proving_ms: performance_proving_ms,
                transcript_generation_ms: 0.0,  // TODO: from PhaseTimer
                sumcheck_ms: 0.0,               // TODO: from PhaseTimer
                fri_ms: None,                   // TODO: from PhaseTimer
                verification_outside_timed_region: true,  // invariant — always true
            },
            statistics: None,  // TODO: populate from Criterion repetitions
        },
        telemetry: Telemetry {
            peak_rss_kb: 0,            // TODO: from /proc/self/status
            proof_size_bytes: proof_bytes.len(),
            transcript_size_bytes: transcript_bytes.len(),
            binary_size_bytes: 0,      // TODO: stat(current_exe())
        },
    })
}

/// Generate a UUIDv4 string.
fn uuid_v4() -> String {
    // TODO: use `uuid = { version = "1", features = ["v4"] }`
    // uuid::Uuid::new_v4().to_string()
    "00000000-0000-4000-8000-000000000000".to_string()  // placeholder
}
```

---

## Compatibility Guarantees

### v1 consumer guarantees (TscpAdapter)

- Adapter MUST accept any artifact where `schema_version = "tscp-pulse-artifact-v1"` and the document validates against the schema.
- Adapter MUST reject artifacts where `verification.correctness_gate_passed = false` — these must not be rendered as valid results.
- Adapter MUST reject artifacts where `benchmark.performance.verification_outside_timed_region ≠ true`.
- Adapter MUST surface `verification.status` prominently — a `"fail"` artifact must not appear as a passing result.

### v1 producer guarantees (reference emitter)

- Emitter MUST emit `created_at` as real UTC time, never hardcoded.
- Emitter MUST populate the full provenance chain (all five fields required).
- Emitter MUST run the correctness gate before accepting any timing.
- Emitter MUST emit `verification_outside_timed_region: true` for performance measurements.
- Emitter MUST NOT conflate layer claims — one artifact, one layer.

### Breaking vs non-breaking changes

| Change | Classification |
|---|---|
| Add optional field to `telemetry` | Non-breaking |
| Add optional field to `benchmark.statistics` | Non-breaking |
| Add new enum value to `verification.status` | Breaking |
| Add required field anywhere | Breaking |
| Change `schema_version` value | Breaking |
| Remove any field | Breaking |

Breaking changes require a new schema version (`tscp-pulse-artifact-v2`) and a new adapter
acceptance path. Old artifacts remain valid under the version they were emitted at.

---

## CPU vs GPU Comparison Template

When publishing speedup claims, the comparison table must reference the artifact provenance:

```
Comparison baseline:
  manifest_sha256: <hash>
  repo_commit: <40-char sha>
  layer: end_to_end
  correctness: all correctness_gate_passed=true
  verifier_unchanged: true (both CPU and GPU rows)

Results (median, N reps):

| trace_size | cpu_proving_ms | gpu_proving_ms | speedup | proof_digest_match | transcript_match |
|------------|---------------|---------------|---------|-------------------|-----------------|
| 1024       |               |               |         |                   |                 |
| 4096       |               |               |         |                   |                 |
| 16384      |               |               |         |                   |                 |
```

A speedup claim is valid only when:
- Both rows reference the same `manifest_sha256`
- `correctness_gate_passed = true` for both
- `proof_digest_match = true` (GPU produces the same proof)
- `transcript_digest_match = true`
- `verifier_unchanged = true` on both rows

---

## Phase 3 Closure

```
Phase 3 — Verifier Handoff Contract
Status: SEALED

Producer contract:
  ✓ JSON Schema v1  (schemas/tscp-pulse-artifact-v1.schema.json)
  ✓ Output specification  (this document)
  ✓ Golden artifact fixture  (fixtures/phase3/golden_artifact_v1.json)
  ✓ Reference emitter stub  (embedded above)
  ✓ Compatibility guarantees

Consumer contract:
  ✓ TscpAdapter frozen
  ✓ Dashboard boundary frozen
  ✓ PulseViewModel frozen

Acceptance path:
  Emitter → Golden Artifact → Schema → Adapter

Next execution boundary:
  Verifier producer implementation + contract test suite
```

The next engineering work is the verifier emitter and its contract tests — not more UI changes.
Additional dashboard refinement at this point would polish the consumer side while the producer
seam is still empty. The leverage is in making artifact generation impossible to misuse.

---

## Canonical Encoding Rule

**This section defines what the `artifact_digest` covers and how cross-language verifiers must replicate it.**

### The digest covers the exact emitted byte stream

The `artifact_digest` in `EmitResult` is the SHA-256 of the serialized JSON bytes, not a
hash of the Rust struct's in-memory representation. This is the serialization-before-digest
invariant.

```
artifact_digest = SHA-256(emitted_json_bytes)
```

A verifier receiving the artifact file recomputes this as:

```python
import hashlib
digest = hashlib.sha256(open("artifact.json", "rb").read()).hexdigest()
assert digest == artifact["provenance"]["artifact_digest"]
```

### Canonical encoding is the emitter's byte stream

The reference emitter uses `serde_json::to_string()` which produces:

- **No spaces** around `:` or `,` separators
- **Field order:** struct definition order (serde preserves this in Rust)
- **Float encoding:** Grisu3 algorithm (serde_json default)
- **Null:** `null` for `Option::None`
- **Unicode:** unescaped (UTF-8 passthrough for printable chars)

A cross-language verifier that needs to reproduce the digest must replicate this exactly.
The practical rule: **do not re-serialize to verify; re-read the emitted bytes.**

The safe verification path is:

```
Read artifact.json bytes as-is
    │
    ▼
SHA-256(those bytes) == provenance.artifact_digest?
    │
    ▼
If yes: byte stream is authentic
Parse JSON to check field values
```

The unsafe path (do not do this):

```
Parse artifact.json into object
    │
    ▼
Re-serialize to string           ← different encoder = different bytes
    │
    ▼
SHA-256(re-serialized)          ← will not match
```

### Float precision across language boundaries

`serde_json` (Rust) and standard JSON encoders in Go, Python, and JavaScript use different
floating-point rendering algorithms (Grisu3, Ryu, dtoa). For most values they produce
identical output; for edge cases they diverge.

**v1 mitigation:** Since the digest covers the emitted byte stream rather than re-encoded
content, float divergence only matters if a future system re-emits artifacts rather than
forwarding the original bytes. As long as verifiers read-then-hash (not parse-then-reserialize),
this is not a problem.

**Future note:** If artifacts ever cross organizational boundaries where re-serialization is
unavoidable, adopt a fixed-precision encoding rule (e.g. all floats as 6 decimal places,
or switch timing values to integer microseconds). That would be a v2 schema change.

### Producer requirement (added)

Emitter MUST use `serde_json::to_string()` (compact, no pretty-print) for the canonical
artifact bytes before computing the digest. `serde_json::to_string_pretty()` produces
different bytes and a different digest — it must not be used for digest computation even if
used for display.
