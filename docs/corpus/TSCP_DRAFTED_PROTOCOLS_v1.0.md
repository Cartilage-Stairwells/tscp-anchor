# TSCP Drafted Protocols v1.0

**Document:** TSCP_DRAFTED_PROTOCOLS_v1.0  
**Date:** 2026-10-07  
**Status:** DRAFTED specifications (not frozen, not established, not promoted)  
**Depends on:** TSCP_NAMED_ROLES_v1.1, TSCP_ARCHITECTURE_AND_STARK_SNARK_v1.0  
**Epistemic rule:** claim_scope ⊆ evidence_scope. These texts define intended interfaces and failure behavior for documentation and implementation planning. They do not execute promotions, admit evidence, or alter machine artifacts.

---

## Document Control

| Item | Value |
|------|--------|
| VoxCanon severe-exception protocol | DRAFTED |
| VexScale observational telemetry token | DRAFTED |
| Machine-readable schema adoption | Not required by this document |
| Named-role strings in hash preimages | Prohibited |
| Authority conferred by these protocols | None |

---

# Part A — VoxCanon Severe-Exception Protocol

## A.1 Purpose

Define how **VoxCanon (Conformance)** behaves when a severe policy exception is detected: misalignment between a frozen or assayed object and the mandates / relationship constraints owned by **VoxArchon (Layer 0)**.

This protocol is an alignment gate. It is not a performance instrument and not a custody principal.

## A.2 Scope

**In scope**

- Detection of severe policy exceptions
- Classification of exception severity
- Emission of structured exception records
- Fail-closed handling rules
- Explicit non-mutation of upstream semantic content

**Out of scope**

- Changing VoxArchon vocabulary or relationship types
- Mutating VexVector canonical encodings
- Automatic promotion, demotion, or unfreeze of any layer
- On-chain transaction construction
- Cryptographic proof generation (proofs may be *referenced* as evidence; this protocol does not produce them)

## A.3 Definitions (DRAFTED)

| Term | Meaning |
|------|---------|
| **Policy exception** | Observed divergence between an object under check and a declared VoxArchon constraint (relationship type, completeness, semantic_equal boundary, or plane-preservation rule) |
| **Severe exception** | Exception that, if ignored, would allow custody→authority collapse, plane violation, or admission of an object that fails the declared alignment predicate |
| **Exception record** | Structured observational report emitted by VoxCanon; carries no authority |
| **Fail-closed** | Unrecognized, incomplete, or severely exceptional input is rejected; processing does not continue under a permissive interpretation |
| **Upstream** | VoxArchon emission and VexProbe assay outputs; also VexVector frozen encodings presented for alignment |

## A.4 Severity Classes (DRAFTED)

| Class | Label | Typical triggers | Default disposition |
|-------|-------|------------------|---------------------|
| S0 | Informational | Non-normative annotation mismatch | Record; continue |
| S1 | Recoverable | Missing optional attestation field with documented default | Record; continue only if defaults are explicit in the governing contract |
| S2 | Blocking | Required relationship missing; semantic_equal failure; incomplete object | Reject; emit exception record |
| S3 | Severe / constitutional | Custody presented as authority; plane-crossing edge; mutation of frozen semantic content detected | Reject; emit exception record; escalate to human/governance surface; do not auto-remediate |

Only **S2** and **S3** are “severe” for the purpose of this protocol title. S3 is the class that must never be silently repaired.

## A.5 Detection Obligations

VoxCanon SHALL evaluate, against the declared VoxArchon constraints applicable to the object class:

1. Presence of required relationship edges (`responds_to`, `depends_on`, `refines`, `supersedes`, `derived_from` as applicable).
2. Satisfaction of `semantic_equal` / `complete` predicates where those predicates are defined for the object class.
3. Plane-preservation: no edge or attribute that would treat custody as authority.
4. Non-mutation: the semantic payload presented for alignment is byte-identical (or canonically identical under the governing serialization contract) to the payload emitted/assayed upstream, except for explicitly permitted provenance wrappers.
5. Presence of required attestation blocks when the governing contract requires them (e.g., VexProbe test-compliance attestation before freeze acceptance).

Exact predicate libraries are not fixed by this draft; they must be bound to versioned VoxArchon vocabulary artifacts.

## A.6 Fail-Closed Handling Rules

| Condition | Required action |
|-----------|-----------------|
| S2 or S3 detected | Halt alignment success path |
| Exception record emission fails | Treat as S3 operational failure; do not report alignment success |
| Upstream semantic content would need mutation to “pass” | Forbidden; remain failed |
| Ambiguous severity | Classify at least S2; do not default to continue |
| Missing governing vocabulary version | S2 or S3; cannot align against an unbound rule set |

**Prohibited actions**

- Auto-editing semantic content to satisfy a rule
- Downgrading S3 to S1 without governance record
- Treating a valid STARK/SNARK proof as sufficient to override an S3 plane violation
- Inserting named-role strings into canonical encodings as a remediation

## A.7 Exception Record Format (DRAFTED logical fields)

Logical fields only. Serialization may follow P0-CANONICAL-SERIALIZATION when bound.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `record_id` | opaque id | yes | Unique per emission |
| `timestamp` | UTC instant | yes | Observation time |
| `voxcanon_version` | version string | yes | Protocol version of this checker |
| `vocabulary_ref` | artifact ref | yes | VoxArchon vocabulary / rule set version |
| `object_ref` | artifact ref | yes | Object under alignment check |
| `severity` | enum S0–S3 | yes | Severity class |
| `code` | stable string | yes | Machine-stable exception code |
| `message` | string | yes | Human-readable summary |
| `violated_constraints` | list | yes | Identifiers of failed predicates |
| `evidence_refs` | list | no | Hashes / proof ids / test receipts (references only) |
| `mutation_attempted` | boolean | yes | Must be false under correct operation |
| `disposition` | enum | yes | `recorded` \| `rejected` \| `escalated` |
| `observer` | identity ref | no | Optional operator/service identity; not authority |

**Invariant:** The exception record is observational. `disposition: escalated` notifies a governance surface; it does not itself promote or unfreeze any layer.

## A.8 Interaction with Other Layers

```
VoxArchon  --(meaning)-->  VexProbe  --(assay)-->  VexVector  --(frozen)-->  VoxCanon
                                                                      │
                                                                      ├─ pass → alignment attestation (separate draft)
                                                                      └─ S2/S3 → exception record (this protocol)
                                                                               │
                                                                               └─ observational feedback only
                                                                                  (no semantic mutation upstream)
```

VexScale may record that an exception occurred as a metric event; it does not decide alignment.

## A.9 Status and Elevation Path

This Part A is **DRAFTED**. Elevation toward ESTABLISHED requires:

1. Binding to a versioned VoxArchon vocabulary artifact  
2. Concrete exception code registry  
3. Implementation tests demonstrating fail-closed behavior for S2/S3 fixtures  
4. Explicit non-mutation tests  

Elevation to FROZEN requires governance adoption and cryptographic binding rules consistent with the naming charter (structural dependency freeze) — not defined as executed by this document.

---

# Part B — VexScale Observational Telemetry Token

## B.1 Purpose

Define a minimal, observational telemetry token for **VexScale (Benchmark Plane)** so that performance and operational measurements can be recorded without implying admission, custody, or promotion.

## B.2 Scope

**In scope**

- Logical token fields for observational metrics
- Environment binding
- Hash linking to measured artifacts
- Explicit “observational only” semantics

**Out of scope**

- Defining which benchmarks are authoritative (see registered baselines in PROJECT_FACTS / BENCHMARK_PROVENANCE when present)
- Claiming end-to-end prover acceleration
- Replacing criterion or other measurement methodologies
- On-chain settlement of metrics

## B.3 Design Principles

1. **Observational only** — the token reports; it does not admit.  
2. **Environment-bound** — metrics without hardware / software fingerprint are incomplete.  
3. **Artifact-linked** — metrics should reference the measured object by hash when available.  
4. **Non-authoritative** — presence of a token never upgrades claim status from measured → verified → production.  
5. **Compatible with registered baselines** — token may cite an `evidence_identity` such as those in PROJECT_FACTS without redefining them.

## B.4 Telemetry Token — Logical Schema (DRAFTED)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token_id` | opaque id | yes | Unique emission id |
| `schema_version` | string | yes | e.g. `vexscale-telemetry-1` |
| `timestamp` | UTC instant | yes | Time of measurement or aggregation |
| `metric_name` | string | yes | Stable metric identifier |
| `metric_value` | number \| string | yes | Primary result |
| `unit` | string | yes | e.g. `Melem/s`, `ns/op`, `dimensionless` |
| `aggregation` | enum | no | `single` \| `mean` \| `geomean` \| `peak` \| `p50` \| `p99` |
| `sample_count` | integer | no | Number of samples if aggregated |
| `subject_hash` | hex digest | recommended | Hash of measured artifact / binary / corpus |
| `subject_ref` | artifact ref | no | Human or URI reference to subject |
| `evidence_identity` | string | no | Registered baseline id if applicable |
| `environment` | object | yes | See B.5 |
| `method_ref` | string | recommended | Methodology document or command |
| `repro_command` | string | no | Exact reproduction command when available |
| `claim_class` | enum | yes | `measured` \| `inferred` \| `projected` \| `not_a_claim` |
| `notes` | string | no | Non-normative commentary |

**Invariant:** `claim_class` must not be omitted. Inferred or projected values must not be labeled `measured`.

## B.5 Environment Object (DRAFTED)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cpu_model` | string | recommended | From environment |
| `cpu_flags` | list | recommended | e.g. presence/absence of avx512f |
| `target_cpu` | string | recommended | Compiler target (e.g. `native`, `x86-64`, `icelake-server`) |
| `rustc_version` | string | recommended | When Rust toolchain used |
| `os` | string | no | OS / kernel summary |
| `core_count` | integer | no | |
| `feature_flags` | object | no | Build features affecting the binary |
| `hostname_hash` | hex | no | Optional privacy-preserving host binding |

Metrics gathered without AVX-512 flags on hardware that cannot execute AVX-512 paths must not be presented as AVX-512 results.

## B.6 Example Metric Names (non-exhaustive, DRAFTED)

| metric_name | Intended meaning |
|-------------|------------------|
| `babybear_dit_throughput` | DIT kernel elements per second |
| `babybear_mont_mul_ns` | Montgomery multiply latency |
| `ntt_roundtrip_mismatch_count` | Correctness counter (should be 0) |
| `fri_verify_latency_ms` | FRI verify wall time |
| `sumcheck_verify_latency_ms` | Sumcheck verify wall time |
| `voxcanon_exception_count` | Count of VoxCanon S2/S3 records in window |
| `alignment_pass_rate` | Passes / attempts (observational) |

## B.7 Semantic Restrictions

| Restriction | Rule |
|-------------|------|
| No admission | Token does not satisfy an admissibility contract by itself |
| No promotion | Token does not move a layer DRAFTED → ESTABLISHED → FROZEN |
| No authority | Token is not an FCO and has no Authority field that can be non-zero |
| Honesty of class | `claim_class: measured` requires actual execution on the stated environment |
| Kernel ≠ prover | Kernel throughput tokens must not be retitled as end-to-end prover speedup |

## B.8 Relationship to Exception Protocol

When VoxCanon emits an S2/S3 exception record, VexScale MAY emit a telemetry token with `metric_name = voxcanon_exception_count` (or a per-code metric) and `claim_class = measured`. The telemetry token does not replace the exception record and does not clear the exception.

## B.9 Status and Elevation Path

This Part B is **DRAFTED**. Elevation toward ESTABLISHED requires:

1. Binding to a concrete serialization (JSON / CBOR / canonical form per P0 serialization when applicable)  
2. Alignment with any sealed `BENCHMARK_PROVENANCE` / evidence_identity registry  
3. At least one implemented emitter and one consumer that treat `claim_class` correctly  
4. Explicit rejection tests for mislabeled projected data as measured  

---

## Part C — Cross-Protocol Invariants

1. Both protocols are **observational** with respect to semantic content.  
2. Neither protocol grants custody or authority.  
3. Neither protocol inserts named-role strings into hash preimages.  
4. Fail-closed (VoxCanon) and honest claim_class (VexScale) are complementary: alignment failures are rejected; measurements are not laundered into stronger claims.  
5. STARK/SNARK verification success is evidence that may appear in `evidence_refs` or `subject_hash` linkage; it does not override S3 constitutional failures.

---

## Part D — Evidence Boundary

| Item | Label |
|------|-------|
| Existence of this drafted protocol text | OBSERVED (this file) |
| Implementation of either protocol in tscp-anchor | NOT_FOUND_IN_INSPECTED_SCOPE as of this writing |
| Compatibility with PROJECT_FACTS non-claims | DECLARED design intent |
| Elevation to ESTABLISHED or FROZEN | Not executed |

---

## Part E — Recommended Immediate Follow-ups

1. Produce JSON Schema drafts for `ExceptionRecord` and `VexScaleTelemetryToken`.  
2. Add fixture vectors: one S3 plane-violation case that must reject; one measured kernel token with full environment binding.  
3. Cross-link exception codes to VoxArchon vocabulary version once that registry is explicit.  
4. Keep SNARK-wrapper deployment (if any) outside these protocols as a separate deployment path document.

---

*End of TSCP_DRAFTED_PROTOCOLS_v1.0*
