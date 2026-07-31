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

## Frozen Claims Table

| Claim | Status | Evidence |
|---|---|---|
| BabyBear field implementation exists | Verified | Source and tests in referenced artifacts |
| AVX-512 backend exists | Verified | Related implementation repository (access restricted) |
| SIMD output matches reference paths | Verified | Equivalence testing artifacts (access restricted) |
| Montgomery arithmetic formalized | Verified | Lean 4 formal modules |
| Entire NTT formally verified | Not claimed | — |
| Kernel benchmark measurements exist | Historical benchmark result | Referenced benchmark artifacts |
| End-to-end zk prover acceleration | Not claimed | — |
| External cryptographic audit performed | Not claimed | — |

A reviewer should be able to identify claim boundaries from this table alone.

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

## Verification

To verify repository state:

```bash
git status
git log --all --oneline --decorate
git tag -n
git diff
