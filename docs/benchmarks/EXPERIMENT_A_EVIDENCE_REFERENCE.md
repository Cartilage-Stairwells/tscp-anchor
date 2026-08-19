# Experiment A — Cross-Repository Evidence Reference: experiment_a_afea62bc

> This document registers and cross-references the Experiment A evidence baseline
> identified as **experiment_a_afea62bc**. The evidence artifacts reside in a
> separate repository; this document does not duplicate or modify them.

---

## What experiment_a_afea62bc Is

A measured AVX-512 kernel-level speedup evaluation of a hand-written 32-bit/16-wide
DIF butterfly implementation against a controlled scalar baseline, captured under
an explicitly constrained build configuration and recorded with cryptographic
provenance in the source-owning repository.

## What experiment_a_afea62bc Is Not

- It is **not** an end-to-end prover speedup measurement.
- It is **not** comparable to firebird_74c6e5f (different scope, different hardware, different methodology).
- It is **not** a universal AVX-512 performance claim across all CPUs.
- It is **not** an energy efficiency measurement (energy measurement was aborted).
- It is **not** a prover integration validation.
- It is **not** a formal verification result (formal proofs are in tscp-anchor, not in the source repo).
- It is **not** representative of performance on Intel hardware (only AMD Zen 5 tested).

---

## Evidence Location

| Field | Value |
|---|---|
| Evidence identity | `experiment_a_afea62bc` |
| Source repository | `Cartilage-Stairwells/zksha-rx-reviewer-access` |
| Source commit | `46c09eba5d2f99299ec9cb6ded0ec9ef984e6495` |
| Evidence branch | `evidence/experiment-a` |
| Evidence commit | `8df0c247cc503b751ee91245bb952b6994aaf2e6` |
| Evidence directory | `evidence/experiment_a/` (14 files) |
| Layer-1 artifacts | 12 files, SHA-256 pinned via SHA256SUMS |
| Layer-2 attestation | `PROVENANCE_RECORD.md` (in evidence directory) |
| Evidence manifest SHA-256 | `afea62bc98276eebefd1fc18ecd9c6fad455630546888f19fd361285b251a9c3` |

This document is a cross-reference. The evidence artifacts are NOT duplicated here.
To inspect the evidence, checkout the `evidence/experiment-a` branch in
`zksha-rx-reviewer-access` and navigate to `evidence/experiment_a/`.

---

## Source Identity

The measured source is `zksha-rx-reviewer-access` at commit `46c09eb` (main branch).
Source files are byte-identical to tagged commit `01db486d` / `review-v0.1.13`
(4 intervening commits are documentation-only, verified by SHA-256 match).

Key source files (SHA-256 verified against remote):
- `src/avx512_butterfly_32bit.rs` — hand-written AVX-512 DIF butterfly kernel
- `src/ntt.rs` — NTT implementation
- `benches/three_lane_bench.rs` — three-lane benchmark harness

---

## Hardware / Environment

| Field | Value |
|---|---|
| CPU | AuthenticAMD, family 191, model 2 (Zen 5 class, znver5) |
| AVX-512 features | F, DQ, CD, BW, VL, VBMI, VBMI2, VNNI, BITALG, VPOPCNTDQ |
| Cores | 4 |
| Host | modal (gvisor-based container sandbox) |
| Kernel | 4.19.0-gvisor |
| Rust | 1.97.1 (8bab26f4f 2026-07-14) |
| RUSTFLAGS | `-C target-cpu=x86-64` (prevents auto-vectorization in scalar baseline) |

---

## Methodology

| Field | Value |
|---|---|
| Benchmark framework | Criterion 0.8.2 (pinned in Cargo.lock) |
| Harness | `benches/three_lane_bench.rs` (scalar, AVX2, AVX-512) |
| Targeted run | 30 samples, 1s warmup, 3s measurement |
| Sizes | 2^8 through 2^20 (13 sizes) |
| Correctness gate | PASS (all three lanes produce identical output) |
| Scalar baseline | `target-cpu=x86-64` (SSE only, no auto-vectorization) |
| AVX-512 lane | Hand-written intrinsics (16-wide, zmm) |
| Butterfly type | DIF (Decimation-in-Frequency) |
| Field | BabyBear (P = 0x78000001) |
| Representation | Montgomery R = 2^32, u32 elements |

### Methodology Distinction

Experiment A uses `target-cpu=x86-64` to isolate the hand-written AVX-512 kernel's
contribution by eliminating compiler auto-vectorization from the scalar baseline.

The source repository's CANONICAL_RESULTS.md uses `target-cpu=native` which allows
compiler auto-vectorization, producing a faster scalar baseline and lower speedup
ratios (1.265x-1.276x geometric mean).

Both measurements are valid under their respective methodologies:
- Experiment A: isolates AVX-512 kernel contribution (7.23x peak, 4.42x geometric mean)
- Canonical: measures realistic improvement including auto-vectorization (1.265x-1.276x)

firebird_74c6e5f (in this repository) uses `target-cpu=icelake-server` and measures
the end-to-end proving pipeline on Intel Ice Lake-SP — a different scope, different
hardware, and different methodology from experiment_a_afea62bc.

---

## Binary Verification

| Lane | zmm count | ymm count | xmm count | ISA confirmed |
|---|---|---|---|---|
| Scalar | 0 | 0 | 96 | SSE only |
| AVX2 | 0 | 31 | 40 | AVX2 |
| AVX-512 | 83 | 19 | 13 | AVX-512 |

---

## Results

### Headline (2^20, 524,288 butterfly operations)

| Lane | Median time | Speedup vs scalar |
|---|---|---|
| Scalar | 2.0164 ms | 1.00x |
| AVX2 | 2.0454 ms | 0.99x |
| AVX-512 | 0.2787 ms | **7.23x** |

### Geometric Mean (2^8 through 2^20)

| Comparison | Geometric mean |
|---|---|
| AVX-512 vs scalar | **4.42x** |
| AVX2 vs scalar | 1.00x |

---

## Claims Supported (per CLAIM_LANGUAGE_POLICY.md)

- **measured**: AVX-512 kernel speedup of 7.23x peak at 2^20, 4.42x geometric mean,
  under the documented benchmark configuration (target-cpu=x86-64, Criterion 0.8.2,
  AMD Zen 5).
- **verified**: Correctness gate PASS, ISA identity (scalar=0 zmm, AVX2=31 ymm,
  AVX-512=83 zmm), SHA-256 manifest integrity, source byte-identity with remote
  commit 46c09eb.

## Claims NOT Supported

- **not claimed**: End-to-end prover acceleration
- **not claimed**: Energy efficiency or energy savings
- **not claimed**: Universal AVX-512 performance
- **not claimed**: Formal verification of the SIMD implementation
- **not claimed**: Prover integration
- **not claimed**: Performance on Intel hardware

---

## Canonical Claim Language

> Experiment A measured a 7.23x peak speedup at 2^20, with a 4.42x geometric mean
> across the specified benchmark range, for the tested AVX-512 butterfly
> implementation against the specified scalar baseline on AMD Zen 5. Correctness
> and ISA identity were independently checked. Package energy was not measured on
> the execution host, and the experiment does not establish prover integration.

### Prohibited Substitutions

The following must NOT be substituted for the canonical claim:
- "7.23x faster prover" (not established)
- "7.23x system acceleration" (not established)
- "energy efficient" (not measured)
- "formally verified AVX-512" (formal proofs cover Montgomery arithmetic, not SIMD kernel)
- "integrated AVX-512 prover" (not established)
- "global energy reduction" (not measured)

---

## Interpretation Boundary

```
LAYER 1 — EVIDENCE          evidence/experiment_a/ (in zksha-rx-reviewer-access)  (immutable)
LAYER 2 — ATTESTATION       PROVENANCE_RECORD.md + evidence commit 8df0c247       (describes evidence)
LAYER 2 — CROSS-REFERENCE   this document (in tscp-anchor)                        (registers identity)
LAYER 3 — INTERPRETATION    future analysis, comparisons                          (evolves freely)
```

---

## Relationship to Other Evidence Baselines

| Baseline | Scope | Hardware | Method | Result | Relationship |
|---|---|---|---|---|---|
| `firebird_74c6e5f` | End-to-end prover | Intel Ice Lake-SP | target-cpu=icelake-server | (sealed, see FIREBIRD_AVX512_BASELINE.md) | Different scope + hardware |
| `experiment_a_afea62bc` | Kernel butterfly | AMD Zen 5 | target-cpu=x86-64 | 7.23x peak, 4.42x GM | This baseline |

These are complementary, not competitive. firebird_74c6e5f measures the proving
pipeline; experiment_a_afea62bc measures the butterfly kernel in isolation.

---

## Audit Trail

1. Evidence artifacts created and sealed in zksha-rx-reviewer-access (Mutation 1)
2. Evidence commit 8df0c247 created on evidence/experiment-a branch, parent 46c09eb
3. 14 files added (12 Layer-1 + SHA256SUMS + PROVENANCE_RECORD.md), 0 modified
4. Independent verification (Mutation 2): 10/10 gates passed, all hashes verified
5. Cross-reference registered in tscp-anchor BENCHMARK_PROVENANCE.md (this document)
6. tscp-anchor master untouched; this document is on audit/2026-08-19-evidence-refresh branch

All 6 checks: **PASSED**.
