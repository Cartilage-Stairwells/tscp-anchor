# TSCP Anchor — Verified Project Facts (Repository Scope)

> This document describes only facts verified from repository state and referenced release artifacts.

## Repository Identity

- **Repository**: `https://github.com/Cartilage-Stairwells/tscp-anchor`
- **Purpose**: TSCP protocol formal verification and custody boundary research repository

## Source-of-Truth Boundary

This document describes only facts verified from repository state and referenced release artifacts.

Claims requiring external benchmark execution, hardware availability, unpublished research notes, or undocumented experiments are not considered part of this document unless linked to a reproducible artifact.

All statements reference one or more of:

- Committed artifacts visible in `git`
- Released tags and associated documentation
- Linked external repositories with reproducible contents

## Document Authority Order

Multiple supporting documents may exist for this project. Public claims in this document take precedence over unpublished or internal materials.

The public claim boundary is:

1. **PROJECT_FACTS.md** — Canonical public claims
2. **Reviewer packet** — External review scope and guidance
3. **Engineering documentation** — Implementation details and technical context
4. **Supporting records** — Additional research and development materials

Documents describing implementation details or development history do not expand the public claims beyond what is established here.

## Repository Contents

### Core Artifacts

- `README.md` — Repository overview, environment information, usage instructions, and project references
- `tscp-docs/` — Protocol documentation and specification materials
- Lean formal verification sources
- CI workflow definitions under `.github/workflows/`

### Formal Verification Scope

The repository contains formal verification work related to:

- Protocol semantics
- Custody boundary definitions
- Verification invariants
- Reproducibility and evidence handling

Formal artifacts are intended to provide machine-checkable boundaries and verification surfaces.

## Evidence and Verification

Verification artifacts in this repository are organized around:

- Reproducible repository state
- Automated verification workflows
- Historical validation artifacts
- Reviewable source materials

Evidence should be evaluated against committed repository state and associated release artifacts.

## Performance Claims Boundary

This repository does not make production-performance guarantees.

Performance measurements, benchmarks, or hardware-specific results must be evaluated only through their associated reproducible benchmark artifacts and execution conditions.

No performance claim should be inferred from the existence of formal verification artifacts alone.

### Benchmark Interpretation

Historical Criterion benchmark measurements describe isolated computational kernels.

They should not be interpreted as proving equivalent acceleration of complete proving workloads without system-level profiling.

Kernel speedup does not linearly compose into end-to-end pipeline speedup. Cache hierarchy, memory bandwidth, workload scheduling, and system architecture may affect overall performance.

### Registered Evidence Baselines

This repository registers evidence baselines in [BENCHMARK_PROVENANCE.md](docs/benchmarks/BENCHMARK_PROVENANCE.md). Each baseline has a unique evidence identity and is immutable once sealed or registered.

| Evidence identity | Scope | Hardware | Method | Result | Status |
|---|---|---|---|---|---|
| `firebird_74c6e5f` | End-to-end prover | Intel Ice Lake-SP | `target-cpu=icelake-server` | (sealed) | Sealed |
| `experiment_a_afea62bc` | Kernel butterfly | AMD Zen 5 | `target-cpu=x86-64` | 7.23x peak, 4.42x GM | Registered |

The `experiment_a_afea62bc` baseline is a cross-repository evidence artifact. Its evidence resides in `Cartilage-Stairwells/zksha-rx-reviewer-access` (branch `evidence/experiment-a`, commit `8df0c247`). See [EXPERIMENT_A_EVIDENCE_REFERENCE.md](docs/benchmarks/EXPERIMENT_A_EVIDENCE_REFERENCE.md) for the full cross-reference.

These baselines are complementary, not competitive. They measure different scopes on different hardware under different methodologies.

## Frozen Claims Table

| Claim | Status | Evidence |
|---|---|---|
| BabyBear field implementation exists | Verified | Source and tests in referenced artifacts |
| AVX-512 backend exists | Verified | Related implementation repository; existence verified, SIMD correctness/formal verification not claimed |
| SIMD output matches reference paths | Verified | Equivalence testing artifacts (access restricted) |
| Montgomery arithmetic formalized | Verified | Lean 4 formal modules |
| Entire NTT formally verified | Not claimed | — |
| Kernel benchmark measurements exist | Historical benchmark result | Referenced benchmark artifacts |
| AVX-512 kernel speedup measured (7.23x peak, 4.42x GM) | Measured | `experiment_a_afea62bc` (cross-repository, zksha-rx-reviewer-access) |
| End-to-end zk prover acceleration | Not claimed | — |
| External cryptographic audit performed | Not claimed | — |
| Energy efficiency of AVX-512 kernel | Not claimed | Energy measurement aborted (gvisor sandbox) |
| Prover integration of AVX-512 kernel | Not claimed | Not tested in Experiment A |

A reviewer should be able to identify claim boundaries from this table alone.

### Claim Language for experiment_a_afea62bc

> Experiment A measured a 7.23x peak speedup at 2^20, with a 4.42x geometric mean
> across the specified benchmark range, for the tested AVX-512 butterfly
> implementation against the specified scalar baseline on AMD Zen 5. Correctness
> and ISA identity were independently checked. Package energy was not measured on
> the execution host, and the experiment does not establish prover integration.

This is a **measured** claim, not a **verified** or **production** claim. The scalar
baseline uses `target-cpu=x86-64` (no auto-vectorization), which differs from the
source repository's canonical benchmark (`target-cpu=native`, 1.265x-1.276x). Both
are valid under their respective methodologies.

## Reviewer Entry Points

Recommended review order:

1. **README.md**
   - Repository overview
   - Setup instructions
   - Build and verification guidance

2. **Protocol Documentation**
   - `tscp-docs/`
   - Specification and design materials

3. **Formal Sources**
   - Lean verification artifacts
   - Formal definitions and proofs

4. **CI Workflows**
   - Automated verification and reproducibility checks

5. **Related Artifacts**
   - Linked repositories and benchmark materials where applicable

### Reviewer Scope Guidance

A productive review should target a specific boundary rather than attempting to validate the entire project.

Examples:

- One implementation surface
- One methodology question
- One architectural comparison
- One formal boundary

## Explicit Non-Claims

- **No security audit**: No independent security audit claim is made by this repository.
- **No production-ready claim**: The repository represents research and verification work unless a separate release explicitly states otherwise.
- **No unsupported provenance claim**: Repository history and artifacts should be evaluated from available Git records and linked evidence.
- **No unverified transformation claim**: Informal descriptions are separated from reproducible artifacts.
- **No prover integration claim**: Experiment A measures kernel-level butterfly performance, not prover integration.
- **No energy efficiency claim**: Energy measurement was aborted due to sandbox limitations.

## Verification

To verify repository state:

```bash
git status
git log --all --oneline --decorate
git tag -n
git diff
```
