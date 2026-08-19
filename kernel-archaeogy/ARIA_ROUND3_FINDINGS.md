# Aria Round 3 Correspondence Audit — Findings & Fixes

**Date:** August 18, 2026
**Auditor:** Aria (ChatGPT)
**Subject:** Specification ↔ Rust implementation correspondence

## Three Concrete Defects Found

### FIX 1: Empty evidence panic (FAIL → FIXED)
- **Finding:** `admit(contract, &[])` panics on `evidence[0]` indexing instead of returning `RejectedEvidence`
- **Spec says:** insufficient evidence must produce `TSCP-ADMIT-INSUFFICIENT-EVIDENCE`
- **Fix:** Use `evidence.first().cloned()` instead of `evidence[0].clone()`. `RejectedEvidence.evidence` is now `Option<Evidence>` to handle the empty case.
- **Test:** `t_empty_evidence_no_panic` — 31/31 pass

### FIX 2: Canon-version correspondence absent (FAIL → FIXED)
- **Finding:** `CanonVersionMismatch` error code exists but no code path produces it. Evidence had no `canon_version` field.
- **Spec says:** evidence canon_version must match contract canon_version (§3.2 Stage 1.3)
- **Fix:** Added `canon_version: String` field to `Evidence`. Validation stage now checks `ev.canon_version != contract.canon_version()` and rejects with `CanonVersionMismatch`.
- **Test:** `t_canon_version_mismatch` — 31/31 pass

### FIX 3: Contract immutability violated (FAIL → FIXED)
- **Finding:** All `Contract` fields were `pub`, allowing mutation: `contract.max_evidence_count = 0`
- **Spec says:** "Contracts are read-only after creation" (§2.3)
- **Fix:** All Contract fields are now private. Construction via `Contract::new()` which validates. Read-only accessors provided. No mutation path exists.
- **Test:** `t_contract_immutability` — compiles only with accessors; field mutation would not compile

## Deferred

### Duplicate Admission (DEFERRED)
- Spec defines `TSCP-ADMIT-DUPLICATE-ADMISSION` but §3 does not define the operational mechanism for "already admitted"
- Not an implementation bug — specification gap

## What Survived

### AdmittedEvidence Construction Boundary: PASS
No safe-Rust constructor, public field, serde, Default, unsafe, FFI, or conversion path lets an attacker manufacture `AdmittedEvidence` without `admit()`. Clone requires already-admitted value.

### Semantic Firewall: PASS
Fabricated digests still admitted because structurally valid. Admission ≠ truth preserved.

## Updated Classification

| Axis | Result |
|:---|:---|
| Digest structural validation | PASS |
| artifact_type validation | PASS |
| Role validity | PASS |
| Canon-version correspondence | **FIXED** (was FAIL) |
| Type binding | PASS |
| Role binding | PASS |
| Duplicate digest | PASS |
| Minimum evidence (empty domain) | **FIXED** (was FAIL/panic) |
| Maximum evidence | PASS |
| Required roles | PASS |
| Contract immutability | **FIXED** (was FAIL) |
| AdmittedEvidence private construction | PASS |
| Deserialization bypass | PASS |
| Unsafe/transmute bypass | PASS |
| FFI bypass | PASS |
| Clone bypass | PASS |
| Authority laundering | PASS |
| Truth/correctness laundering | PASS |
| Duplicate-admission predicate | DEFERRED (spec gap) |

**Test count:** 27 → 31 (4 new tests for the three fixes + empty contract converse)
**Result:** 31/31 pass
