# Verification Surface Drift Test Plan

**Document type:** Validation suite specification
**Status:** v1.0
**Motivating case:** Issue #27 — Verification Surface Drift
**Date:** 2026-07-26

---

## Purpose

This document specifies a negative/positive validation suite that proves the
custody machinery catches the exact failure class that created Issue #27.

The merge of PR #29 establishes the machinery — the nine invariants, the
artifact schemas, the gate chain. This test plan proves the machinery works.

The principle:

```
Invalid Evidence Path  →  Rejected
Correct Implementation Path  →  Accepted
```

The milestone is not "AVX-512 passes." The milestone is:

```
A receipt about the wrong implementation path is now impossible to certify.
```

That is the Issue #27 closure condition.

---

## Acceptance predicate

The final acceptance predicate:

```
Claim = Target = Artifact = Execution = Observation = Scope
```

No equality → no receipt. Every axis must match. A single mismatch on any
axis invalidates the receipt, regardless of whether all other axes pass.

---

## Test matrix

### Test 1 — Symbol Identity Trap

**Invariant violated:** 2 (Target Binding — backend dimension)

**Setup:**
- Receipt claims backend: `avx512`, symbol: `avx512_radix2_butterfly_32`
- Execution shows symbol: `avx512_radix2_butterfly_32`, backend: `scalar`
- The function name matches. The execution path does not.

**Expected result:** `REJECT`
**Reason:** `ExecutedBackend ≠ ClaimedBackend`
**Catches:** Name ≠ Identity. A function named `avx512_radix2_butterfly_32`
that actually runs scalar arithmetic is not an AVX-512 execution, regardless
of what the symbol is called.

**Fixture:** `tests/custody/issue27_symbol_identity_trap.json`

---

### Test 2 — Scalar Fallback Injection

**Invariant violated:** 5 (Fallback Prohibition)

**Setup:**
- Receipt claims backend: `avx512`
- Execution shows: `selected_backend: avx512`, `fallback_used: true`,
  `actual_backend: scalar`

**Expected result:** `REJECT`
**Reason:** `FallbackUsed = true ⇒ Status ≠ VERIFIED`
**Catches:** "AVX-512 available but silently skipped." A CI environment that
requests AVX-512, falls back to scalar, and still claims AVX-512 in the
receipt.

**Fixture:** `tests/custody/issue27_fallback_injection.json`

---

### Test 3 — Build Artifact Mismatch

**Invariant violated:** 3 (Build Artifact Identity)

**Setup:**
- Source commit: `5e24faa`
- Receipt claims `artifact_hash: ABC123`
- Observed binary: `artifact_hash: DEF456`

**Expected result:** `REJECT`
**Reason:** `BuildArtifactHash(receipt) ≠ BuildArtifactHash(observed)`
**Catches:** Same source label, different compiled artifact. The placeholder
`avx512_impl::avx512_radix2_butterfly` and the real SIMD function share the
same commit and `target_feature` attribute but produce different codegen.

**Fixture:** `tests/custody/issue27_build_mismatch.json`

---

### Test 4 — Claim Scope Expansion

**Invariant violated:** 6 (Claim Scope Integrity)

**Setup:**
- Evidence proves: `avx512_radix2_butterfly_32` element-wise equivalence
- Claim states: `complete AVX512 NTT backend correctness`

**Expected result:** `REJECT`
**Reason:** `ClaimScope ⊄ VerifiedScope`
**Catches:** Evidence inflation. The butterfly function equivalence does not
establish NTT pipeline correctness. That requires staged equivalence with
its own receipt.

**Fixture:** `tests/custody/issue27_scope_violation.json`

---

### Test 5 — Observer Coupling

**Invariant violated:** 7 (Observation Independence)

**Setup:**
- Verifier obtains execution evidence from the same component producing the
  claim
- The binary says "I executed AVX-512"
- No independent observation

**Expected result:** `REJECT`
**Reason:** `Observer ⊥ Target` violated — observer shares code with target
**Catches:** Self-attestation through the execution trace. The trace becomes
a self-claim: "I executed myself."

**Fixture:** `tests/custody/issue27_observer_coupling.json`

---

### Test 6 — Missing Hardware Probe (Bonus)

**Invariant violated:** 4 (Hardware Presence)

**Setup:**
- Receipt claims backend: `avx512`
- `cpu_feature_verified: false` (or field missing)
- Test silently skipped on non-AVX-512 hardware

**Expected result:** `REJECT`
**Reason:** `CpuFeaturePresent = false` but `ClaimedBackend = avx512`
**Catches:** The `staged_cross_backend_equivalence` pattern — checks
`is_avx512_supported()`, prints "skipping" if false, test passes vacuously.

**Fixture:** `tests/custody/issue27_missing_hardware.json`

---

### Test 7 — Authority Confusion (Bonus)

**Invariant violated:** 1 (Authority Neutrality)

**Setup:**
- Receipt has `authority.granted: true`
- All other fields correct

**Expected result:** `REJECT`
**Reason:** `Authority(r) ≠ ⊥` — a receipt cannot represent permission
**Catches:** A consumer treating execution evidence as authorization to
deploy, ship, or act.

**Fixture:** `tests/custody/issue27_authority_confusion.json`

---

### Test 8 — Positive Control: Real Montgomery AVX Path

**Invariant violated:** None — all nine pass

**Setup:**
- Claim: `avx512_radix2_butterfly_32` equivalence to butterfly_reference oracle
- Target: `avx512_butterfly_32bit.rs`, `avx512_radix2_butterfly_32`, commit `5e24faa`
- Execution: `selected_backend: avx512`, `fallback_used: false`
- Hardware: `cpu_feature_verified: true`, `cpu_model: Intel Xeon Platinum 8480+`
- Observation: disassembly + feature probe + harness isolation
- Scope: `claimed_scope ⊆ verified_scope`

**Expected result:** `ACCEPT`
**Reason:** All nine invariants satisfied
**Catches:** Nothing — this is the positive control. Only this path may
produce `VERIFIED`.

**Required fields:**
```yaml
claim:
  backend: avx512
  domain:
    input: MontgomeryBabyBear
    output: MontgomeryBabyBear

target:
  module: avx512_butterfly_32bit.rs
  symbol: avx512_radix2_butterfly_32

execution:
  selected_backend: avx512
  fallback_used: false

observation:
  disassembly_verified: true
  cpu_feature_probe: true
  harness_isolation: true
```

**Fixture:** `tests/custody/valid_avx512_receipt.json`

---

## Required field chain

```
Claim
  = Target Binding (Invariant 2)
    = Build Artifact (Invariant 3)
      = Execution (Invariants 4, 5)
        = Observation (Invariant 7)
          = Scope (Invariant 6)
            = Authority Neutrality (Invariant 1)
              = Receipt VERIFIED
```

Each link is an equality check. Any inequality → REJECT.

---

## Implementation order

1. **Add test fixtures** (JSON files in `tests/custody/`)
2. **Add verifier** (Python script checking each fixture against all nine invariants)
3. **Run against VERIFICATION_INVARIANTS.md**
4. **All negative cases must REJECT**
5. **Positive control must ACCEPT**
6. **Only after all pass**: proceed to Montgomery scalar == Montgomery AVX-512 and begin performance measurement

---

## Issue #27 closure condition

The Issue #27 closure condition is not "AVX-512 passes." It is:

```
A receipt about the wrong implementation path is now impossible to certify.
```

These tests prove that condition. Each negative test is a receipt that would
have been accepted under the old (pre-custody) model but is now rejected.
The positive test is the only path that produces a valid receipt.

When all negative tests reject and the positive test accepts, Issue #27 is
closed: not because the implementation was proven correct, but because the
custody model now makes it impossible to certify evidence about the wrong
implementation path.
