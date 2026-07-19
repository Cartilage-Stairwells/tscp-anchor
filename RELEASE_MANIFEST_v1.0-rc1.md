# TSCP Release Candidate 1 (v1.0-rc1)
# Release Manifest

**Date:** 2026-07-18
**Regenerated:** 2026-07-18T22:15:00Z (provenance correction — see change note)
**Sealed by:** Cartilage-Stairwells Verification Authority
**GPG Key:** E747C3AF22573539

## Scope

This release candidate packages the TSCP proving stack evidence corpus.
It represents the transition from architectural exploration to a frozen,
reproducible evidence execution model.

**No architectural changes are pending.** Remaining work is execution,
benchmarking on qualified hardware, and final release custody.

---

## Repository Anchors

| Repository | Commit | Status |
|---|---|---|
| Triune-Oracle/tscp-anchor (custody-merge-ready) | `8b0969d61df3f9758193c3c7ec18a04f85ca4b5a` | ⏳ Pending GPG re-sign |
| Cartilage-Stairwells/avx512-butterfly | `288d58af9bf2033e83ce72bdfa15cdf3f4dd5e3e` | ⏳ Pending GPG re-sign |
| Triune-Oracle/tscp-canon | pinned, no changes | ✅ Stable |

The re-sign procedure is documented in RE-SIGN.md.

---

## Change Note (2026-07-18 — Provenance Correction)

The previous manifest (2026-07-17) pointed at commit `1cff176bb3a0cc0ac555a11c8487751168b6f6ce`
(Cartilage-Stairwells/tscp-anchor). The PR under review exists on `Triune-Oracle/tscp-anchor`
at commit `8b0969d61df3f9758193c3c7ec18a04f85ca4b5a`. This manifest has been regenerated to describe the
exact commit tree being merged.

Additionally, this commit batch fixes three reviewer findings:
1. **P1 #2 (Serde strict rejection):** `#[serde(deny_unknown_fields)]` added to
   `TransitionReceipt`; `test_custody_expression_blocked` now constructs a real CBOR
   payload with a forbidden "custody" field and asserts rejection.
2. **P2 #4 (Mutation test overclaim):** Doc comment corrected — `test_mutation_always_detected`
   now accurately claims exhaustive bit-flip coverage over the 96 hash-field bytes only.
3. **P2 #5 (Run-log SHA mismatch):** `IDENTITY_SEAL.md` updated to use the SHA-256 that
   matches the committed `VERIFICATION_RUN.md` artifact (`4db5f591...`).

---

## Evidence Corpus

### 1. Backend Abstraction (tscp-backends) ✅ Verified

| Artifact | SHA-256 | Status |
|---|---|---|
| `crates/tscp-backends/evidence/stage2_verification.json` | `fa2208322d28bb7781bdf6057b5f285160a38bcebacf593257a501e0d9f16822` | ✅ Verified |
| `crates/tscp-backends/src/lib.rs` | `564a8023e7179b4a49037881c1209b2c270240db9dcfb1d773b9d9d51b601eb7` | ✅ Verified |

**Verification gates:** cargo fmt ✅, cargo clippy ✅, scalar roundtrip ✅ (2/2), AVX-512 parity ✅ (4/4).

### 2. Serialization Conformance (tscp-kernel) ✅ Verified

| Artifact | SHA-256 | Status |
|---|---|---|
| `crates/tscp-kernel/tests/serialization_conformance.rs` | `e13fd5d0c0ba1ff4301efe6900feed37cf500f1bcec633a39ce12383e406f475` | ✅ |
| `crates/tscp-kernel/src/serialization.rs` | `676563d082490dcdd69787446da2e5279917ef97f97047371be817b9ea4da8b6` | ✅ |
| `crates/tscp-kernel/src/types.rs` | `c538f9440f94ab64652ef7cc0f06dc1ba4e08d95ecf09a38aa307bd7258f468e` | ✅ (updated) |
| `tscp-serialization-v0.1-verification/VERIFICATION_RUN.md` | `4db5f5919603926b3b4ed9e8792c4233a1a3dcb3d9220663f7a6db76b64f6019` | ✅ |
| `tscp-serialization-v0.1-verification/IDENTITY_SEAL.md` | `d609d77b524d7a00ec883e3b5c3565bfd7fb8fe9b2abbe810c78edac77459971` | ✅ (updated) |

**Verification gates:** 12/12 conformance tests PASS. Authority boundary enforced
via `#[serde(deny_unknown_fields)]`. Custody expression CBOR payload rejected.

### 3. Integrity Manifest

| Artifact | SHA-256 |
|---|---|
| `SHA256SUMS` | `f03ef3ba198f67679cf639c7b5e98951bc2377c16eac1f536ab65e149e76e649` |

SHA256SUMS covers all evidence artifacts above. IDENTITY_SEAL.md is included in SHA256SUMS
to seal the verification record. The manifest itself is not self-referential.

---

## Pending Before v1.0 Final

| Item | Blocker |
|---|---|
| GPG re-sign of both repo commits | Requires local execution with key `E747C3AF22573539` |
| AVX-512 hardware benchmark | Requires qualified AVX-512 host |
| P2 #3 — Evidence file population | Evidence stubs remain; Option A (populate from run) preferred |
| tscp-canon envelope v2 | Unmerged branch |

---

*Cartilage-Stairwells Verification Authority*
*Regenerated: 2026-07-18*
