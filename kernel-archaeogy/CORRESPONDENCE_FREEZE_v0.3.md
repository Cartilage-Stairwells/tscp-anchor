# Correspondence Freeze v0.3 — Admissibility Kernel

**Date:** August 19, 2026
**Frozen at:** commit 5645c1f
**Branch:** kernel-archaeology/admissibility-experiment-v0.1
**Repository:** https://github.com/Cartilage-Stairwells/tscp-anchor
**Status:** FROZEN — further capabilities are outside this freeze unless separately specified and adversarially validated.

---

## 1. Established Claims

| # | Claim | Status |
|:---|:---|:---|
| 1 | **Semantic separation** — AdmittedEvidence is an admissibility fact, not a truth or authority fact. | PASS |
| 2 | **Construction enforcement** — Under the frozen safe-Rust model, the intended constructor boundary is mechanically enforced. | PASS |
| 3 | **Specification correspondence** — Within the explicitly defined domain, the Rust predicate and specification predicate were attacked bidirectionally and no remaining divergence was found. | PASS |
| 4 | **Threat-model boundedness** — The claim is explicitly not being generalized to excluded mechanisms or universal system security. | PASS |

---

## 2. Precise Correspondence Statement

The claim is NOT:

> ∀ e ∈ D_spec: Admission_Rust(e) ↔ Admission_Spec(e)

which would require mathematical proof of exhaustive domain coverage.

The claim IS:

> Within the explicitly enumerated and tested admissibility domain D_test ⊆ D_spec, bidirectional interrogation found no divergence between Admission_Rust and Admission_Spec.

D_test is defined by:
- Contracts with canon_version ∈ AcceptedCanonVersions = {"1.0"}
- Evidence with 64-char lowercase hex digests
- Four-role enum (input, output, attestation, witness)
- Three-stage admission (validation → binding → completeness)
- Six attack vectors (false positive, false negative, component predicate divergence, boundary/domain, repaired-seam regression, specification gaps)

If the six attack vectors are shown to exhaustively partition D_spec, the stronger universal formulation may be justified. Until then, the empirical qualification remains.

---

## 3. Evidence Chain

The evidentiary claim is not "95/95 therefore correct." It is:

> Hostile interrogation repeatedly attempted to falsify the correspondence; it found concrete failures; those failures were repaired at the correct layer; the repaired specification and implementation were re-attacked; no remaining divergence was identified within the defined domain.

```
27 tests (initial experiment)
  ↓
3 implementation/specification defects discovered (Aria Round 3)
  ↓
31 regression tests (defects repaired)
  ↓
Round 3b bidirectional interrogation
  ↓
2 specification ambiguities discovered (canon_version, empty evidence)
  ↓
Spec v0.3 amendments (A & B — spec clarified, not implementation guessing)
  ↓
Round 3c amendment attack
  ↓
1 test-vector defect discovered and corrected (C.8 wrong artifact_type)
  ↓
95/95 (31 regression + 38 Round 3b + 26 Round 3c)
  ↓
CORRESPONDENCE = PASS
```

The sequence of discovered defects → repairs → re-attacks is part of the evidence. Do not squash.

---

## 4. Fundamental Firewall

```
Evidence
   │
   │  forbidden
   X────────────────→ Decision
   │
   ▼
admit()
   │
   ▼
AdmittedEvidence
   │
   ▼
evaluate()
   │
   ▼
Decision
```

Semantic distinction (the conceptual heart of the kernel):

```
admissible  ≠  true  ≠  correct  ≠  authoritative
```

This distinction is permanently visible in the specification (§4 semantic non-implications) and in the implementation (type-level enforcement: AdmittedEvidence has no authority fields, cannot be recycled as Evidence, cannot bypass admit()).

---

## 5. Freeze Disposition

| Item | Status |
|:---|:---|
| Admissibility ≠ truth | FROZEN |
| Admissibility ≠ authority | FROZEN |
| Evidence cannot bypass admit() | FROZEN under threat model |
| Rust/spec predicate correspondence | PASS within bounded domain D_test ⊆ D_spec |
| Duplicate admission | QUARANTINED — spec defines error code, no mechanism. Implementation makes no claim. |
| admitted_at format | DEFERRED — static string vs RFC 3339 clock. Doesn't affect determinism or decision. |
| Universal security | NOT CLAIMED |

---

## 6. Frozen Threat Model

The claim is bounded by:

- **Safe Rust only** — no unsafe, no FFI, no reflection
- **No serde** — serialization cannot manufacture AdmittedEvidence
- **No persistence** — no state survives between calls
- **No network** — pure function, no side effects
- **Language semantics** — rustc is the trusted compiler implementation; safe Rust language semantics are the trusted computing base

Excluded from this freeze:
- Serialization-based construction attacks
- Compiler compromise
- Persistence/state corruption
- FFI bypass
- Custody transitions
- Duplicate admission detection
- Network-level attacks

Any future capability that introduces state, persistence, serialization, duplicate detection, or custody transitions gets its own scope and gate.

---

## 7. Artifact Inventory

| File | Purpose |
|:---|:---|
| `ADMISSIBILITY_CONTRACT_SPEC.md` | Specification v0.3 (amended with A & B) |
| `kernel/src/lib.rs` | Rust implementation (admit function + types) |
| `kernel/src/tests.rs` | 31 regression tests (6 property categories) |
| `kernel/src/round3b_tests.rs` | 38 Round 3b bidirectional tests |
| `kernel/src/round3c_tests.rs` | 26 Round 3c amendment attack tests |
| `ARIA_ROUND3B_FINDINGS.md` | Round 3b findings (69/69, HOLD, 2 ambiguities) |
| `ARIA_ROUND3C_FINDINGS.md` | Round 3c findings (95/95, PASS) |
| `SPEC_AMENDMENTS_v0.3.md` | Amendment A (AcceptedCanonVersions) & B (Evidence \| null) |
| `RUST_THREAT_MODEL.md` | Frozen threat model definition |
| `VALIDATION_RESULTS.txt` | 95/95 test output |

---

## 8. Commit History (Do Not Squash)

```
10fc8d0  initial experiment (27 tests)
   ↓
a8f38b1  Aria R3 fixes (31 tests)
   ↓
3f7be4c  Round 3b attack framework
   ↓
93f49a3  Round 3b: 69/69, HOLD (2 ambiguities)
   ↓
39bedde  Spec v0.3: Amendments A & B
   ↓
5645c1f  Round 3c: 95/95, PASS ← FREEZE BOUNDARY
```

Each commit represents a discovery or repair in the evidence chain. The history IS the evidence.

---

## 9. Boundary Statement

> Admissibility correspondence established. Further capabilities are outside the freeze unless separately specified and adversarially validated.

This freeze establishes that the Rust implementation faithfully instantiates the specified admissibility predicate over the explicitly defined domain D_test, under the frozen safe-Rust threat model.

It does NOT establish:
- Universal correctness of the admissibility predicate
- Security against excluded attack vectors
- Completeness of the domain partition
- Authority of admissible evidence

The next change — whether duplicate admission, custody transitions, serialization, or state — requires its own specification, its own threat model, and its own adversarial validation gate.
