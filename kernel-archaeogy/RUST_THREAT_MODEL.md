# TSCP Admissibility Kernel — Rust Threat Model Freeze v0.1

**Date:** August 18, 2026
**Purpose:** Define the trusted computing base and excluded mechanisms for the Rust experimental implementation, per Aria review §8 and Johnny's two-stage review.

---

## Trusted Assumptions

These are the assumptions the experiment depends on. They are categorized by nature:

| Assumption | Nature | Justification |
|:---|:---|:---|
| Rust's specified safe-language, type-system, and visibility guarantees | Language semantics | Rust language specification defines what safe code can and cannot do |
| Correctness of the compiler/toolchain (rustc 1.97.1) | Trusted implementation | The compiler is a trusted implementation of the language semantics. If rustc is buggy or compromised, language guarantees may not hold. |
| Compiled module/visibility boundaries | Language guarantee | Module visibility (`pub` vs private) is enforced by rustc. A correct compiler enforces what the specification promises. |
| Correctness of the `admit()` implementation | Kernel logic | The function under test. Its correctness is established by the 27-test suite — but only behaviorally, not by correspondence proof. |

### Key Distinction

> "Safe Rust" is an assumption about the language semantics.
> rustc is a trusted implementation of those semantics.

This distinction prevents conflating language guarantees with implementation correctness.

---

## Explicitly Excluded Mechanisms

The following are OUTSIDE the trust boundary. Each has a precise status:

| Mechanism | Status | What the experiment establishes | What it does NOT establish |
|:---|:---|:---|:---|
| `unsafe` Rust | Excluded | Zero `unsafe` blocks in the crate | If `unsafe` were added, it could bypass the boundary via transmute/reinterpret |
| FFI (`extern "C"`, `#[no_mangle]`, `#[repr(C)]`) | Excluded | No FFI declarations exist in the crate | If FFI were added, foreign bytes could construct the type without re-admission |
| Serialization (serde) | Excluded | No serde dependency, no Serialize/Deserialize impl | If `impl Deserialize for AdmittedEvidence` were added, deserialization would be a second admission path |
| Persistence (database, file I/O) | Excluded | No persistence mechanism exists | If persistence were added without a separate `PersistedAdmissionRecord` type + re-admission, reconstruction could manufacture the type |
| Macros / code generation | Excluded | No macros emit `AdmittedEvidence` | If a proc macro or build script were added that constructs the type, it would bypass `admit()` |
| Type erasure (`dyn Any`) | Partially tested | `downcast_ref::<AdmittedEvidence>()` on an erased `Evidence` value returns `None` | Other erasure paths (trait objects, enums, generic containers) untested |
| Compiler/toolchain compromise | Excluded | N/A | A compromised compiler could generate code that bypasses any type boundary |
| Malicious build tooling | Excluded | N/A | Build script or proc macro could inject construction code |

---

## Scope of the Experiment's Claims

The experiment establishes:

> Within the deliberately constrained crate/API surface, using safe Rust only, with no `unsafe`, no FFI, no serde, no persistence, and no macros that emit `AdmittedEvidence`, the only way to produce an `AdmittedEvidence` value is through `admit()`.

The experiment does NOT establish:

> No mechanism in Rust can bypass the admissibility boundary.

That stronger claim depends on the threat model above, which explicitly excludes `unsafe`, FFI, serialization, persistence, and generated code.

---

## Construction Path Enumeration (Universe Definition)

The "every construction path" claim is bounded by the declared threat model universe:

| Mechanism | In Universe? | Blocked? |
|:---|:---|:---|
| Struct literal (direct field access) | Yes | ✅ Private fields |
| `pub fn new()` associated function | Yes | ✅ Does not exist |
| `Default` trait | Yes | ✅ Not implemented |
| Serde Deserialize | Yes (as excluded) | ✅ No serde dependency |
| `unsafe` transmute | Yes (as excluded) | ✅ No unsafe code |
| FFI construction | Yes (as excluded) | ✅ No FFI declarations |
| Macro-generated construction | Yes (as excluded) | ✅ No macros emit type |
| `admit()` | Yes | THE ONLY PATH |

The universe is defined by the threat model, not "every mechanism Rust could ever provide."

---

## Relationship to the Correspondence Question

The threat model defines what the implementation can and cannot do mechanically.

It does NOT establish that what the implementation does matches the specification.

```
Admission_Rust(e) = Expected(e)     [established by 27 tests]
Admission_Rust(e) ≡ Admission_Spec(e)     [NOT established — HOLD]
```

The threat model is a necessary condition for the correspondence claim, not a sufficient one.

---

## 15-Point Classification Summary

- **8 PASS** — properties that hold regardless of excluded mechanisms
- **5 HOLD (current: PASS)** — hold in current build; could be violated if excluded mechanisms added
- **2 PASS for declared universe** — tested within the explicitly defined threat model
- **0 FAIL**
- **1 HOLD (correspondence)** — not part of the 15 criteria, but the central unresolved question

The experiment validates the proposed boundary under its declared assumptions. It does not validate the boundary against mechanisms explicitly excluded from the threat model, nor does it prove the implementation corresponds to the specification.
