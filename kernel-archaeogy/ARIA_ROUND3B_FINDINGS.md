# Aria Round 3b — Bidirectional Correspondence Findings

**Date:** August 18, 2026
**Auditor:** Lyra (Base44 Superagent)
**Subject:** Bidirectional correspondence interrogation of commit a8f38b1
**Prior:** Aria Round 3 (3 defects) → FIXED → 31/31 regression → this audit
**Test count:** 69/69 pass (31 regression + 38 Round 3b)

---

## The Gate

> PASS correspondence only when Round 3b finds no known bidirectional divergence AND no unresolved implementation/specification ambiguity within the explicitly defined domain.

---

## Correspondence Matrix

```
                    SPEC ACCEPT          SPEC REJECT
RUST ACCEPT         ✓ (verified)         ✗ NONE FOUND (false positive)
RUST REJECT         ✗ NONE FOUND         ✓ (verified)
                    (false negative)
```

**No false positive found.** No input was discovered where the Rust implementation admits evidence that the specification requires rejected.

**No false negative found.** No input was discovered where the specification requires admission but Rust rejects.

---

## Vector 1: False Positive (Rust accepts, Spec rejects) — NONE FOUND

| Test | Attack | Result |
|:---|:---|:---|
| V1.1 | media_type=None accepted | PASS — spec says optional |
| V1.2 | uppercase hex digest rejected | PASS — spec says lowercase |
| V1.3 | RejectedEvidence.evidence populated for non-empty | PASS |
| V1.4 | duplicate digest (non-consecutive) caught | PASS — HashSet catches all |
| V1.5 | canon_version acceptance not checked | **DIVERGENCE** (see Gap 1) |
| V1.7 | collects all type rejections (not first-fail) | PASS |

**Finding:** No false positive in the behavioral domain. V1.5 identifies a specification gap (the "accepted version" check), but this is a spec-level ambiguity, not a case where Rust admits what the spec rejects — the spec doesn't define what "accepted" means, so there's no concrete input the spec rejects that Rust accepts.

---

## Vector 2: False Negative (Spec accepts, Rust rejects) — NONE FOUND

| Test | Attack | Result |
|:---|:---|:---|
| V2.1 | empty media_type string accepted | PASS |
| V2.2 | all-zeros digest accepted | PASS — valid 64-char hex |
| V2.3 | min==max contract works | PASS |
| V2.4 | empty required_roles accepted | PASS — spec doesn't require non-empty |
| V2.5 | all same roles (no required) accepted | PASS |
| V2.6 | role not in contract rejected | PASS — correct rejection |

**Finding:** No false negative. Rust does not over-reject any specification-valid input.

---

## Vector 3: Component Predicate Divergence — NONE FOUND

| Test | Attack | Result |
|:---|:---|:---|
| V3.1 | validation collects ALL structural failures | PASS |
| V3.2 | canon-version mismatch fires for ALL mismatched items | PASS |
| V3.3 | same item gets both structural AND canon-version rejection | PASS |
| V3.4 | validation passes → binding fails (correct stage) | PASS |
| V3.5 | validation+binding pass → completeness fails (correct stage) | PASS |
| V3.6 | min count checked before required roles | PASS — matches spec order |

**Finding:** Each subpredicate (Validation, Binding, Completeness, CanonVersion) corresponds to its spec counterpart. Stage ordering matches spec §3.2. Rejection collection within stages matches spec §3.4 ("collected, not first-fail"). No component predicate divergence found.

---

## Vector 4: Boundary/Domain Edge Cases — NONE FOUND

| Test | Attack | Result |
|:---|:---|:---|
| V4.1 | empty evidence slice — no panic | PASS (FIX 1 holds) |
| V4.2 | exactly max_evidence_count | PASS |
| V4.3 | one over max | PASS — ExcessEvidence |
| V4.4 | min=1, max=1 (single item contract) | PASS |
| V4.5 | duplicate digest, different types/roles | PASS — caught |
| V4.6 | same role, different digests | PASS — valid |
| V4.7 | mixed canon versions (some match, some don't) | PASS — only mismatched rejected |

**Finding:** All boundary cases handled correctly. The empty-domain fix holds. Edge cases around min==max, single-item contracts, and duplicate detection with different metadata all behave per spec.

---

## Vector 5: Repaired-Seam Regression — ALL HOLD

| Test | Attack | Result |
|:---|:---|:---|
| V5.1 | empty evidence with 3 different contracts | PASS — no panic |
| V5.2 | partial canon mismatch (1 of 3 wrong) | PASS — only wrong item rejected |
| V5.3 | contract accessor correctness | PASS — all 8 accessors return correct values |
| V5.4 | Contract::new rejects all invalid constructions | PASS — 4 invalid cases rejected |
| V5.5 | determinism stress (3 identical calls) | PASS — same result |
| V5.6 | admission digest changes with different evidence | PASS — different digests |

**Finding:** All three Round 3 repairs hold under stress. The empty-domain fix, canon-version check, and contract immutability all survive adversarial testing beyond their original test cases.

---

## Vector 6: Specification Gaps — 5 IDENTIFIED

### Gap 1: canon_version acceptance check (AMBIGUITY)
**Spec §2.3 condition 7:** "canon_version must match an accepted TSCP-CANON-001 version"
**Rust:** `if c.canon_version.is_empty() { return Some(ContractInvalid); }`
**Divergence:** Rust checks non-empty. Spec requires matching an "accepted" version. The spec doesn't define what "accepted" means or provide a version list.
**Classification:** SPECIFICATION AMBIGUITY — the spec requires a check that isn't fully defined. Rust implements a weaker version because the spec doesn't provide enough information to implement the full check.
**Disposition:** HOLD — resolve in spec (define accepted version list) or explicitly relax spec to "non-empty."

### Gap 2: RejectedEvidence.evidence type (AMBIGUITY)
**Spec §2.5:** `readonly evidence: Evidence;` (non-optional)
**Rust:** `pub evidence: Option<Evidence>;`
**Divergence:** For non-empty evidence, Rust populates `Some(evidence)`. For empty evidence (the fixed panic case), Rust uses `None`. The spec type is non-optional, but the spec doesn't define what RejectedEvidence.evidence should contain when there IS no evidence item.
**Classification:** SPECIFICATION AMBIGUITY — the spec type doesn't account for the empty-domain case. Rust's `Option<Evidence>` is a reasonable adaptation, but it's a type-level divergence.
**Disposition:** HOLD — resolve in spec (make RejectedEvidence.evidence optional, or define a separate EmptyRejection type).

### Gap 3: Evidence canon_version representation (REPRESENTATION DIVERGENCE)
**Spec §2.2:** Evidence has 4 fields (digest, artifact_type, media_type, role). Canon version is "implicit, established upstream."
**Rust:** Evidence has 5 fields (adds `canon_version: String`).
**Divergence:** Spec says canon version is implicit (not a field, established during upstream canonicalization). Rust makes it explicit (a field on Evidence).
**Classification:** REPRESENTATION DIVERGENCE — the Rust Evidence type has a field the spec doesn't define. This is a reasonable implementation choice (how else would the implementation check canon-version correspondence?), but it's a type-level change.
**Disposition:** RECORD — not an ambiguity per se (the behavior matches), but the type differs. Should be documented in spec as a permitted implementation strategy.

### Gap 4: Duplicate admission (DEFERRED — correctly)
**Spec §2.5:** Defines `TSCP-ADMIT-DUPLICATE-ADMISSION` error code.
**Spec §3.2:** Does not define the operational mechanism for "already admitted to this contract."
**Rust:** Does not implement duplicate admission. No error code in enum.
**Classification:** SPECIFICATION GAP — correctly deferred. The spec defines the error code but not the mechanism. Rust correctly makes no claim.
**Disposition:** QUARANTINED — do not implement in Rust until spec defines the mechanism.

### Gap 5: admitted_at timestamp format (MINOR)
**Spec §2.4:** `admitted_at: string; // RFC 3339 UTC timestamp`
**Rust:** `"2026-08-19T06:00:00Z".to_string()` — hardcoded static string.
**Divergence:** Rust uses a fixed timestamp instead of a dynamic RFC 3339 UTC clock value. Spec says this field is "informational only and must not affect the admission decision."
**Classification:** MINOR DIVERGENCE — doesn't affect determinism or correctness. The value is RFC 3339 format compliant. Just not from a real clock.
**Disposition:** RECORD — acceptable for experimental implementation. Production implementation should use a real timestamp.

---

## Final Disposition

```
Bidirectional behavioral divergence:     NONE FOUND
Component predicate divergence:            NONE FOUND
Repaired-seam regression:                  ALL HOLD
Specification ambiguities (unresolved):     2 (Gap 1, Gap 2)
Specification gaps (recorded):             3 (Gap 3, Gap 4, Gap 5)

CORRESPONDENCE = HOLD
```

The gate requires "no known bidirectional divergence AND no unresolved implementation/specification ambiguity." There is no bidirectional divergence — the implementation and specification agree on every tested input across all six attack vectors. But two specification ambiguities remain:

1. **canon_version acceptance check** — spec says "accepted version," Rust checks non-empty. The spec is underspecified.
2. **RejectedEvidence.evidence type** — spec says non-optional, Rust uses Option for empty domain. The spec doesn't account for the empty case.

Both are specification-level issues, not implementation defects. They should be resolved by clarifying the specification, not by modifying the Rust implementation.

---

## Updated Correspondence Matrix

| Axis | Round 3 | Round 3b | Change |
|:---|:---|:---|:---|
| Digest structural validation | PASS | PASS | — |
| artifact_type validation | PASS | PASS | — |
| Role validity | PASS | PASS | — |
| Canon-version correspondence | **FAIL → FIXED** | PASS | Improved |
| Type binding | PASS | PASS | — |
| Role binding | PASS | PASS | — |
| Duplicate digest | PASS | PASS | — |
| Minimum evidence (empty domain) | **FAIL → FIXED** | PASS | Improved |
| Maximum evidence | PASS | PASS | — |
| Required roles | PASS | PASS | — |
| Contract immutability | **FAIL → FIXED** | PASS | Improved |
| AdmittedEvidence construction | PASS | PASS | — |
| Deserialization bypass | PASS | PASS | — |
| Unsafe/transmute bypass | PASS | PASS | — |
| FFI bypass | PASS | PASS | — |
| Clone bypass | PASS | PASS | — |
| Authority laundering | PASS | PASS | — |
| Truth/correctness laundering | PASS | PASS | — |
| Duplicate-admission predicate | DEFERRED | DEFERRED | — |
| Stage ordering | untested | PASS | New |
| Stage collection (not first-fail) | untested | PASS | New |
| Determinism | untested | PASS | New |
| Admission digest evidence-dependence | untested | PASS | New |
| canon_version acceptance check | untested | **AMBIGUITY** | New |
| RejectedEvidence type (empty domain) | untested | **AMBIGUITY** | New |
| Evidence canon_version representation | untested | **RECORD** | New |
| admitted_at format | untested | **MINOR** | New |

---

## Test Summary

- Regression tests: 31/31 pass
- Round 3b bidirectional tests: 38/38 pass
- Total: 69/69 pass
- Correspondence: HOLD (no behavioral divergence, 2 spec ambiguities)

---

## Recommendation

The two ambiguities should be resolved by amending the specification:

1. **Spec §2.3 condition 7:** Either define the accepted TSCP-CANON-001 version list, or relax to "canon_version must be non-empty" (matching the implementation).

2. **Spec §2.5 RejectedEvidence:** Either make `evidence` optional (`evidence: Evidence | null`), or define a separate rejection type for the empty-domain case.

Once these two spec clarifications are made, the gate can be re-evaluated. If no new ambiguities are found, correspondence moves to PASS.

The implementation should NOT be modified to resolve these — that would make the implementation an accidental spec author. The spec must be clarified first.
