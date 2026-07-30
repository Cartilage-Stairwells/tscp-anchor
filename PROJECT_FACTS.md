# PROJECT_FACTS.md

**Source-of-truth document for external reviewers, grant committees, and collaborators.**

Last updated: 2026-07-29

---

## Identity

**TSCP** (Triune Structured Codex Protocol) — a verification custody framework for cryptographic implementations. The system makes verification evidence independently auditable: every claim about what was tested, how it was tested, and what build artifact was tested is recorded in a cryptographically signed receipt with nine machine-checked invariants.

Built on [Plonky3](https://github.com/Plonky3/Plonky3). Formal proofs in Lean 4. Implementation in Rust. On-chain anchor in Solidity.

Author: Sean Southwick — [Cartilage-Stairwells](https://github.com/Cartilage-Stairwells) (org) / [Triune-Oracle](https://github.com/Triune-Oracle) (personal)

---

## Repository Map

| Repository | Role | Visibility | Branch strategy |
|---|---|---|---|
| [tscp-anchor](https://github.com/Cartilage-Stairwells/tscp-anchor) | Verification & custody framework — proving layer, formal proofs, custody invariants, test suites | **Public** | `master` (GPG-signed commits required) |
| [tscp-pl-phase1](https://github.com/Cartilage-Stairwells/tscp-pl-phase1) | Frozen Phase 1 research artifact — AVX-512 butterfly kernels, Lean4 formal proof, benchmarks | **Public** | `phase1-freeze` (frozen, content verified to match tested sandbox as of 2026-07-08) |
| avx512-butterfly | Active evidence & benchmark repository — NTT equivalence, staged cross-backend comparison, custody receipts | **Access restricted** | `master` (GPG-signed commits required) |

**The relationship:**

```
tscp-anchor        → Custody framework + formal verification + on-chain anchor
                        │
                        │ verifies evidence from
                        ↓
avx512-butterfly   → Active implementation: AVX-512 NTT, backend parity tests,
                      execution trace receipts, build identity artifacts
                        │
                        │ frozen snapshot at Phase 1
                        ↓
tscp-pl-phase1     → Frozen research artifact: 9.15× speedup, 61 verification
                      points, Lean4 proof (27 theorems, 0 sorry), benchmarks
```

---

## Verified Technical Claims

### Custody framework (tscp-anchor, PR #29 — merged to master)
- **9 verification invariants** across 4 layers (see [VERIFICATION_INVARIANTS.md](VERIFICATION_INVARIANTS.md))
- **6-class failure taxonomy**: False Shoreline, Self-Attestation, Authority Confusion, Provenance Gap, Semantic Drift, Verification Surface Drift
- **12/12 fixture tests pass** (9 negative cases + 3 evidence alignment + 1 positive control)
- **24/24 mutation tests pass** — each invariant proven necessary and sufficient in isolation
- **Core invariant**: ∀r: (ClaimedTarget(r), ClaimedBackend(r)) = (ExecutedTarget(r), SelectedBackend(r))
- Receipt lifecycle: GENERATED → AUDITED → TARGET MISMATCH DISCOVERED → SUPERSEDED/REVOKED
- Historical receipt from commit 0205722 **rejected by 6 independent invariants**

### Formal verification (tscp-anchor)
- **Lean 4** formal backbone — [TSCP_Formal_Backbone.lean](TSCP/Formal/TSCP_Formal_Backbone.lean)
- [Montgomery.lean](TSCP/Formal/Montgomery.lean) — Montgomery reduction correctness
- [BridgePreservation.lean](TSCP/Formal/BridgePreservation.lean) — bridge preservation theorem
- [Boundary.lean](BabyBear/Boundary.lean) — trust ledger / axiom inventory
- [Core.lean](BabyBear/Core.lean) — BabyBear predicate theorem (replaced `proof_valid` axiom)
- Bézout identity — dual-inverse derivation for Montgomery constants
- **0 `sorry`** in any proof file

### On-chain anchor
- [TSCPAnchor.sol](contracts/) — immutable Solidity registry
- Deployed on **Sepolia** at `0x6FDB70F31E4815bE866Fd6aDD32802f90F9B5E06`
- Verifiable via any Sepolia block explorer

---

## Performance Evidence

### AVX-512 implementation (avx512-butterfly)
- **3.3×–4.4× speedup** over scalar (refinement receipt, commit 9473af6, [AVX512_REFINEMENT_RECEIPT.md](docs/AVX512_REFINEMENT_RECEIPT.md))
- **9.15× speedup** (Phase 1 frozen benchmark, tscp-pl-phase1, Criterion 0.5, 100 samples, <0.1% CI)
- **135 staged comparisons** across 45 stages × 3 backends — 0 mismatches
- **140/140 correctness tests** after DIF butterfly correction (Semantic Drift resolved)
- Compiler: `rustc 1.97.1 (8bab26f4f 2026-07-14)` — matched between CI and sandbox
- Artifact hash: `198bf0e7c139e3405f7a0987db02c408335204da1a23060714d21afd219a912c`

---

## Reproducibility Status

- Build environment: reproducible (rustc version pinned, CI workflow captures toolchain metadata)
- Benchmark environment: documented in provenance JSON (CPU, kernel, ISA flags)
- Lean proofs: compile with Lean 4.31.0 (core only, no Mathlib required)
- Custody test suite: `python3 tests/custody/verify_custody_receipts.py` — 12/12 pass
- Mutation test suite: `python3 tests/custody/mutation_tests.py` — 24/24 pass

---

## Issue #27 — Verification Surface Drift (CLOSED)

**What happened:** A verification receipt (commit 0205722 in avx512-butterfly) claimed to certify AVX-512 NTT equivalence, but the test actually exercised a scalar delegate wrapper. The AVX-512 SIMD path was never invoked by the test. The receipt was sealed 8 minutes before the real SIMD code was committed.

**What the custody framework did:**
- Historical receipt (0205722) **rejected by 6 independent invariants**: Target Binding, Build Artifact Identity, Hardware Presence, Fallback Prohibition, Claim Scope Integrity, Observation Independence
- Corrected receipt (9473af6) **passes all 9 invariants** after evidence alignment
- No hardware re-execution needed — the codebase already had the infrastructure (feature probes, harness isolation, oracle independence), the receipt had not populated all fields
- Three fixtures document the alignment process: raw → observation-aligned → fully-closed

---

## Comparative Context

These projects address adjacent problems. TSCP does not replace any of them — it operates at a different layer (evidence custody) that could wrap any of their implementations.

| Project | Scope | Relationship to TSCP |
|---|---|---|
| HACL*/EverCrypt (MIT/Project Everest) | Verified cryptographic implementations (ChaCha20, Curve25519, Poly1305, AES). First verified SIMD on ARM Neon + AVX-512 via HACLxN. | Adjacent: implementation correctness. A TSCP receipt could verify that evidence about a HACL* implementation actually exercises the claimed code path. |
| Jasmin (INRIA) | Verified high-performance crypto assembly via Coq. AVX-2 supported; AVX-512 not yet per 2020 paper. | Adjacent: verified low-level crypto. Same question TSCP asks — did the evidence touch the right code? — applies to Jasmin-compiled code. |
| Fiat-Crypto (MIT) | Generated arithmetic implementations with Rocq proofs. Scalar code generation; no SIMD. | Adjacent: arithmetic correctness. TSCP's custody framework could verify that a Fiat-Crypto generated artifact's evidence chain binds to the actual compiled output. |
| libcrux (Cryspen/CE Labs) | Cryptographic library using HACL* verified code. Kobeissi (IACR 2026/192) documented specification bugs in verified AVX-2 code (Findings 1–3, Feb 2026; Findings 1–2 fixed March 2026). | Adjacent: cryptographic primitives. The Kobeissi findings illustrate the failure class TSCP formalizes as "Verification Surface Drift" — the verification boundary did not cover what was claimed. |
| SLSA (Google) | Supply-chain provenance framework. Tracks build provenance (builder identity, source, build instructions). Does not verify execution binding or artifact hashes. | Adjacent: build/provenance assurance. SLSA answers "where did this come from?" TSCP answers "does the evidence actually test what it claims to test?" |
| TSCP stack (this project) | Evidence custody, verification receipts, execution binding, build identity, formal failure taxonomy. | Own scope: evidence/ custody/ proof pipeline. Operates above implementation verification, not alongside it. |

**Key sources:**
- HACLxN: "Verified Generic SIMD Crypto" (ePrint 2020/572)
- Kobeissi: "Verification Theatre: False Assurance in Formally Verified Cryptographic Libraries" (IACR ePrint 2026/192)
- Jasmin: "High-Assurance and High-Speed Cryptography" (HAL 01649140)
- Fiat-Crypto: "Simple High-Level Code for Cryptographic Arithmetic" (MIT)
- SLSA: slsa.dev specification

---

## Limitations

- Plonky3 version pinned to 0.6.1 pending validated migration to 0.6.2
- External cryptographic audit has not been performed
- Sepolia deployment is for development verification only
- AVX-512 code paths require hardware with AVX-512F + AVX-512DQ support
- Lean proofs cover Montgomery reduction and custody properties, not full cryptographic algorithms
- The competitive matrix above reflects publicly documented capabilities as of July 2026

---

## Current Review State

- tscp-anchor: **Public**, all custody framework artifacts on master
- tscp-pl-phase1: **Public**, frozen Phase 1 artifact on `phase1-freeze` branch
- avx512-butterfly: **Access restricted** (active development, contains unreleased evidence; contact for access)
- GPG signing required on master branches (enforced via GitHub repository rulesets)
- Signing key: ECDSA, fingerprint `84692E6294128CC1C4ACCD15E747C3AF22573539`

---

## Timeline

| Date | Milestone |
|---|---|
| 2026-07-14 | Project migration, CI setup, toolchain baseline |
| 2026-07-15 | IEP evidence schema freeze, π Machin/Gauss first artifact |
| 2026-07-16 | Core vocabulary, environment contract, E7 invariant, serialization scaffold |
| 2026-07-17 | VEP 0.1.1–0.1.4, CI enforcement, evidence bundle, v1.0-rc1 release |
| 2026-07-20 | Lean 4.32 compilation fixes, ProofArtifact custody boundary |
| 2026-07-23 | Formal core v2.2, PLONKY3 comparison, supply chain seal, BabyBear trust frontier |
| 2026-07-24 | NTT equivalence (staged cross-backend), real SIMD vectorized butterfly (AVX-512) |
| 2026-07-25 | MontgomeryBridge (PR #28), attestation workflow, custody architecture design |
| 2026-07-26 | DIF butterfly correction (Semantic Drift → 140/140 pass), verification gate v1, custody boundary artifacts (PR #29), verification surface drift test suite, mutation tests |
| 2026-07-27 | Issue #27 CLOSED — existing AVX-512 evidence passes all 9 custody invariants |
