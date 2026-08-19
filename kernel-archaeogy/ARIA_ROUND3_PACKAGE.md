# TSCP Admissibility Kernel — Round 3 Package for Aria

**Date:** August 18, 2026
**Subject:** Rust experimental implementation + revised classification per Johnny's two-stage review
**Prior:** ADMISSIBILITY_CONTRACT_SPEC v0.2 + Aria Review 2 (ARIA_REVIEW_2.md)
**Status:** READY FOR JOHNNY REVIEW — DO NOT SEND UNTIL APPROVED

---

## The Central Question

The Round 3 question is not whether a private constructor exists in Rust.

It is:

> **Does Rust `admit()` preserve the meaning of the canonical admissibility specification?**

Formally:

```
Admission_Rust(e) ≡ Admission_Spec(e)     [correspondence — UNPROVEN]
```

The 27-test suite establishes something weaker:

```
Admission_Rust(e) = Expected(e)          [behavioral agreement on tested cases]
```

Those are not equivalent claims. A sufficiently malicious or mistaken implementation could pass all 27 tests while implementing:

```
Spec predicate + unnoticed extra acceptance condition
```

or:

```
Spec predicate - unnoticed rejection condition
```

The tests catch known counterexamples. They do not by themselves prove semantic equivalence.

This is exactly the custody problem: a faithful-looking implementation can agree with test observations while still requiring an explicit contract-preservation argument.

**Specification ↔ Rust correspondence: HOLD.** This is the centerpiece of Round 3.

---

## What Changed Since Aria's Round 2

1. **Rust implementation** of `admit()` as pure protocol logic — `tscp_delta/kernel/src/lib.rs`
2. **27-test suite** exercising 6 property categories — `tscp_delta/kernel/src/tests.rs`
3. **Rust threat model frozen** — `tscp_delta/RUST_THREAT_MODEL.md`
4. **Revised classification** — two-stage review by Johnny applied: "no mechanism exists" ≠ "mechanism is impossible"

---

## Three-Level Claim Structure

| Level | Claim | Status |
|:---|:---|:---|
| Specification | AdmittedEvidence means the admission contract was satisfied. AdmittedEvidence ≠ true, ≠ correct, ≠ authentic, ≠ authoritative. | ✅ Specified in ADMISSIBILITY_CONTRACT_SPEC v0.2 |
| Rust implementation | Safe Rust mechanically represents/enforces the distinction between admissible and non-admissible evidence under the declared threat model. | ✅ 27/27 tests pass across 6 property categories |
| Security guarantee | No general security guarantee is claimed. | ❌ Not claimed. Depends on declared threat model + correspondence argument. |

### Semantic Firewall (preserved from spec)

```
AdmittedEvidence
    ≠ true
    ≠ correct
    ≠ authentic
    ≠ authoritative
```

This is consistent with the custody model: an implementation inherits meaning from its specification through an explicit contract; agreement with observed outputs is not itself the custody relation.

---

## Test Properties (Not Test Counts)

The 27 tests exercise 6 property categories. The properties are the argument; the count is supporting evidence.

### Property 1: Positive Admission

```
valid structure + correct binding + complete schema → ACCEPT
```

The system admits evidence that satisfies the complete admission contract. Tests: `t_valid_admission`, `t_determinism`, `c1_evidence_cannot_enter_evaluate`.

### Property 2: Validation Rejection

```
malformed structure → REJECT (at VALIDATION stage)
```

Evidence with invalid digest format (not 64-char lowercase hex) or empty artifact_type is rejected before binding is even evaluated. Tests: `t_invalid_structure_valid_binding`, `t_empty_artifact_type`, `t_invalid_contract`.

### Property 3: Binding Rejection

```
wrong specification/type/role binding → REJECT (at BINDING stage)
```

Evidence whose artifact_type or role is not in the contract's admissible set is rejected. Duplicate digests are rejected. Tests: `t_type_rejection`, `t_role_rejection`, `t_duplicate_digest`.

### Property 4: Completeness Rejection

```
missing required evidence → REJECT (at COMPLETENESS stage)
```

Evidence that satisfies validation and binding but is insufficient in count, exceeds the maximum, or is missing a required role is rejected. Tests: `t_insufficient_evidence`, `t_excess_evidence`, `t_missing_required_role`.

### Property 5: Semantic Firewall

```
structurally valid fabricated evidence → may be admissible
→ does NOT thereby become true/correct/authoritative
```

Evidence with fabricated digests (`deadbeef...`, `cafef00d...`) is admitted because it is structurally valid. The system does NOT do:

```
digest looks plausible → therefore true → admit
```

It does:

```
digest satisfies structural contract → admit
```

This is the most important behavioral result. Admission must not be secretly defined as an authenticity oracle. Tests: `c10_validation_structural`, `t_semantic_laundering`, `c11_binding_association`, `c12_completeness_schema_relative`.

### Property 6: Construction Boundary

```
external safe Rust → cannot directly construct AdmittedEvidence
```

Within the declared threat model (safe Rust, no unsafe, no FFI, no serde, no persistence, no macros), the only way to produce an AdmittedEvidence value is through `admit()`. Tests: `c2_no_external_construction`, `c3_construction_paths`, `c4_no_serialization_construction`, `c5_no_persistence_construction`, `c6_no_ffi`, `c7_no_unsafe`, `c8_no_macro_construction`, `c9_type_erasure`.

---

## Rust Threat Model (Frozen)

### Trusted Assumptions

| Assumption | Nature |
|:---|:---|
| Rust's specified safe-language, type-system, and visibility guarantees | Language semantics assumption |
| Correctness of the compiler/toolchain (rustc 1.97.1) | Trusted implementation of language semantics |
| Compiled module/visibility boundaries | Language guarantee enforced by trusted compiler |
| Correctness of the `admit()` implementation | Kernel logic — the thing under test |

### Explicitly Excluded

| Mechanism | Status | What the experiment establishes | What it does NOT establish |
|:---|:---|:---|:---|
| `unsafe` Rust | Excluded | Zero `unsafe` blocks in the crate | If `unsafe` were added, it could bypass the boundary via transmute |
| FFI (`extern "C"`, `#[repr(C)]`) | Excluded | No FFI declarations exist | If FFI were added, foreign bytes could construct the type |
| Serialization (serde) | Excluded | No serde dependency, no Serialize/Deserialize impl | If `impl Deserialize` were added, deserialization would be a second admission path |
| Persistence | Excluded | No persistence mechanism exists | If added without `PersistedAdmissionRecord` + re-admission, could bypass |
| Macros / code generation | Excluded | No macros emit `AdmittedEvidence` | If a proc macro or build script were added that constructs the type, it would bypass |
| Type erasure (`dyn Any`) | Partially tested | `downcast_ref` on erased `Evidence` returns `None` | Other erasure paths (trait objects, enums, generic containers) untested |
| Compiler/toolchain compromise | Excluded | N/A | A compromised compiler could bypass any type boundary |

### Key Distinction

> "Safe Rust" is an assumption about the language semantics. rustc is a trusted implementation of those semantics.

This distinction prevents conflating language guarantees with implementation correctness.

---

## Revised 15-Point Classification

### PASS (8) — properties that hold in the declared configuration and do not depend on excluded mechanisms

1. **Evidence cannot directly enter evaluate()** — PASS. Type system enforces. No function accepts Evidence and produces a Decision.

2. **AdmittedEvidence cannot be constructed externally** — PASS. All fields private. No pub constructor. No Default. Safe-Rust visibility guarantee.

7. **Unsafe facilities are explicitly outside TCB** — PASS. Zero `unsafe` blocks. TCB explicitly excludes `unsafe`. Statement: "the type invariant holds for safe code under Rust's safety model."

10. **Validation is structural only** — PASS. Fabricated digests admitted because structurally valid. Behaviorally confirms: admissible ≠ true.

11. **Binding is association, not endorsement** — PASS. Evidence with correct type/role admitted regardless of artifact genuineness.

12. **Completeness is schema-relative** — PASS. Evidence satisfying schema requirements admitted. Not epistemic completeness.

13. **No authority semantics in AdmittedEvidence** — PASS. Fields: contract_id, contract_version, evidence, admitted_at, admission_digest. None express authority.

14. **Authority is downstream only** — PASS. `admit()` returns AdmittedEvidence, not Authority or Decision.

15. **Specification meaning preserved** — PASS. Error codes match spec. Implementation follows spec types, stages, and semantics.

### HOLD (current: PASS) (5) — hold in current build; could be violated if excluded mechanisms were added

4. **Serialization cannot manufacture admission** — HOLD (current: PASS). No serde impl in this build. **Current crate: PASS. General Rust: HOLD.**

5. **Persistence cannot manufacture admission** — HOLD (current: PASS). No persistence mechanism exists. **Current crate: PASS. General: HOLD.**

6. **FFI cannot manufacture admission** — HOLD (current: PASS). No FFI declarations exist. **Current crate: PASS. General: HOLD.**

8. **Generated code cannot introduce unauthorized constructor** — HOLD (current: PASS). No macros emit AdmittedEvidence. **Current crate: PASS. General mechanism: HOLD.**

9. **Type erasure cannot bypass re-admission** — HOLD (current: PASS for tested path). Only `dyn Any` erasure tested. **Tested mechanism: PASS. Broader category: HOLD.**

### PASS for declared universe (2) — tested within the explicitly defined threat model

3. **Every construction path enumerated** — PASS for declared universe. 8 mechanisms checked within the declared threat model. All blocked except `admit()`. The universe is defined by the threat model, not "every mechanism Rust could ever provide."

7. **Unsafe boundary explicit** — PASS. Also listed under PASS above. The TCB boundary is declared, not inferred.

---

## The Correspondence Gap

The 27 tests establish:

```
Admission_Rust(e) = Expected(e)     for tested cases
```

They do NOT establish:

```
Admission_Rust(e) ≡ Admission_Spec(e)     [semantic equivalence]
```

The difference:

- A malicious implementation could pass all 27 tests while implementing `Spec predicate + unnoticed extra acceptance condition`
- A mistaken implementation could pass all 27 tests while implementing `Spec predicate - unnoticed rejection condition`
- The tests catch known counterexamples; they do not prove the Rust predicate IS the canonical predicate

This is exactly the custody problem: agreement with observed outputs is not the custody relation. An explicit contract-preservation argument is needed.

**Specification ↔ Rust correspondence: HOLD pending stronger evidence.**

---

## Questions for Aria Round 3

### Question 1 (Construction Boundary)
Under the frozen safe-Rust threat model, is the Evidence → admit() → AdmittedEvidence construction boundary mechanically defensible?

### Question 2 (Semantic Firewall)
Does any specified or tested path cause AdmittedEvidence to acquire truth, correctness, authenticity, or authority semantics?

### Question 3 (Stage Semantics)
Are Validation, Binding, and Completeness semantically no stronger than the canonical admissibility specification?

### Question 4 (Correspondence — CENTERPIECE)
Does the Rust implementation implement the canonical admissibility predicate itself, or only a behaviorally compatible approximation demonstrated by the current tests? What evidence would distinguish those two claims?

### Question 5 (Minimal Remaining Assumption)
What is the smallest remaining assumption preventing the Rust implementation from being treated as a verified custody boundary?

---

## Final Disposition

```
Specification:                              PASS
Semantic firewall:                         PASS
Rust implementation:                       PASS under declared threat model
27-test conformance suite:                 PASS across 6 property categories
General security guarantee:                NOT CLAIMED
Specification ↔ implementation correspondence: HOLD pending stronger evidence
Next review:                               Rust-specific adversarial red team
```

### The Review Frontier

```
Round 1: Architecture
    ↓
Round 2: Admissibility semantics
    ↓
Minimal Rust implementation
    ↓
Round 3: Rust-specific adversarial review
    ↓
Specification ↔ implementation correspondence     ← WE ARE HERE
```

The thing Aria should try to break now is not the existence of a private constructor. It is the proposition:

```
Rust admit() preserves the meaning of the canonical admissibility specification.
```

If that survives hostile review, the kernel will have crossed a much more significant boundary than merely demonstrating that a Rust type is difficult to construct incorrectly.

---

## File Inventory

| File | Content |
|:---|:---|
| `tscp_delta/ADMISSIBILITY_CONTRACT_SPEC.md` | Specification v0.2 (updated per Aria Round 2) |
| `tscp_delta/ARIA_REVIEW_2.md` | Aria's red-team review |
| `tscp_delta/RUST_THREAT_MODEL.md` | Rust threat model freeze |
| `tscp_delta/kernel/src/lib.rs` | Rust implementation (pure protocol logic, zero deps) |
| `tscp_delta/kernel/src/tests.rs` | 27-test suite (6 property categories) |
| `tscp_delta/kernel/Cargo.toml` | Package manifest (no dependencies) |
| `tscp_delta/VALIDATION_RESULTS.txt` | Raw test output (27/27 pass) |

---

## Note on Independent Verification

The Round 3 material was produced in Lyra's sandbox. Johnny has reviewed the classification and corrected overclaims. The Rust threat model, test properties, and correspondence gap framing reflect Johnny's two-stage review. The implementation and test suite are available for Aria to inspect directly if Johnny provides access to the sandbox or transfers the files.
