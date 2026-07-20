# Archive Index

This document is the master index for all TSCP project repositories,
sealed artifacts, tags, and milestones.

## Repository Map

### Primary Repositories

| Repo | Account | Visibility | Role | Status |
|:---|:---|:---|:---|:---|
| **tscp-anchor** | Cartilage-Stairwells | Public | Integration, verification, governance | ACTIVE — canonical |
| **avx512-butterfly** | Cartilage-Stairwells | Private | AVX-512 NTT kernels, VEP evidence | ACTIVE — sealed at vep-0.1.4 |
| **tscp-anchor** | Triune-Oracle | Public (billing-locked) | Legacy remote | ARCHIVED — CI non-functional |

### Historical / Legacy Repositories (Triune-Oracle)

| Repo | Cluster | Status | Notes |
|:---|:---|:---|:---|
| tscp-pl-phase1 | TSCP | ARCHIVE (compliance anchor) | 27 Lean 4 theorems, 0 sorry. `phase1-freeze` is read-only. |
| tscp-canon | TSCP | ARCHIVE | Conformance fixtures and canonical test vectors. |
| tscp-crown-capsule | TSCP | REVIEW | PLpgSQL, 3kb. Purpose unclear. |
| toolintell | TSCP / TOOLS | REVIEW | TSCP runtime artifacts, FRI/STARK analysis. |
| triune-swarm-engine | TRIUMVIRATE | MIGRATE | Python multi-agent orchestrator. |
| *(~30 other repos)* | Various | ARCHIVE/DELETE | Inactive, templates, or disposable. |

## Sealed Artifacts

### Tags on tscp-anchor (Cartilage-Stairwells)

| Tag | Commit | Type | Signed | Date |
|:---|:---|:---|:---|:---|
| `v1.0-rc1` | `abb7ad53` | Release candidate | Yes | 2026-07-17 |
| `tscp-serialization-v0.1.0-rc1-signed` | `81ead102` | Feature seal | Yes | 2026-07-19 |
| `tscp-serialization-v0.1.0-rc1` | `81ead102` | Feature seal (unsigned) | No | 2026-07-19 |
| `tscp-mini-ntt-parity-v1` | `27de2f10` | Acceptance package | Yes | 2026-07-18 |
| `tscp-anchor-custody-v1` | — | Custody marker | — | 2026-07-17 |
| `custody-migration-2026-07-17` | — | Migration marker | — | 2026-07-17 |
| `pre-stage2-custody-merge` | — | Pre-merge marker | — | 2026-07-17 |

### Tags on avx512-butterfly (Cartilage-Stairwells)

| Tag | Commit | Type | Signed | Date |
|:---|:---|:---|:---|:---|
| `v1.0-rc1` | `a330235` | Release candidate | Yes | 2026-07-17 |
| `vep-0.1.4-sealed` | `49eac02` | VEP milestone seal | Yes | 2026-07-17 |
| `avx512-v1-evidence-sealed` | `49eac02` | Evidence boundary | Yes | 2026-07-17 |
| `avx512-opt-start-v0.1.0` | `49eac02` | Optimization start | Annotated | 2026-07-17 |

## Acceptance Packages

### tscp-mini-ntt-parity-v1

| Field | Value |
|:---|:---|
| Repository | tscp-anchor |
| Commit | `27de2f100b7e0ad2dc4a5ad09899c93ff361f85f` |
| Tag | `tscp-mini-ntt-parity-v1` (GPG-signed) |
| Verification | 24/24 PASS (vector_generator.py → verify.py) |
| SHA256SUMS | `2c9125388a2b08c70596f03f06af39e5435841c901e919f6820fbd36a9c5f0d3` |
| GPG key | `84692E6294128CC1C4ACCD15E747C3AF22573539` |
| Date | 2026-07-18 |

## VEP Milestones

| Milestone | Status | Date | Commit (avx512-butterfly) |
|:---|:---|:---|:---|
| VEP-0.1.1 — Minimal Evidence Loop | SEALED | 2026-07-17 | `823775b` |
| VEP-0.1.2 — Full Evidence Bundle | SEALED | 2026-07-17 | `3dab112` |
| VEP-0.1.3 — Submission Packaging | SEALED | 2026-07-17 | `2e31ab8` |
| VEP-0.1.4 — CI Enforcement | SEALED | 2026-07-17 | `0d09b10` |

All VEP milestones GPG-signed, verified on GitHub, CI green.

## CI Workflows

### tscp-anchor (Cartilage-Stairwells)

| Workflow | File | Triggers | Status |
|:---|:---|:---|:---|
| TSCP Verification Predicate CI | `.github/workflows/ci.yml` | push, PR | Active |
| TSCP Anchor Pipeline | `.github/workflows/anchor.yml` | push (master) | Active |
| Verify Receipts | `.github/workflows/verify-receipts.yml` | push, PR | Active |
| WASM Smoke Test (JIT) | `.github/workflows/wasm-smoke.yml` | push, PR | Active |
| Supply Chain Governance | `.github/workflows/supply-chain-governance.yml` | push, PR (registry files) | Active |
| Stage 2 Backend Verification | `.github/workflows/stage2-verify.yml` | push | Active |

### avx512-butterfly (Cartilage-Stairwells)

| Workflow | File | Triggers | Status |
|:---|:---|:---|:---|
| VEP Validation | `.github/workflows/vep-validation.yml` | push, PR | Active |

## Cryptographic Anchors

| Anchor | Value |
|:---|:---|
| GPG signing key | `8469 2E62 9412 8CC1 C4AC CD15 E747 C3AF 2257 3539` |
| Key holder | Sean Christopher Southwick |
| Key type | ECDSA P-384 |
| Primary email | `adamantinespine@gmail.com` |
| Secondary email | `schlagetorren@gmail.com` |
| Public key file | `signer-public-key.asc` |

## Supply Chain Exceptions Summary

| ID | Status | Dependency | Next Review |
|:---|:---|:---|:---|
| EX-0001 | RESOLVED | serde_cbor v0.11.2 | — |
| EX-0002 | ACTIVE | crossbeam-epoch v0.9.18 | 2026-08-19 |
| EX-0003 | ACTIVE | wasmtime v29.0.1 | 2026-08-19 |

## Commit Signing History

Signed commits (verified=True on GitHub):
- `1e5e824` — ciborium merge (2026-07-19)
- `58c18a7` — test signing key (2026-07-19)
- `81ead10` — custody merge (2026-07-19)

Historical unsigned commits (documented in SECURITY.md):
- `e176f88` — ciborium feature commit (2026-07-19)
- `c048743` through `5bb0bb2` — cargo fmt/clippy cleanup (16 commits, 2026-07-19)

Policy: protect future commits, document historical unsigned commits,
do not rewrite history unless an explicit provenance migration is decided.

---

*Last updated: 2026-07-19*
*Maintained by: Cartilage-Stairwells*
*Canonical repository: https://github.com/Cartilage-Stairwells/tscp-anchor*
