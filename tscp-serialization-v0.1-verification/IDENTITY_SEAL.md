# Identity Seal: tscp-serialization-v0.1

**Status:** VERIFIED
**Sealed:** 2026-07-18T22:15:00Z
**Authority:** Cartilage-Stairwells Verification Authority

## Verification Result

All seven verification predicates defined in CONTRACT.md have been satisfied.

| # | Predicate | Verdict |
|---|---|---|
| 1 | Schema Compliance | PASS |
| 2 | Canonical Encoding Correctness | PASS |
| 3 | Hash Stability | PASS |
| 4 | Mutation Sensitivity | PASS |
| 5 | Context Isolation | PASS |
| 6 | Toolchain Reproducibility | PASS |
| 7 | Authority Boundary Preservation | PASS |

## Environment

- **rustc:** 1.97.1 (8bab26f4f 2026-07-14)
- **cargo:** 1.97.1 (c980f4866 2026-06-30)
- **Platform:** Linux x86_64 (gvisor)
- **Kernel:** 4.19.0-gvisor

## Identity Anchors

| Artifact | SHA-256 |
|---|---|
| Verification run log | `4db5f5919603926b3b4ed9e8792c4233a1a3dcb3d9220663f7a6db76b64f6019` |
| Test suite (serialization_conformance.rs) | `e13fd5d0c0ba1ff4301efe6900feed37cf500f1bcec633a39ce12383e406f475` |
| serialization.rs | `676563d082490dcdd69787446da2e5279917ef97f97047371be817b9ea4da8b6` |
| types.rs | `c538f9440f94ab64652ef7cc0f06dc1ba4e08d95ecf09a38aa307bd7258f468e` |

## Change note (2026-07-18)

The test suite and types.rs were updated to fix two reviewer findings:

1. **P1 #2 — Serde strict rejection:** Added `#[serde(deny_unknown_fields)]` to
   `TransitionReceipt`. Updated `test_custody_expression_blocked` to construct a real
   CBOR payload with a "custody" field and assert rejection (not just round-trip stability).

2. **P2 #4 — Mutation test overclaim:** Corrected the doc comment on
   `test_mutation_always_detected` to accurately state that it covers exhaustive bit-flip
   mutation across the 96 hash-field bytes only. The `kernel_version` and `kind` fields
   are covered by `test_mutation_changes_hash`.

The verification run log hash was already correct in SHA256SUMS. The seal had an
internal inconsistency (stale hash `34d55c12...`). This regeneration corrects it.

## Scope

This seal certifies artifact identity verification only.
It does NOT imply VERIFIED_CUSTODY. Custody carries independent predicates
and is not inherited from artifact identity verification.

## Transition

```
FROZEN_SPECIFICATION
        ↓
  Verification execution (12/12 tests PASS)
        ↓
VERIFICATION_PACKAGE_PASS
        ↓
VERIFIED_ARTIFACT_IDENTITY ← sealed 2026-07-18T22:15:00Z
```

---

*Sealed by Cartilage-Stairwells Verification Authority*
*tscp-serialization-v0.1-verification*
