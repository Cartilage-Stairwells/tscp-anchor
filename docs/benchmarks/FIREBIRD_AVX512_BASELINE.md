# CPU AVX-512 Evidence Baseline — firebird_74c6e5f

> This document describes the sealed CPU AVX-512 evidence baseline identified as
> **firebird_74c6e5f**. It does not modify or supersede the evidence artifact.
> The evidence artifact is immutable; corrections require a new evidence identity.

---

## What firebird_74c6e5f Is

A reproducible CPU AVX-512 reference environment for the TSCP oracle-layer prover,
captured under an explicitly constrained build configuration and recorded with
cryptographic provenance. It is the frozen comparison reference for CPU-side
performance of the end-to-end proving pipeline.

## What firebird_74c6e5f Is Not

- It is **not** a GPU performance claim.
- It is **not** a general AVX-512 performance claim across all CPUs.
- It is **not** a universal speedup measurement.
- It is **not** a comparison with a future GPU implementation.
- It is **not** representative of every Ice Lake or AVX-512 machine.

It establishes **one** reproducible CPU AVX-512 reference environment.

---

## Evidence Identity

| Field | Value |
|---|---|
| Baseline name | `firebird_74c6e5f` |
| Record SHA-256 | `0d9c2c8ef46409c904ede8715f83fa6b20a1e48fd22c0841f9b7154697967e1c` |
| Evidence bundle SHA-256 | `7feb104a699429822b65edee3b90b76672630b1129167f01f139f0e993780a59` |
| Evidence manifest SHA-256 | `a6b80d0d13fc8391c246910c90892ce81dcd325395ceb8f8487f162ea597ea44` |
| Evidence directory | `/app/benchmark_scaffold/evidence_firebird_74c6e5f/` |
| Canonical record | `/app/benchmark_scaffold/firebird_74c6e5f.json` |

## Provenance Chain

```
16c55a24  docs: add verifier output specification v1 (Phase 3 contract)
    │
    ├─ cc31d21f  docs(verifier-spec): canonical encoding rule
    │
    ├─ 3f435d89  feat(verifier): emitter core (artifact, provenance, timing, serialize-before-digest)
    │
    ├─ a04b73f9  feat(verifier): oracle bridge + PhaseTimer instrumentation
    │
    ├─ 97f12bc0  feat(verifier): prove_and_emit pipeline + PhaseTimer merge
    │             └─ code under benchmark
    │
    └─ ae51e194  bench(verifier): Criterion evidence harness (bench file only; no verifier code changes)
                  └─ Criterion benchmark executed
                         │
                         ▼
                  firebird_74c6e5f  (evidence capture)
                         │
                         └─ 6b5839dd  clean repository state
                                │
                                └─ e81efb78  attestation documentation (this commit, Git tag target)
```

### Commit Hash Mapping

The evidence record (firebird_74c6e5f.json) was sealed with the original unsigned
commit hashes. The commits were subsequently re-signed with GPG signatures to
satisfy the repository's commit-signing rules. The content is identical; only the
hashes changed.

| Role | Hash in evidence record | Hash in repository (signed, GitHub-verified) |
|---|---|---|
| Phase 3 contract | `da57bdd9` | `16c55a24` |
| Canonical encoding | `8c766108` | `cc31d21f` |
| Emitter core | `bb121851` | `3f435d89` |
| Oracle bridge | `a3ae814c` | `a04b73f9` |
| Code under test | `57b695c0` | `97f12bc0` |
| Benchmark infrastructure | `d4c63c38` | `ae51e194` |
| Clean repository state | `52f4ef3c` | `6b5839dd` |

**Reproduction note:** Checkout `ae51e194` for the benchmark harness. The oracle-layer
and verifier code at `ae51e194` is identical to `97f12bc0` — the bench commit only adds
the bench file and criterion dependency. Build with
`RUSTFLAGS="-C target-cpu=icelake-server"` and run:
`cargo bench -p tscp-verifier --bench evidence_baseline -- --save-baseline evidence_baseline`

---

## Hardware / Environment

| Field | Value |
|---|---|
| CPU | GenuineIntel, family 6, model 106 (Ice Lake-SP, 0x6A) |
| AVX-512 features | F, DQ, CD, BW, VL, VBMI, VBMI2, VNNI, BITALG, VPOPCNTDQ |
| Host | modal (Modal.com sandbox) |
| OS | Debian 12 (bookworm) |
| Kernel | 4.19.0-gvisor |
| Rust | 1.97.1 (8bab26f4f 2026-07-14) |
| Target triple | x86_64-unknown-linux-gnu |
| Target CPU | `icelake-server` (explicit, **not** `native`) |
| Build profile | release |
| RUSTFLAGS | `-C target-cpu=icelake-server` |

The kernel (4.19) does not resolve the Ice Lake-SP model name string; CPUID
family/model was used for identification.

---

## Methodology

- **Benchmark framework:** Criterion 0.5.1
- **Sample size:** 10 per trace size per measurement type
- **Warmup:** 3 seconds per benchmark
- **Sampling mode:** Flat
- **Measurement types:**
  - `integrity` — prove + verify (full pipeline; **not** for speedup claims)
  - `performance` — prove only, verify outside timed region (for future hardware/backend comparisons)
- **Correctness gate:** verification must pass before any timing is accepted
- **Layer:** end_to_end

### Timing Source Distinction

**Criterion's `estimates.json` is the canonical timing source.** These values include
proper statistical analysis with confidence intervals and standard deviations.

**PhaseTimer values are retained as instrumentation/provenance.** They show where
time is spent within the prove call (transcript generation, sumcheck, FRI) but are
not the canonical timing reference. The two measurement methods produce slightly
different values (1-6% delta), which is expected.

---

## Test Status

| Check | Result |
|---|---|
| tscp-verifier tests | 12/12 passed |
| Compiler warnings | 0 |
| All verification gates (benchmark) | PASSED |
| Working tree at capture | Clean |
| 8-point audit | All checks passed |

---

## What Was Established

- AVX-512 hardware verified on Ice Lake-SP (CPUID family=6, model=106).
- Evidence-mode build used explicit `target-cpu=icelake-server`, not `native`.
- Criterion benchmark executed under a frozen, reproducible methodology.
- Correctness gates passed for all trace sizes.
- 12/12 verifier tests passed with zero compiler warnings.
- Working tree clean at time of capture.
- Criterion `estimates.json` values are the canonical timing source.
- PhaseTimer data retained as instrumentation/provenance.
- Evidence bundle and canonical record are cryptographically identified.
- Exploration manifest `688d3663` remains separate from the evidence baseline.

## What This Does Not Claim

- It does not claim GPU performance.
- It does not claim general AVX-512 performance across all CPUs.
- It does not claim a universal speedup.
- It does not claim a comparison with a future GPU implementation.
- It does not claim the CPU baseline is representative of every Ice Lake or AVX-512 machine.

It establishes one reproducible CPU AVX-512 reference environment.

## Interpretation Boundary

This document attests to the evidence capture. Interpretation of the measurements
— what the numbers mean, why the result matters, comparison with other
implementations — is a separate layer that may evolve without reopening the evidence.

```
LAYER 1 — EVIDENCE          firebird_74c6e5f          (immutable)
LAYER 2 — ATTESTATION       this document + Git tag   (describes evidence)
LAYER 3 — INTERPRETATION    future analysis           (evolves freely)
```

---

## Audit Trail

The evidence baseline was sealed after an 8-point audit:

1. Benchmark infrastructure committed (`ae51e194`, originally `d4c63c38`)
2. Diff contains only intended benchmark/evidence infrastructure
3. Evidence bundle records the exact repository commit under test (`97f12bc0`, originally `57b695c0`)
4. Criterion `estimates.json` is canonical; PhaseTimer is instrumentation
5. Bundle hash verified after all final edits
6. Freeze script revalidated in evidence mode
7. Working tree clean at HEAD (`6b5839dd`, originally `52f4ef3c`)
8. Final commit hash recorded alongside `firebird_74c6e5f`

All 8 checks: **PASSED**.

---

## Future CPU/GPU Comparison Architecture

```
CPU evidence                GPU evidence
firebird_74c6e5f            firebird_<future-id>
        |                           |
        +-----------+---------------+
                    |
                    v
          comparison artifact
          (separate claim, references both)
```

A CPU-GPU comparison is a **separate claim** that identifies both evidence
baselines. Neither baseline is the implicit denominator for the other.
