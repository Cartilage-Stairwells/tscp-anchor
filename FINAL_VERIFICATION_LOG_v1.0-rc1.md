# Final Verification Log — v1.0-rc1
# TSCP Release Candidate 1

**Date:** 2026-07-17
**Sealed by:** Cartilage-Stairwells Verification Authority
**GPG Key:** E747C3AF22573539

---

## 1. Repository Integrity

### tscp-anchor (Cartilage-Stairwells)

| Check | Result |
|---|---|
| Master branch | `abb7ad53` (HEAD, origin/master) |
| Signed tag v1.0-rc1 | ✅ Verified (ECDSA, E747C3AF22573539) |
| Commit `abb7ad53` | ✅ Good signature (SEAN CHRISTOPHER SOUTHWICK) |
| Commit `5c6ca424` | ✅ Good signature |
| Commit `b89ee735` | ✅ Good signature |
| Commit `3fe4ec3d` | ✅ Good signature (custody-migration-2026-07-17 tag) |
| Working tree | Clean |
| Remote sync | Up to date with origin/master |
| GitHub release | Published (prerelease, not latest) |
| Release URL | https://github.com/Cartilage-Stairwells/tscp-anchor/releases/tag/v1.0-rc1 |

Signed chain:
```
v1.0-rc1 (signed tag)
    → abb7ad53 (signed commit — RE-SIGN.md update)
    → 5c6ca424 (signed commit — release manifest + SHA256SUMS)
    → b89ee735 (signed commit — serialization conformance suite)
    → 3fe4ec3d (signed commit — custody migration checkpoint)
    → e27b79d8 (signed commit — NttBackend abstraction)
```

### avx512-butterfly (Cartilage-Stairwells)

| Check | Result |
|---|---|
| Master branch | `a330235` (HEAD, origin/master) |
| Signed tag v1.0-rc1 | ✅ Verified (ECDSA, E747C3AF22573539) |
| Commit `a330235` | ✅ Good signature (SEAN CHRISTOPHER SOUTHWICK) |
| Commit `e2443f5` | ✅ Good signature |
| Commit `3d1e6c7` | ✅ Good signature |
| Working tree | Clean |
| Remote sync | Up to date with origin/master |
| GitHub release | Published (prerelease, not latest) |
| Release URL | https://github.com/Cartilage-Stairwells/avx512-butterfly/releases/tag/v1.0-rc1 |

Signed chain:
```
v1.0-rc1 (signed tag)
    → a330235 (signed commit — SHA256SUMS)
    → e2443f5 (signed commit — compile fix + IEP runner + serialization fixture)
    → 3d1e6c7 (signed commit — domain-blind core vocabulary, Commit 1)
    → 389b19c (signed commit — freeze order steps 1-6)
```

---

## 2. Test Execution Summary

### avx512-butterfly (85 tests)

| Suite | Tests | Result | Coverage |
|---|---|---|---|
| Library unit tests | 38 | ✅ 38/38 | BabyBear field, Montgomery, canonical, hash, evidence, predicate |
| BabyBear domain | 12 | ✅ 12/12 | Reference agreement, mul semantics, cross-backend equivalence |
| BabyBear Montgomery | 10 | ✅ 10/10 | Golden vectors, oracle agreement, scalar backend, boundary |
| Core domain blindness | 8 | ✅ 8/8 | Domain-blind contract, evidence rejection, hash identity |
| IEP enforcement | 11 | ✅ 11/11 | Full decision matrix: policy/authority/artifact/evidence rejection paths |
| Legacy Montgomery regression | 6 | ✅ 6/6 | Oracle boundary, AVX-512 path, r64 reduction |
| **Total** | **85** | **✅ 85/85** | |

Toolchain: rustc 1.97.1 (8bab26f4f 2026-07-14), cargo 1.97.1, Linux x86_64

### tscp-anchor (12 tests)

| Suite | Tests | Result | Coverage |
|---|---|---|---|
| Serialization conformance | 12 | ✅ 12/12 | Schema compliance, canonical encoding, hash stability, mutation sensitivity, context isolation, toolchain reproducibility, authority boundary |

Toolchain: rustc 1.97.1, cargo 1.97.1, Linux x86_64

### Grand Total: 97/97 tests pass

---

## 3. Evidence Artifacts

### IEP Runner — Reference Evidence

| Field | Value |
|---|---|
| Artifact file | `benchmarks/reports/firebird_reference_80dc195.json` |
| Output hash | `sha256:d2a418be1dec267776a7f7392f521dee0d58651e37295656a4c9e82f4b35bddc` |
| Corpus hash | `sha256:48da63b99e1c7e0ce2490dd503e1d536850d286136b4b6e7d779a814f152e319` |
| Backend | reference (scalar butterfly) |
| Corpus ID | babybear_vectors_001 |
| Triples | 10 |

### π Conformance Graph

| Field | Value |
|---|---|
| Methods | AGM (Brent-Salamin), Machin, Gauss |
| Digits | 1488 |
| Spot check | Gourdon digit extraction (digits 1488-1496) |
| Identity hash | `sha256:e38786f0c1fd0bd30a147df515452300ae7d2fddf0728d6f429585146a5f3430` |
| Status | Verified |

### SHA256SUMS

Both repos contain `SHA256SUMS` files with integrity checksums for all evidence artifacts. These are included in the signed commit history.

---

## 4. Frozen Specifications

| Specification | Location | Frozen At |
|---|---|---|
| IEP v0.1 | avx512-butterfly/IEP.md | `c422bfb` |
| Evidence schema | instrument/evidence_schema.json | `ee63d46` |
| Promotion policy | instrument/policy/promotion_policy.json | `09e5acb` |
| Evidence kinds | instrument/policy/evidence_kinds.json | `736e5d1` |
| Domain-blind core | src/core/ (7 files) | `3d1e6c7` (Commit 1) |
| Cryptographic vocabulary | All core types | `3d1e6c7` |
| Authority boundary | instrument/authority.json | `8913145` |

---

## 5. Publication Status

| Step | Status | Timestamp |
|---|---|---|
| Signed commit history | ✅ Complete | 2026-07-17 07:34 PDT |
| Signed tags (both repos) | ✅ Complete | 2026-07-17 07:38 PDT (tscp-anchor), 07:56 PDT (avx512-butterfly) |
| Remote verification | ✅ Complete | 2026-07-17 07:58 PDT |
| GitHub releases published | ✅ Complete | 2026-07-17 08:03 PDT |
| Team notification sent | ✅ Complete | 2026-07-17 08:09 PDT (Message ID: 19f70a1d48cd8167) |
| Final verification log | ✅ This document | 2026-07-17 08:38 PDT |

---

## 6. Pending Items for v1.0 Final

| Item | Blocker | Impact |
|---|---|---|
| AVX-512 hardware benchmark | Requires qualified AVX-512 host | Cannot produce real AVX-512 performance measurements; current backend delegates to scalar path |
| AVX-512 backend real intrinsics | Currently stub (scalar fallback) | Backend trait exists, AVX-512 feature gate exists, but no real __mm512 intrinsics |
| avx512-butterfly cleanup (#1, #3, #4, #6) | Separate PR | Repository housekeeping, not blocking release |
| tscp-canon envelope v2 | Unmerged branch | Canonicalization dependency update |

---

## 7. Architecture Assessment

The project architecture is **stable and frozen**. No pending architectural changes exist. The remaining work is:

- **Execution** (running benchmarks on qualified hardware)
- **Implementation** (real AVX-512 intrinsics)
- **Custody** (already complete for rc1)

The transition from architectural exploration to a frozen, reproducible evidence execution model is complete. The IEP enforcement layer mechanically gates all future transitions through a typed decision tree. The domain-blind core ensures that new domains (π, serialization, future cryptographic workloads) enter as instances, not dependencies.

---

*Sealed by Cartilage-Stairwells Verification Authority*
*GPG Key: E747C3AF22573539*
*2026-07-17 08:38 PDT*
