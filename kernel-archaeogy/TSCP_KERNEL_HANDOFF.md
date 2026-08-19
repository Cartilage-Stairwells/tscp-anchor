# TSCP Evidence-to-Authority Kernel — Handoff Document

**Date:** August 19, 2026
**Status:** Correspondence Freeze v0.3 — SEALED
**Freeze commit:** 067fb3e
**Branch:** kernel-archaeology/admissibility-experiment-v0.1
**Repository:** https://github.com/Cartilage-Stairwells/tscp-anchor
**Working tree:** Pristine
**Local ↔ remote:** Synchronized

---

## 1. What Was Done

The TSCP DELTA.zip archive (70 files, ~27MB) was archaeologically investigated by 6 parallel sub-agents to determine whether a real, minimal, technically defensible kernel could be extracted from the accumulated work. The investigation found a **strongly indicated** kernel: an Evidence-to-Authority Decision Engine with four components (canonical serialization, non-self-referential proof structure, decision function, separation invariant).

From that archaeology, a specification was written for the **Admissibility Contract** — the boundary between canonical evidence and the decision function. The specification was then implemented as a minimal Rust experiment and adversarially validated through three rounds of hostile interrogation by Aria (ChatGPT acting as red-team reviewer).

The result is a **frozen, immutable, reproducible evidentiary artifact** establishing a tested Rust/specification equivalence claim within its declared boundary.

---

## 2. Established Claims

| # | Claim | Status |
|:---|:---|:---|
| 1 | **Semantic separation** — AdmittedEvidence is an admissibility fact, not a truth or authority fact. | FROZEN |
| 2 | **Construction enforcement** — Under the frozen safe-Rust model, the intended constructor boundary is mechanically enforced. | FROZEN |
| 3 | **Specification correspondence** — Within the explicitly defined domain D_test ⊆ D_spec, bidirectional interrogation found no divergence between Admission_Rust and Admission_Spec. | PASS |
| 4 | **Threat-model boundedness** — The claim is explicitly not being generalized to excluded mechanisms or universal system security. | FROZEN |

**Terminology:** This is an *experimentally established correspondence claim within the declared domain and threat model*. It is NOT "empirical proof" (proof has a stronger mathematical/formal-verification implication than the experiment provides). It is NOT "the kernel is proven correct."

---

## 3. The Evidence Chain

```
Specification (ADMISSIBILITY_CONTRACT_SPEC.md)
     │
     ▼
Rust implementation (kernel/src/lib.rs)
     │
     ▼
Hostile interrogation (Aria, 3 rounds)
     │
     ├── Round 3: 3 implementation/spec defects found → repaired
     ├── Round 3b: 2 specification ambiguities found → spec amended (v0.3)
     └── Round 3c: 1 test-construction error found → corrected
             │
             ▼
       Correct layer repaired each time
             │
             ▼
       Re-interrogation after each repair
             │
             ▼
       95/95 tests + no divergence
             │
             ▼
      CORRESPONDENCE FREEZE v0.3
             │
             ▼
       Immutable baseline (commit 067fb3e)
```

The sequence of discovered defects → repairs → re-attacks IS the evidence. The history must not be squashed. Future work does not get to quietly rewrite the evidence underneath the claim.

---

## 4. Commit History (Do Not Squash)

```
067fb3e  Correspondence Freeze v0.3 — admissibility kernel sealed
5645c1f  Round 3c: amendment attack — 95/95, CORRESPONDENCE = PASS
39bedde  Spec v0.3: Amendments A & B — close two Round 3b ambiguities
93f49a3  Round 3b: bidirectional correspondence interrogation — 69/69, HOLD
3f7be4c  Add Round 3b package: bidirectional correspondence interrogation
a8f38b1  FIX: Aria Round 3 correspondence defects — 3 fixes, 31/31 tests
10fc8d0  TSCP Kernel Archaeology — Admissibility Experiment v0.1
```

---

## 5. Frozen Threat Model

The claim is bounded by:

- **Safe Rust only** — no unsafe, no FFI, no reflection
- **No serde** — serialization cannot manufacture AdmittedEvidence
- **No persistence** — no state survives between calls
- **No network** — pure function, no side effects
- **rustc** — trusted compiler implementation; safe Rust language semantics are the TCB

Excluded from this freeze:
- Serialization-based construction attacks
- Compiler compromise
- Persistence/state corruption
- FFI bypass
- Custody transitions
- Duplicate admission detection
- Network-level attacks

---

## 6. The Fundamental Firewall

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

This is permanently visible in the specification (§4 semantic non-implications) and in the implementation (type-level enforcement: AdmittedEvidence has no authority fields, cannot be recycled as Evidence, cannot bypass admit()).

---

## 7. Test Inventory

| Suite | Count | File |
|:---|:---|:---|
| Regression (6 property categories) | 31 | kernel/src/tests.rs |
| Round 3b bidirectional | 38 | kernel/src/round3b_tests.rs |
| Round 3c amendment attack | 26 | kernel/src/round3c_tests.rs |
| **Total** | **95** | all pass |

---

## 8. Artifact Inventory

### Specification
- `ADMISSIBILITY_CONTRACT_SPEC.md` — v0.3 (amended with A & B)
- `SPEC_AMENDMENTS_v0.3.md` — Amendment A (AcceptedCanonVersions) & B (Evidence | null)

### Implementation
- `kernel/src/lib.rs` — Rust implementation (admit function + types)
- `kernel/src/tests.rs` — 31 regression tests
- `kernel/src/round3b_tests.rs` — 38 Round 3b tests
- `kernel/src/round3c_tests.rs` — 26 Round 3c tests
- `kernel/Cargo.toml` — package definition

### Review Evidence
- `ARIA_REVIEW_2.md` — Aria's second red-team review (HOLD → experimental implementation justified)
- `ARIA_ROUND3_FINDINGS.md` — Round 3 findings (3 defects, 31/31 after repair)
- `ARIA_ROUND3_PACKAGE.md` — Round 3 correspondence package
- `ARIA_ROUND3B_FINDINGS.md` — Round 3b findings (69/69, HOLD, 2 ambiguities)
- `ARIA_ROUND3B_PACKAGE.md` — Round 3b interrogation framework
- `ARIA_ROUND3C_FINDINGS.md` — Round 3c findings (95/95, PASS)

### Freeze
- `CORRESPONDENCE_FREEZE_v0.3.md` — formal freeze document
- `RUST_THREAT_MODEL.md` — frozen threat model definition
- `VALIDATION_RESULTS.txt` — 95/95 test output

### Archaeology (read-only background)
- `FOUNDING_DOCUMENT.md` — Aria's founding document
- `KERNEL_CHARTER.md` — kernel charter (disposition: STRONGLY INDICATED)
- `KERNEL_INVARIANT_REGISTRY.md` — 12 invariants, 7 enforced
- `KERNEL_EVIDENCE_MATRIX.md` — component evidence assessment
- `IMPLEMENTATION_READINESS.md` — architecture recommendation
- `INDEPENDENT_REVIEW.md` — self-adversarial review
- `ARCHAEOLOGY_REPORT.md` — summary report
- `ARCHAEOLOGY_*.md` — 6 detailed archaeological analyses
- `TSCP-CANON-001.md` — canonical serialization specification
- `RECOVERY_ELIMINATION_REPORT.md` — recovery pattern analysis

---

## 9. Quarantine and Deferred Items

| Item | Status | Description |
|:---|:---|:---|
| Duplicate admission | QUARANTINED | Spec defines error code TSCP-ADMIT-DUPLICATE-ADMISSION but no mechanism. Implementation makes no claim. |
| admitted_at format | DEFERRED | Static string instead of real RFC 3339 clock. Doesn't affect determinism or decision. |
| Universal security | NOT CLAIMED | The correspondence is within the frozen safe-Rust threat model only. |

---

## 10. Next Stage

The frozen commit becomes the root for future work:

```
067fb3e (FREEZE BOUNDARY)
    │
    ├── duplicate-admission (own spec, threat model, gate)
    ├── persistence/state recovery (own spec, threat model, gate)
    ├── custody transitions (own spec, threat model, gate)
    ├── serialization boundary (own spec, threat model, gate)
    └── other separately scoped experiments
```

Each future capability gets:
1. Its own specification surface
2. Its own threat model
3. Its own evidence corpus
4. Its own attack criteria
5. Its own gate

**Do not expand the frozen kernel.** The experiment has done its job. The next change should get its own scope and gate, especially if it introduces state, persistence, serialization, duplicate detection, or custody transitions. Those mechanisms can materially change the threat model.

---

## 11. Key Documents for Resumption

If resuming this work in a new session:

1. **Read first:** `CORRESPONDENCE_FREEZE_v0.3.md` — the freeze boundary
2. **Then:** `ADMISSIBILITY_CONTRACT_SPEC.md` — the specification (v0.3)
3. **Then:** `RUST_THREAT_MODEL.md` — what's in/out of scope
4. **Then:** `ARIA_ROUND3C_FINDINGS.md` — the final correspondence claim
5. **Then:** This handoff document — for context and next steps

The GitHub repository is the durable, externally verifiable record. The sandbox contains working copies. The repository is authoritative.
