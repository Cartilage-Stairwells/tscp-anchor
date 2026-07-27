# Verification Custody Invariants

**Document type:** Formal invariant definitions
**Status:** v2.0 (post-merge canonical)
**Motivating case:** Issue #27 — Verification Surface Drift
**Date:** 2026-07-26

---

## Purpose

This document formalizes the invariants that the verification custody model
enforces. Each invariant is a property that must hold for a receipt to be
valid. A receipt violating any invariant is invalid, even if tests pass,
hashes match, and outputs are correct.

These invariants were derived from the failure taxonomy established after
Issue #27 exposed that a receipt could claim AVX-512 backend coverage while
the cited test exercised only a scalar delegate wrapper.

---

## Failure taxonomy

| Class | Broken relationship | Core question |
|---|---|---|
| False Shoreline | Evidence scope → insufficient | Did the evidence cover enough? |
| Self-Attestation | Verifier → independence | Did the verifier depend on the thing it verified? |
| Authority Confusion | Evidence → authority boundary | Did the evidence overstep what it can prove? |
| Provenance Gap | Artifact → history | Can we trace the artifact's full history? |
| Semantic Drift | Implementation → mathematical meaning | Did the implementation compute the right mathematical object? |
| Verification Surface Drift | Claim → exercised execution path | Did the evidence actually touch the thing it claims to describe? |

**Key distinction:** Semantic Drift and Verification Surface Drift are adjacent
but not identical.

- **Semantic Drift**: The execution path is real, the implementation is
  reached, but it computes the wrong mathematical object.
- **Verification Surface Drift**: The execution path itself is wrong. The
  claimed target was never reached. The mathematical question was never
  asked.

The dependency ordering is:

```
Target Binding → Execution → Semantic Verification
```

A semantic oracle cannot detect a drift if it never reaches the target
implementation. This is not a limitation of the oracle — it is a precondition
that was never satisfied.

---

## Invariant ordering

Invariants are organized in four layers, mirroring the causal chain:

```
Layer 1 — Identity
    1. Authority Neutrality
    2. Target Binding
    3. Build Artifact Identity

Layer 2 — Execution
    4. Hardware Presence
    5. Fallback Prohibition
    6. Claim Scope Integrity

Layer 3 — Observation
    7. Observation Independence

Layer 4 — Governance
    8. Gate Ordering
    9. Receipt Lifecycle Integrity
```

Each layer depends on the previous layer. Identity must be established
before execution can be verified. Execution must be verified before
observation can be meaningful. Observation must be independent before
governance can be enforced.

---

## Layer 1 — Identity

### Invariant 1: Authority Neutrality

**Statement:**

```
∀r: Authority(r) = ⊥
```

**Where:**
- `r` is a verification receipt
- `Authority(r) = ⊥` means the receipt grants no authority, permission, or
  authorization

**A receipt is invalid if:**
- It is consumed as authorization to deploy, ship, or act
- It crosses an authority boundary (e.g. from evidence to permission)
- Its schema implies that verification evidence constitutes approval

**Motivation:** The previous FCO boundary work established that Receipt ≠
Permission. Execution evidence is not an authorization artifact. A consumer
treating a receipt as permission to act is overstepping its authority
boundary. This invariant makes the boundary explicit in the schema, not
merely in prose.

This changes the rule from "users should understand receipts are not
permissions" to "a receipt object cannot represent permission." The latter
is a stronger boundary, and it holds through composition: no sequence of
receipt operations produces an authorization.

```
Receipt ↛ Authority
```

**Artifact:** Enforced by `authority.granted: false` and
`authority.jurisdiction_crossed: false` in both
`IMPLEMENTATION_TARGET_BINDING.md` and `EXECUTION_TRACE_RECEIPT.md`.

---

### Invariant 2: Target Binding

**Statement:**

```
∀r: (ClaimedTarget(r), ClaimedBackend(r)) = (ExecutedTarget(r), SelectedBackend(r))
```

**Where:**
- `r` is a verification receipt
- `ClaimedTarget(r)` is the implementation symbol the receipt claims to verify
- `ClaimedBackend(r)` is the execution backend the receipt claims was used
- `ExecutedTarget(r)` is the implementation symbol that was actually invoked
- `SelectedBackend(r)` is the execution backend that was actually selected

**A receipt is invalid if:**

```
ClaimedTarget(r) ≠ ExecutedTarget(r)   (target mismatch)
  OR
ClaimedBackend(r) ≠ SelectedBackend(r)  (backend mismatch)
```

**Even if:**
- Tests pass
- Hashes match
- Outputs match
- The wrapper is deterministic

**Motivation (Issue #27):** The receipt claimed `backends: ["scalar", "avx512"]`
for `butterfly.oracle_equivalence`. But the cited test called `butterfly()` →
`scalar_butterfly_32()`. `ClaimedBackend = avx512`, `SelectedBackend = scalar`.
The invariant is violated. The receipt is invalid.

**Why the backend dimension is necessary:** `butterfly()` and
`avx512_radix2_butterfly_32` are both "the butterfly" — same conceptual
target. A symbol-identity check alone passes. But one runs scalar arithmetic
and the other runs 16-lane SIMD. The backend dimension is what catches the
fallback. Without it, a wrapper that delegates to scalar is a valid target
match because the scalar function is technically "the butterfly" that got
executed.

**Artifacts:** Enforced by `IMPLEMENTATION_TARGET_BINDING.md` (target identity)
and `EXECUTION_TRACE_RECEIPT.md` (execution verification).

---

### Invariant 3: Build Artifact Identity

**Statement:**

```
∀r: BuildArtifactHash(r) = Hash(Source, Compiler, Flags, Features, TargetCPU, Dependencies)
```

**Where:**
- `Source` is the git commit SHA
- `Compiler` is the compiler version
- `Flags` are the compilation flags
- `Features` are the CPU features enabled
- `TargetCPU` is the target triple
- `Dependencies` is the dependency lock file hash

**A receipt is invalid if:**
- Only the source commit is pinned but not the build artifact
- The toolchain or flags are not recorded
- The dependency lock hash is missing
- Two different compilations of the same commit could produce different codegen

**Motivation:** A git commit SHA pins the source, but not the compiled
artifact. Different compilers, flags, or LLVM versions can produce different
machine code from the same commit. Without the full build identity, Lean can
prove a theorem about one compilation of a commit while production runs another.

Same commit SHA does not imply same binary artifact because:
- compiler versions change,
- dependencies change,
- feature flags change,
- target features change.

The artifact hash closes that gap.

In the Issue #27 case, `lib.rs::avx512_impl::avx512_radix2_butterfly` is
decorated with `#[target_feature(enable="avx512f,avx512dq")]` but delegates to
`scalar_radix2_butterfly`. The source and target_feature attribute are
identical to the real SIMD function, but the compiled codegen is radically
different — one has `vpmullq` instructions, the other doesn't. The build
artifact hash distinguishes them.

The dependency lock hash is necessary because two compilations of the same
commit with different dependency versions could produce different codegen.
The build identity must explain why the produced binary is the one being
observed.

**Artifact:** Enforced by `IMPLEMENTATION_TARGET_BINDING.md` field
`build_identity` (with `source_commit`, `compiler`, `target_triple`,
`cpu_features`, `cargo_features`, `dependency_lock`, `flags`,
`artifact_hash`).

---

## Layer 2 — Execution

### Invariant 4: Hardware Presence

**Statement:**

```
∀r: ClaimedBackend(r) = avx512 ⇒ CpuFeaturePresent(r)
```

**Where:**
- `CpuFeaturePresent(r)` means the CPU feature required by the claimed
  backend was verified present at test execution time

**A receipt claiming an AVX-512 backend is invalid if:**
- The test silently skipped because `is_avx512_supported()` returned false
- The CPU feature was not verified at test time
- The feature probe is missing from the receipt

**Motivation (Issue #27):** `staged_cross_backend_equivalence` checks
`is_avx512_supported()` and prints "skipping" if false. On non-AVX-512
hardware (including most CI runners), the test passes vacuously. A receipt
generated from such a run claims AVX-512 coverage that was never exercised.

**Artifact:** Enforced by `EXECUTION_TRACE_RECEIPT.md` field
`hardware.cpu_feature_verified: true`.

---

### Invariant 5: Fallback Prohibition

**Statement:**

```
∀r: ClaimedBackend(r) = avx512 ⇒ ExecutedBackend(r) = avx512
∀r: FallbackUsed(r) = true ⇒ ReceiptStatus(r) ≠ VERIFIED
```

**Where:**
- `ExecutedBackend(r)` is the backend that was actually selected at runtime
- `FallbackUsed(r)` is true if the target backend was unavailable and a
  fallback was used instead

**A receipt is invalid if:**
- The claimed backend was requested but a fallback was used
- The CI environment silently produced: requested avx512, executed scalar,
  receipt claims avx512

**Motivation (Issue #27):** This is the most Issue #27-specific invariant.
The core failure was: `butterfly()` was labeled as the avx512 backend in the
receipt, but it delegates to `scalar_butterfly_32()`. The fallback was not
explicit — it was baked into the function. This invariant ensures that even
if the fallback is silent (embedded in a wrapper), the receipt cannot claim
the higher backend.

Without this invariant, a CI environment could silently produce:
```
requested: AVX-512
executed: scalar
receipt says: AVX-512
```
which is exactly the failure class being eliminated.

The receipt must fail closed. If the backend cannot be executed, no receipt
is generated for that backend — not a receipt with a fallback masquerading
as the higher backend.

**Artifact:** Enforced by `EXECUTION_TRACE_RECEIPT.md` fields
`execution.fallback_used: false` and `fallback_policy.forbidden: true`.

---

### Invariant 6: Claim Scope Integrity

**Statement:**

```
∀r: ClaimScope(r) ⊆ VerifiedScope(r)
```

**Where:**
- `ClaimScope(r)` is the property the receipt claims to establish
- `VerifiedScope(r)` is the property the evidence actually establishes
- `⊆` denotes that the claimed scope must be a subset of (no broader than)
  the verified scope

**A receipt is invalid if:**

```
ClaimScope(r) ⊄ VerifiedScope(r)   (scope overclaim)
```

**Even if:**
- The target symbol was correctly identified and executed
- The backend was correctly selected
- The tests passed
- The hardware was present

**Motivation:** The Issue #27 correction verifies that `Target = Executed`.
But there is still a remaining possibility: the executed target is
`avx512_radix2_butterfly_32`, the claimed target is "entire AVX512 NTT
backend," and the symbol matches. The claim scope is larger than the
evidence.

This is the same pattern as Evidence Boundary ≠ Authority Boundary, but
applied to claim granularity:

```
Valid:
  claim: avx512_radix2_butterfly_32 equivalence
  evidence: tested avx512_radix2_butterfly_32 against butterfly_reference

Invalid:
  claim: entire AVX512 NTT backend correctness
  evidence: tested avx512_radix2_butterfly_32 against butterfly_reference
  (scope overclaim — NTT pipeline correctness requires staged equivalence)
```

**A receipt cannot claim a broader property than the executed evidence
establishes.** The claim scope must be a subset of the verified scope.

**Artifact:** Enforced by `EXECUTION_TRACE_RECEIPT.md` fields
`claim_scope.claimed_scope`, `claim_scope.verified_scope`, and
`claim_scope.scope_valid: true`. Also by `IMPLEMENTATION_TARGET_BINDING.md`
field `claim.scope`.

---

## Layer 3 — Observation

### Invariant 7: Observation Independence

**Statement:**

```
∀r: Observer(r) ⊥ Target(r)
```

**Where:**
- `Observer(r)` is the mechanism that produced the execution trace
- `Target(r)` is the implementation being verified
- `⊥` denotes structural independence (no shared code path)

**A receipt is invalid if:**
- The binary being tested produces its own execution trace
- The observer shares implementation code with the target
- The observation method is the code under test itself

**Observation ladder:**

The required minimum is all three of:

1. **Disassembly** (`cargo asm`) — proves the compiled code IS SIMD.
   Does NOT prove it was executed.
2. **Feature probe** (CPUID check logged externally) — proves the hardware
   was present at test time. Does NOT prove the specific code path was taken.
3. **Harness isolation** — the test module imports the target but shares no
   implementation code. Confirms the test doesn't depend on the target's
   internal implementation.

Supporting evidence (cannot substitute for the minimum):

4. **Performance counters** (`perf stat -e avx512f_inst_retired`) — confirms
   SIMD instructions were executed during the test run. **Not proof of
   identity.** AVX instructions may execute in unrelated code. A scalar path
   may produce similar timing. Performance counters confirm execution
   characteristics, not execution identity. Useful as corroboration, not as
   proof.

The key invariant:

```
Timing ≠ Identity
```

A fast result does not prove the intended backend executed. Performance
counters alone should not be treated as proof. They are evidence of execution
characteristics, not identity.

**Motivation:** Without observation independence, the custody model
reintroduces Self-Attestation through the execution trace. The trace becomes
a self-claim: "I executed myself." This defeats the purpose of the execution
binding gate.

The strongest minimum remains:

```
Disassembly + Feature Probe + Harness Isolation
```

Performance counters are supporting evidence, not a substitute for any of
the three.

**Artifact:** Enforced by `EXECUTION_TRACE_RECEIPT.md` field
`observation.independence_note` and `observation.method`.

---

## Layer 4 — Governance

### Invariant 8: Gate Ordering

**Statement:**

```
Gate_i must pass before Gate_{i+1} can be evaluated
```

**Gate chain:**

```
Implementation Identity Gate  → (IMPLEMENTATION_TARGET_BINDING.md is ACTIVE)
        |
        v
Execution Binding Gate       → (EXECUTION_TRACE_RECEIPT.md is ACTIVE)
        |
        v
Domain Equivalence Gate      → (inputs/outputs confirmed in Montgomery domain)
        |
        v
Backend Parity Gate          → (SIMD output == scalar output, on real hardware)
        |
        v
Semantic Reference Gate      → (output == reference oracle, oracle is independent)
        |
        v
Formal Theorem               → (Lean proof, only after all gates pass)
        |
        v
Optimization Receipt         → (performance evidence, post-verification)
```

**Motivation:** Formalization (Lean) must not begin until the implementation
path is frozen and the execution binding is established. Otherwise Lean can
produce a perfectly valid proof about an abstraction that does not correspond
to production execution.

In the Issue #27 case, `FORMAL_PROOF_SURFACE.md` lists `simd_matches_scalar`
as a sorry-placeholder: `∀ a b w, simd_butterfly(a,b,w) = scalar_butterfly(a,b,w)`.
If someone fills in that sorry, they'd be proving that the placeholder
`avx512_radix2_butterfly` (which delegates to scalar) equals scalar. The
theorem is trivially true. It proves nothing about the real SIMD
implementation. The gate chain prevents this.

---

### Invariant 9: Receipt Lifecycle Integrity

**Statement:**

```
A SUPERSEDED receipt cannot be re-activated.
A REVOKED receipt cannot be re-activated.
A new receipt must be created with an explicit reference to the superseded or revoked one.
```

**Lifecycle:**

```
GENERATED → AUDITED → ACTIVE → { SUPERSEDED | REVOKED }
```

| State | Meaning |
|---|---|
| FAILED | Artifact generation did not complete (test crashed, could not execute) |
| GENERATED | Artifact was produced by a test run with observation |
| AUDITED | An independent reviewer confirmed the observation method is independent |
| ACTIVE | The receipt is the current authority for its claim |
| SUPERSEDED | Valid artifact replaced by a newer valid artifact |
| REVOKED | Produced claim was later determined to be invalid |

**SUPERSEDED ≠ REVOKED:**

- **SUPERSEDED** means "replaced by a newer valid artifact." The artifact
  was correct; a newer one now holds authority.
- **REVOKED** means "invalidated due to discovered defect." The artifact's
  claim was withdrawn because it was false.

Issue #27 belongs in REVOKED, not SUPERSEDED, because the problem was not
age or replacement. The claim itself was unsupported. That distinction
preserves historical accuracy.

**Why not "FAILED":** A receipt that discovers a target mismatch should NOT
be marked "FAILED" because the artifact itself was generated successfully.
The test ran, the observation was recorded, the receipt was produced. What
failed is the *claim*, not the *artifact*. The historical record proves: "A
verification artifact existed and was later found insufficient." That is
useful custody information.

The distinction:
- **FAILED**: The test crashed or could not execute. The artifact was not produced.
- **REVOKED**: The artifact was produced, audited, and later found to have a false claim.
- **SUPERSEDED**: The artifact was produced, valid, and replaced by a newer one.

**Motivation (Issue #27):** The receipt at commit `0205722` was
GENERATED and AUDITED. Issue #27 discovered the target mismatch. The
receipt should be REVOKED — its claim was withdrawn, not merely replaced.

---

## Composite invariant

All nine invariants compress to a single principle:

```
┌──────────────────────────────────────────────────────────────────┐
│  Evidence must bind what code, what binary, what execution,       │
│  what claim, and what evidence — and must not be confused         │
│  with authority.                                                  │
└──────────────────────────────────────────────────────────────────┘
```

The mature verification chain is therefore:

```
┌──────────────────────────────────────────────────────────────────┐
│  Specification                                                    │
│    → Implementation Identity                                      │
│    → Execution Binding                                            │
│    → Semantic Verification                                         │
│    → Conformance Evidence                                         │
│    → Receipt                                                      │
│    (Receipt ≠ Permission)                                         │
└──────────────────────────────────────────────────────────────────┘
```

Each stage requires the previous stage to have passed. No stage can be
skipped. No stage can be evaluated out of order. The receipt is the terminal
evidence artifact, not an authorization artifact.

The architecture has moved from "prove the code" to the more precise model:

```
┌──────────────────────────────────────────────────────────────────┐
│  Prove what code, what binary, what execution,                    │
│  what claim, and what evidence.                                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Issue #27 as validation event

Issue #27 is a strong validation event for the custody model. The system
has now exposed two different "wrong object" failures:

**DIF butterfly (Semantic Drift):**
```
Correct verification path + wrong mathematical meaning
```

**AVX-512 gate (Verification Surface Drift):**
```
Correct verification procedure + wrong execution target
```

Together they reveal the deeper invariant: evidence must bind both meaning
and execution identity. Keeping the AVX-512 equivalence gate blocked is the
correct custody action. The architecture is doing exactly what it should:
preventing a valid-looking receipt from crossing a boundary it has not
actually proven.

---

## Review questions

Before freezing these schemas, the following questions should be answered
against Issue #27:

1. **Can the verifier detect scalar fallback?**
   — Yes, via Invariant 5 (Fallback Prohibition) and `execution.fallback_used`.

2. **Can it distinguish wrapper from implementation?**
   — Yes, via Invariant 2 (Target Binding) with backend dimension and
   `build_identity.artifact_hash`.

3. **Can it bind binary artifact to source?**
   — Yes, via Invariant 3 (Build Artifact Identity) with full build identity
   including dependency lock hash.

4. **Can it prevent stale receipt activation?**
   — Yes, via Invariant 9 (Receipt Lifecycle Integrity) — REVOKED and
   SUPERSEDED receipts cannot be re-activated.

5. **Can it prevent execution evidence from being treated as authorization?**
   — Yes, via Invariant 1 (Authority Neutrality) and `authority.granted: false`.

6. **Can it prevent performance counters from being mistaken for identity proof?**
   — Yes, via Invariant 7 (Observation Independence) — performance counters
   are explicitly classified as supporting evidence only. Timing ≠ Identity.

7. **Can it distinguish "replaced" from "withdrawn"?**
   — Yes, via the SUPERSEDED vs REVOKED lifecycle distinction in Invariant 9.

8. **Can it detect scope overclaim — claiming a broader property than the evidence establishes?**
   — Yes, via Invariant 6 (Claim Scope Integrity) — `ClaimScope ⊆ VerifiedScope`.

9. **Can the verifier distinguish symbol identity, binary identity, execution identity, and claim scope?**
   — Yes: symbol identity (Invariant 2), binary identity (Invariant 3),
   execution identity (Invariants 4+5), claim scope (Invariant 6). All four
   are separate checks; passing one does not imply passing another.
