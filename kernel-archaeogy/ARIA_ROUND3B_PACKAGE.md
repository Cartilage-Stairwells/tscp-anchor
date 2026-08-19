# TSCP Admissibility Kernel — Round 3b Package for Aria

**Date:** August 18, 2026
**Subject:** Bidirectional correspondence interrogation of commit a8f38b1
**Prior:** Aria Round 3 findings (3 defects) → FIXED in a8f38b1 → 31/31 regression pass
**Status:** READY FOR ARIA — correspondence HOLD pending this audit

---

## The Gate

> PASS correspondence ONLY when Round 3b finds no known bidirectional divergence and no unresolved implementation/specification ambiguity within the explicitly defined domain.

NOT: "31/31 tests passed → PASS"

That would be the epistemic mistake the custody model warns against. Regression success ≠ correspondence proof. The custody model requires formal contract preservation, not cross-implementation agreement on sampled inputs.

---

## Round 3 Results (Completed)

| Finding | Disposition |
|:---|:---|
| Empty evidence panic | FIXED |
| Dead CanonVersionMismatch | FIXED |
| Mutable contract parameters | FIXED |
| Construction boundary | PASS |
| Semantic firewall | PASS |
| Regression corpus | 31/31 PASS |
| Universal Rust ↔ Spec correspondence | HOLD |
| Duplicate-admission semantics | HOLD / specification gap |

---

## Duplicate Admission — Quarantined

```
DUPLICATE_ADMISSION
    specification: UNDER-SPECIFIED
    implementation: NO CLAIM MADE
    status: HOLD
```

Do NOT implement duplicate-admission in Rust. The implementation must not become an accidental author of specification semantics. This is the exact custody failure the framework prevents.

---

## Round 3b Attack Framework

### The Correspondence Matrix

```
                    SPEC ACCEPT          SPEC REJECT
RUST ACCEPT          ???                 ← false positive (too permissive)
RUST REJECT          ← false negative    ???
                     (too restrictive)
```

Both directions must be attacked, not merely searching for another Rust rejection that ought to succeed.

### Predicate Decomposition

```
Admission
├── Validation
│   └── ValidationRust ↔ ValidationSpec
├── Binding
│   └── BindingRust ↔ BindingSpec
├── Completeness
│   └── CompletenessRust ↔ CompletenessSpec
├── Canon version
│   └── CanonVersionRust ↔ CanonVersionSpec
└── Evidence/domain handling
    └── DomainRust ↔ DomainSpec
```

Each subpredicate can agree independently on the regression suite while their composition differs. The attack must decompose to the component level.

### Six Attack Vectors

#### 1. Rust accepts / Spec rejects (false positive — too permissive)

Construct inputs where the Rust implementation admits evidence that the specification requires rejected.

Check:
- Does the Rust validation stage accept everything the spec requires rejected?
- Are there structural edge cases where Rust's digest check is weaker than the spec's?
- Does the canon-version check cover all specified cases or only the tested case?
- Are there artifact_type values that should be rejected but aren't?

#### 2. Spec accepts / Rust rejects (false negative — too restrictive)

Construct inputs where the specification requires admission but Rust rejects.

Check:
- Are there valid evidence configurations that Rust over-rejects?
- Does the contract validation in Contract::new() reject any valid contract?
- Are there evidence items with valid structure that the Rust digest check incorrectly rejects? (e.g., uppercase hex, leading zeros)
- Does the role enum cover all spec-defined roles?

#### 3. Component predicate divergence

Each subpredicate can agree independently on tested cases while their composition differs.

Check:
- Validation predicate in isolation: does it match the spec's Stage 1 exactly?
- Binding predicate in isolation: does it match the spec's Stage 2 exactly?
- Completeness predicate in isolation: does it match the spec's Stage 3 exactly?
- Composition: V ∧ B ∧ C in Rust — does the ordering matter? The spec says rejection reasons are collected, not first-fail. Does Rust collect or first-fail at each stage?

#### 4. Boundary/domain divergence

Test structurally unusual but specification-valid inputs.

Check:
- Empty evidence slice (fixed — verify it returns RejectedEvidence, not panic)
- Exactly min_evidence_count items
- Exactly max_evidence_count items
- Evidence with media_type = None vs Some
- Evidence with identical digests but different roles (duplicate detection is by digest only)
- Evidence with identical roles but different digests (should be valid if roles aren't required to be unique)
- Contract with required_roles = [] (should this be valid?)
- Contract with min == max
- Canon version edge cases: empty string, numeric, semantic versioning strings

#### 5. Repaired-seam regression

Verify the three fixes actually work under stress, not just on the tested cases.

Check:
- Empty evidence: does it return RejectedEvidence with evidence: None? Does it work with multiple contracts?
- Canon-version mismatch: does it fire for ALL evidence items with wrong canon_version, or just the first? Does it fire when only SOME evidence has wrong canon_version?
- Contract immutability: can you construct a valid Contract and then observe that no mutation path exists? Can you construct a Contract with edge-case parameters that passes validation but shouldn't?

#### 6. Specification gaps

Record gaps without silently resolving them in Rust.

Known gaps:
- Duplicate-admission: spec defines error code but §3 doesn't define the mechanism → HOLD
- Canon version on Evidence: spec says "implicit canon version (established upstream during canonicalization)" — the Rust implementation makes it explicit. Is this a divergence or a legitimate representation choice?
- RejectedEvidence.evidence is now Option<Evidence>: spec defines it as containing the evidence item. For the empty-domain case, there IS no evidence item. Is this a spec gap or an implementation divergence?

---

## Commit Under Audit

```
Commit: a8f38b1
Branch: kernel-archaeology/admissibility-experiment-v0.1
Files:
  kernel-archaeogy/kernel/src/lib.rs   (v0.2 — with 3 fixes)
  kernel-archaeogy/kernel/src/tests.rs (31 tests)
  kernel-archaeogy/ADMISSIBILITY_CONTRACT_SPEC.md (v0.2)
  kernel-archaeogy/ARIA_ROUND3_FINDINGS.md (Round 3 results)
```

Clone:
```
git clone -b kernel-archaeology/admissibility-experiment-v0.1 https://github.com/Cartilage-Stairwells/tscp-anchor.git
```

---

## Questions for Aria Round 3b

### Q1 (False Positive)
Does there exist an input x such that Admission_Rust(x) = ACCEPT but Admission_Spec(x) = REJECT?

### Q2 (False Negative)
Does there exist an input x such that Admission_Spec(x) = ACCEPT but Admission_Rust(x) = REJECT?

### Q3 (Component Divergence)
Do the individual stage predicates (Validation, Binding, Completeness, CanonVersion) each correspond to their spec counterparts, or do any diverge when tested in isolation?

### Q4 (Repaired-Seam Regression)
Do the three fixes hold under stress beyond their tested cases?

### Q5 (Specification Gaps)
What specification ambiguities does the implementation expose that should be resolved in the spec rather than silently resolved in Rust?

---

## Final Disposition (After Round 3b)

```
IF no false positive AND no false negative AND no component divergence:
    correspondence = PASS (for declared domain)

IF any divergence found:
    correspondence = FAIL
    identify defect
    fix
    re-audit

IF specification gaps found:
    record separately
    DO NOT resolve in Rust
    HOLD until spec is clarified
```

The gate is NOT "all tests pass." The gate is "no known bidirectional divergence and no unresolved implementation/specification ambiguity within the explicitly defined domain."
