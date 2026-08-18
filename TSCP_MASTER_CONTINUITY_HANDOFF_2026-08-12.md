# TSCP PHASE 3 — MASTER CONTINUITY HANDOFF
## Evidence, Benchmarking, AVX-512/NTT Verification, and Next Experimental Sequence

**Date:** 2026-08-12
**Purpose:** Complete continuity handoff for a new ChatGPT agent/account/operator
**Project:** TSCP — Transparent Succinct Computational Proof
**Phase:** Phase 3 — benchmark/evidence-based proving-pipeline verification
**Current frozen evidence identity:** `firebird_74c6e5f`

---

## 0. READ THIS FIRST

This document is the authoritative working continuity handoff, not a replacement for the Git repository or immutable evidence.

The central rule is:

> **Do not modify `firebird_74c6e5f`. It is frozen evidence.**

The project uses three epistemic layers:

```
LAYER 1 — EVIDENCE
  Immutable benchmark artifacts and cryptographic hashes.
        ↓
LAYER 2 — ATTESTATION
  Git commits, tags, releases, provenance, signatures, bookkeeping.
        ↓
LAYER 3 — INTERPRETATION
  Human/agent analysis of what the evidence means.
  Revisable without changing Layers 1 or 2.
```

A new agent should not attempt to make the frozen benchmark answer a question that it did not measure.

The frozen baseline answers:

> What does this exact captured Ice Lake-SP FRI proving path actually do?

It does not answer:

> Does the AVX-512 NTT/butterfly make the prover faster?

That question requires a different experiment.

---

## 1. PROJECT

TSCP is a Rust zero-knowledge proving system using FRI over the BabyBear prime field:

```
p = 2^31 - 2^27 + 1
```

The relevant proving infrastructure uses Plonky3 and Poseidon2-based Merkle commitments.

Three repositories exist:

| Repository | Role |
|---|---|
| `tscp-anchor` | Principal active repository for Phase 3 |
| `avx512-butterfly` | AVX-512 kernel maintenance |
| `tscp-pl-phase1` | Planning/specs |

GitHub organization: `Cartilage-Stairwells`

- https://github.com/Cartilage-Stairwells/tscp-anchor.git
- https://github.com/Cartilage-Stairwells/avx512-butterfly.git
- https://github.com/Cartilage-Stairwells/tscp-pl-phase1.git

---

## 2. CURRENT PROJECT QUESTION

The broader project question is whether verified/implemented AVX-512 field arithmetic and NTT/butterfly machinery can provide useful performance improvements to the proving system.

That question has now been decomposed into distinct empirical questions.

**Question A — Captured prover behavior**

> What computation does the current benchmark actually execute?

Answered by `firebird_74c6e5f`.

**Question B — Kernel performance**

> Does the AVX-512 butterfly/NTT actually outperform scalar and AVX2 implementations when directly benchmarked under controlled conditions?

Not yet answered. This is **Experiment A**.

**Question C — Integrated prover performance**

> If the optimized kernel is inserted into a proving path that actually invokes it, does it improve end-to-end proving performance?

Not yet answered. This is **Experiment B**.

These three questions must remain separate.

---

## 3. THREE-LAYER EVIDENCE ARCHITECTURE

### Layer 1 — Evidence

Immutable benchmark output. Includes: measured timings, raw Criterion data, environment capture, evidence manifest, hashes, benchmark result files, evidence bundle.

Never modify an existing evidence identity. If something is wrong, create a new capture with a new identity.

### Layer 2 — Attestation

Describes and authenticates Layer 1. Includes: Git commits, tags, GitHub release, commit provenance, GPG signatures, evidence-to-commit mappings, attestation documents.

Layer 2 bookkeeping can be corrected without altering Layer 1.

### Layer 3 — Interpretation

Analytical interpretation of the evidence. Intentionally mutable. Current: `interpretation_report_firebird_74c6e5f_v2.md` (working/local/not public).

---

## 4. FROZEN EVIDENCE BASELINE

| Field | Value |
|---|---|
| **Identity** | `firebird_74c6e5f` |
| **Status** | FROZEN — DO NOT MODIFY |
| **Record SHA-256** | `0d9c2c8ef46409c904ede8715f83fa6b20a1e48fd22c0841f9b7154697967e1c` |
| **Bundle SHA-256** | `7feb104a699429822b65edee3b90b76672630b1129167f01f139f0e993780a59` |
| **Manifest SHA-256** | `a6b80d0d13fc8391c246910c90892ce81dcd325395ceb8f8487f162ea597ea44` |
| **Audit** | 8/8 checks passed |

---

## 5. HARDWARE / ENVIRONMENT

| Field | Value |
|---|---|
| **CPU** | Intel Ice Lake-SP |
| **CPUID** | family = 6, model = 106, model = 0x6A |
| **Target** | `x86_64-unknown-linux-gnu` |
| **target-cpu** | `icelake-server` |
| **RUSTFLAGS** | `-C target-cpu=icelake-server` |
| **Rust** | 1.97.1 |
| **Host** | Modal.com sandbox |
| **OS** | Debian 12 |
| **Kernel** | 4.19.0-gvisor |

Captured AVX-512 capabilities: F, DQ, CD, BW, VL, VBMI, VBMI2, VNNI, BITALG, VPOPCNTDQ

> Evidence captures use explicit `target-cpu` values. Never use `native` for canonical evidence. `native` is exploration-only.

---

## 6. GIT STATE OF FROZEN BASELINE

| Field | Value |
|---|---|
| **Repository** | `tscp-anchor` |
| **Branch** | `master` |
| **HEAD** | `653ce0612eef479aa910d2377413403ffb72cf0a` |
| **Tag** | `benchmark/firebird_74c6e5f` |
| **GitHub release** | https://github.com/Cartilage-Stairwells/tscp-anchor/releases/tag/benchmark/firebird_74c6e5f |
| **Working tree** | clean |

---

## 7. FROZEN COMMIT CHAIN

```
16c55a24  docs: add verifier output specification v1 (Phase 3 contract)
cc31d21f  docs(verifier-spec): canonical encoding rule
3f435d89  feat(verifier): emitter core (artifact, provenance, timing, serialize-before-digest)
a04b73f9  feat(verifier): oracle bridge + PhaseTimer instrumentation
97f12bc0  feat(verifier): prove_and_emit pipeline + PhaseTimer merge  [CODE UNDER TEST]
ae51e194  bench(verifier): Criterion evidence harness
6b5839dd  chore: gitignore generated benchmark_results.json
e81efb78  docs: attest CPU AVX-512 evidence baseline firebird_74c6e5f
653ce061  docs: correct attestation hash mapping to final GitHub-verified commits
```

Original → GitHub-verified commit mappings:

| Original | GitHub-verified |
|---|---|
| `da57bdd9` | `16c55a24` |
| `8c766108` | `cc31d21f` |
| `bb121851` | `3f435d89` |
| `a3ae814c` | `a04b73f9` |
| `57b695c0` | `97f12bc0` |
| `d4c63c38` | `ae51e194` |
| `52f4ef3c` | `6b5839dd` |

---

## 8. GPG / SIGNING CONTEXT

| Field | Value |
|---|---|
| **Key** | `4B7F7F7E8543997A` |
| **Algorithm** | RSA2048 |
| **Expiration** | 2027-08-12 |
| **Identity** | Sean Christopher Southwick `<schlagetorren@gmail.com>` |

```bash
git config --global user.name "Sean Christopher Southwick"
git config --global user.email "schlagetorren@gmail.com"
git config --global user.signingkey 4B7F7F7E8543997A
git config --global commit.gpgsign true
```

---

## 9. FROZEN PERFORMANCE RESULTS

| Trace size | Median | Std dev |
|---|---|---|
| 2^8 = 256 | 0.3955 ms | 0.0009 ms |
| 2^10 = 1024 | 1.0325 ms | 0.0104 ms |
| 2^12 = 4096 | 3.5840 ms | 0.0517 ms |
| 2^14 = 16384 | 13.5657 ms | 0.1908 ms |

Criterion `estimates.json` is the canonical timing source. PhaseTimer is instrumentation/provenance, not the canonical performance reference.

---

## 10. PHASE TIMER RESULTS

| Trace size | FRI | Transcript | FRI % | Verification |
|---|---|---|---|---|
| 2^8 | 0.373 ms | 0.003 ms | 99.13% | 2.77 ms |
| 2^10 | 1.012 ms | 0.001 ms | 99.91% | 4.08 ms |
| 2^12 | 3.533 ms | 0.001 ms | 99.97% | 5.54 ms |
| 2^14 | 13.593 ms | 0.001 ms | 99.99% | 7.23 ms |

> Verification occurs outside the timed proving region.

---

## 11. OTHER FROZEN METRICS

- **num_queries:** 20
- **Fiat-Shamir rounds:** `log2(trace_size) + 1`
- **Proof sizes:** 2148 bytes (2^8) through 3493 bytes (2^14)
- **Tests:** 12/12
- **Warnings:** 0

---

## 12. THE MOST IMPORTANT DISCOVERY

**The frozen benchmark does not exercise the AVX-512 NTT/butterfly backend.**

The captured path:

```
prove_instrumented_internal
        |
        v
    fri_prove
        |
        +--> MerkleTree::build     [Poseidon2 hashing]
        +--> fri_fold_step         [element-wise field arithmetic]
        +--> fold_domain           [element-wise field squaring]
        +--> challenger.observe/sample
        +--> fri_query_round       [Merkle openings]
```

The NTT/butterfly backend is not reached. `Radix2Interpolator` exists but is only used in `#[cfg(test)]` modules.

> The captured proving path does not invoke the NTT/butterfly backend. This is the central empirical finding.

---

## 13. DO NOT MISSTATE THAT FINDING

**Avoid:** "The AVX-512 butterfly is ineffective." / "AVX-512 provides no speedup." / "The NTT optimization provides zero performance benefit."

**Correct statement:**

> The captured proving path does not invoke the NTT/butterfly backend, so the AVX-512 butterfly optimization contributes no measured work to this captured path.

This is a path/integration finding, not a kernel-performance finding.

---

## 14. SCALING

| Model | Parameters | R² |
|---|---|---|
| Power law: `t = C * N^a` | a = 0.855 | 0.9948 |
| **O(n) + overhead: `t = 0.000816*N + 0.206`** | — | **0.99998** |
| O(n log n) + overhead | — | 0.9989 |
| O(n log² n) + overhead | — | 0.9964 |

> Do not say four points mathematically prove O(n). Preferred: "strongly consistent with an O(n) model plus fixed overhead over the tested range."

---

## 15–18. WORDING DISCIPLINE AND HISTORICAL DATA

**Merkle/Poseidon2:** Say "dominated by FRI, whose implementation performs substantial Poseidon2/Merkle-tree work" — not "Merkle is definitively dominant" unless component timing establishes it.

**Amdahl:** Counterfactual. Not evidence. Experiment A replaces assumptions with measurements.

**aarch64 profile:** Structural reference only. Not a direct comparison. Not an AVX2/scalar baseline.

**Exploration data (`688d3663`):** Used `target-cpu=native`. Not canonical evidence.

---

## 19–20. AVX2 TARGET WARNING AND BACKEND VERIFICATION

> **`icelake-client` is NOT AVX2-only.** Ice Lake client CPUs support AVX-512. Do not use it as AVX2 baseline.

| Target | Candidate | Verification requirement |
|---|---|---|
| AVX-512 | `icelake-server` | (established) |
| AVX2 | `haswell` | AVX-512 absent, AVX2 present in binary |
| Scalar | `x86-64` | AVX absent in binary |

Do not infer capability from target name alone — inspect the emitted binary.

---

## 21. EVIDENCE INVARIANTS

| # | Invariant |
|---|---|
| 1 | `firebird_74c6e5f` is immutable |
| 2 | New evidence gets new identity |
| 3 | Serialization before digest |
| 4 | Verification outside timed region |
| 5 | Criterion `estimates.json` is canonical timing source |
| 6 | Explicit compiler targets required — never `native` |
| 7 | Exploration is not evidence |
| 8 | CPU-vs-GPU requires separate identities |
| 9 | Backend capability verified at emitted-code level |
| 10 | Counterfactual estimates ≠ measurements |

---

## 22–23. KEY FILES AND REPOSITORY CONTENTS

### Evidence bundle (local sandbox paths from previous environment)

```
/app/benchmark_scaffold/firebird_74c6e5f.json
/app/benchmark_scaffold/evidence_firebird_74c6e5f/
  ├── benchmark_results.json
  ├── criterion_summary.txt
  ├── environment_snapshot.txt
  ├── evidence_manifest.json
  └── criterion_raw/
```

### tscp-anchor source files

| File | Role |
|---|---|
| `crates/tscp-verifier/src/artifact.rs` | Schema-aligned artifact types |
| `crates/tscp-verifier/src/provenance.rs` | SHA-256, binary digest, commit info |
| `crates/tscp-verifier/src/timing.rs` | PhaseTimer |
| `crates/tscp-verifier/src/emitter.rs` | Emitter, serialize-before-digest |
| `crates/tscp-verifier/src/oracle_bridge.rs` | FRI prove/verify integration |
| `crates/tscp-verifier/benches/evidence_baseline.rs` | Criterion evidence harness |
| `crates/oracle-layer/src/fri_query.rs` | `fri_prove` |
| `crates/oracle-layer/src/fri.rs` | `fri_fold_step` |
| `crates/oracle-layer/src/merkle.rs` | MerkleTree / Poseidon2 |
| `crates/oracle-layer/src/fft.rs` | Radix2Interpolator / NTT (not invoked by captured benchmark) |
| `crates/oracle-layer/src/sumcheck.rs` | Sumcheck (exists but not exercised) |

---

## 24. INTERPRETATION REPORT

Working document: `interpretation_report_firebird_74c6e5f_v2.md` — Layer 3, WORKING, LOCAL, NOT PUBLIC.

Corrections needed before public release:

| Replace | With |
|---|---|
| "The butterfly has zero measured contribution" | "The butterfly/NTT is not invoked by the captured proving path" |
| "The FRI prove phase scales as O(n)" | "strongly consistent with an O(n) model plus fixed overhead" |
| Unqualified Merkle dominance | "dominated by FRI, whose implementation performs substantial Poseidon2/Merkle-tree work" |

---

## 25. EXPERIMENTAL TREE

```
                    firebird_74c6e5f  (FROZEN)
                             |
             +---------------+----------------+
             |                                |
       What happened?                    What wasn't tested?
             |                                |
      Captured FRI path                  NTT / butterfly
      FRI dominates                       full prover
      linear-model fit                         |
      NTT not invoked                          |
             |                  +-------------+-------------+
             |                  |                           |
             |          Experiment A                  Experiment B
             |          Kernel performance             Integration
             |          scalar/AVX2/AVX-512           actual NTT path
             |                  |                           |
             +------------------+---------------------------+
                                |
                                v
                         Final interpretation
```

---

## 26–28. NEXT STEPS

### Step 0 — Freeze target methodology

| Target | Candidate | Requirement |
|---|---|---|
| AVX-512 | `icelake-server` | established |
| AVX2 | `haswell` | verify AVX-512 absent in binary |
| Scalar | `x86-64` | verify AVX absent in binary |

### Step 1 — AVX2 evidence (new identity)

Same hardware/workload/methodology. Different target. Verify emitted binary.

### Step 2 — Scalar evidence (new identity)

Same. Different target. Verify emitted binary.

---

## 29–31. EXPERIMENTS A AND B

### Experiment A — Kernel Benchmark

**Repository:** `avx512-butterfly`

> Does the verified AVX-512 butterfly/NTT implementation outperform scalar and AVX2 under controlled identical conditions?

Same inputs, operation, transform size, field, algorithm, methodology. Must produce its own evidence identities. Do not graft onto `firebird_74c6e5f`.

### Experiment B — Integration

> Does putting the optimized kernel into a proving path that invokes it improve end-to-end proving performance?

The full prover boundary potentially includes: polynomial evaluation → NTT/FFT → quotient computation → sumcheck → FRI. Architecture must be inspected, not assumed. Separate evidence identity.

**Sumcheck warning:** The current frozen benchmark does not exercise sumcheck. Do not describe `firebird_74c6e5f` as a full-pipeline proving benchmark.

---

## 33–35. CLAIMS AND COMPARISON DISCIPLINE

### Allowed from `firebird_74c6e5f`:

- Ice Lake-SP AVX-512 environment captured with explicit `icelake-server` target.
- Proving workload dominated by FRI.
- Timings strongly consistent with O(n) + fixed overhead.
- NTT/butterfly backend not invoked by captured path.
- Frozen benchmark does not measure AVX-512 NTT/butterfly performance.

### NOT allowed without additional evidence:

- AVX-512 makes/doesn't make prover faster.
- AVX-512 butterfly faster than AVX2/scalar.
- NTT end-to-end speedup.
- X% improvement claims.
- Full prover is O(n).
- Merkle definitively dominant.
- GPU comparisons. Full-pipeline performance.

### Comparison discipline:

Same hardware, workload, methodology, trace sizes, software revision, timing definition, explicit compiler target, verified instruction set, new evidence identity. Denominator must be explicit.

---

## 38–40. OPERATING IN A NEW ENVIRONMENT

1. Clone repositories. 2. Inspect state. 3. Verify commits/tags. 4. Inspect methodology. 5. Locate frozen evidence. 6. Confirm working tree. 7. Reconstruct experimental boundary. 8. Only then modify.

Use read-only inspection first. No destructive operations without justification and backup.

### Already established (do not reopen without new evidence):

- `firebird_74c6e5f` is frozen
- Captured hardware is Ice Lake-SP
- Canonical target is `icelake-server`
- Captured path is FRI-dominated
- NTT/butterfly not invoked
- Amdahl range is counterfactual
- aarch64 profile is structural only
- `native` exploration is not canonical
- `icelake-client` is not AVX2 baseline
- New evidence gets new identity

---

## 41. THREE EVIDENCE IDENTITIES

| Evidence | Identity | Question |
|---|---|---|
| A (captured) | `firebird_74c6e5f` | What does the captured prover execute? |
| B (kernel) | new | How fast is the butterfly/NTT itself? |
| C (integration) | new | What happens when the kernel is actually used? |

Do not collapse into one narrative.

---

## 42. FINAL INTERPRETATION

> `firebird_74c6e5f` is a valid measurement of the captured Ice Lake-SP FRI proving path. The captured path is overwhelmingly dominated by FRI, and its measured timings are strongly consistent with an O(n) model plus fixed overhead over the tested range. Call-path inspection establishes that the NTT/butterfly backend is not invoked by this proving path. Consequently, the frozen baseline does not measure AVX-512 NTT/butterfly performance or establish an end-to-end speedup attributable to that optimization.

---

## 43–44. STATUS AND IMMEDIATE NEXT ACTION

### Completed

- [x] Phase 3 contract, JSON schema, golden fixture
- [x] Verifier output specification, canonical encoding rule
- [x] Emitter core, provenance, PhaseTimer, serialize-before-digest
- [x] Oracle bridge, prove/emit pipeline
- [x] Criterion evidence harness
- [x] Ice Lake-SP evidence capture, explicit `icelake-server` target
- [x] 8/8 evidence audit, GPG-signed commits, GitHub verification
- [x] Benchmark tag, GitHub release
- [x] Layer 3 working interpretation
- [x] NTT call-path absence identified
- [x] FRI-dominated path identified
- [x] Counterfactual Amdahl quarantined

### Not yet completed

- [ ] Freeze AVX2 target methodology
- [ ] Capture AVX2 FRI evidence + verify AVX-512 absent
- [ ] Capture scalar FRI evidence + verify AVX absent
- [ ] Direct kernel benchmark (scalar vs AVX2 vs AVX-512)
- [ ] Measure actual kernel/hash cost relationship
- [ ] Integrated NTT/prover experiment
- [ ] Final public Layer 3 interpretation

### Immediate next action

1. Freeze AVX2/scalar methodology
2. Capture new evidence identities (do not touch `firebird_74c6e5f`)
3. Run Experiment A on the AVX-512 butterfly/NTT kernel
4. Replace hypothetical assumptions with empirical data
5. If warranted, run Experiment B
6. Finalize public narrative

---

## 45. ONE-SCREEN HANDOFF

**PROJECT:** TSCP Phase 3
**FROZEN BASELINE:** `firebird_74c6e5f` — IMMUTABLE
**HARDWARE:** Intel Ice Lake-SP, `target-cpu=icelake-server`

**WHAT IT MEASURES:** Captured FRI proving path

**WHAT IT FOUND:** FRI >99%. Timings fit O(n) + overhead. NTT/butterfly not invoked.

**WHAT IT DOES NOT MEASURE:** AVX-512 butterfly speedup, AVX2 comparison, scalar comparison, full prover speedup, GPU comparison.

**CRITICAL:** `icelake-client` is NOT AVX2-only.

**NEXT:** 1. Freeze AVX2/scalar methodology. 2. Capture AVX2 evidence. 3. Capture scalar evidence. 4. Experiment A: kernel benchmark. 5. Experiment B: integration. 6. Finalize interpretation.

**RULE:** Every new experiment gets a NEW evidence identity. Never modify `firebird_74c6e5f`.

**CENTRAL SENTENCE:**

> "The frozen baseline is a valid measurement of the captured FRI proving path, but it is not a measurement of the AVX-512 NTT/butterfly optimization. The captured path does not invoke the NTT/butterfly backend."

---

## Continuity Recommendation

| Artifact | Role |
|---|---|
| **Master Continuity Handoff** (this file) | Tells a fresh agent how to orient and continue |
| `TSCP_HANDOFF_v2.md` | Project-specific state/package handoff |
| `interpretation_report_firebird_74c6e5f_v2.md` | Layer 3 analytical artifact |
| `firebird_74c6e5f` | Immutable Layer 1 evidence |

The GitHub repositories provide the durable code/attestation substrate; the frozen evidence identity provides the measurement anchor; and this handoff supplies the missing conversational/experimental state.
