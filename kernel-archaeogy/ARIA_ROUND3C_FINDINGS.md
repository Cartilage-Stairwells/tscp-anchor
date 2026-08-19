# Aria Round 3c — Amendment Attack Findings

**Date:** August 19, 2026
**Auditor:** Lyra (Base44 Superagent)
**Subject:** Attack Amendments A & B + their interaction with existing predicates
**Prior:** Round 3b (69/69, HOLD, 2 ambiguities) → Spec v0.3 amendments → this audit
**Test count:** 95/95 pass (31 regression + 38 Round 3b + 26 Round 3c)

---

## The Gate

> PASS correspondence only when Round 3c finds no divergence within the defined test domain, and the amended specification contains no unresolved ambiguity identified by the audit.

NOT: "95/95 passes → PASS"

---

## Attack A: Canon-version converse (Amendment A)

| Test | Input | Expected | Result |
|:---|:---|:---|:---|
| A.1 | canon_version="1.0" | accept | PASS |
| A.2 | Contract construction with "1.0" | Ok | PASS |
| A.3 | canon_version="" | reject | PASS |
| A.4 | canon_version="1.1" | reject | PASS |
| A.5 | canon_version="0.9" | reject | PASS |
| A.6 | canon_version="garbage" | reject | PASS |
| A.7 | canon_version=" 1.0" (whitespace) | reject | PASS |
| A.8 | evidence canon="1.0" matches contract | admit | PASS |
| A.9 | evidence canon="1.1" ≠ contract "1.0" | reject (CanonVersionMismatch) | PASS |
| A.10 | contract "1.1" construction | reject (ContractInvalid) | PASS |

**Bidirectional correspondence verified:**
- Spec accepts canon version "1.0" ↔ Rust accepts "1.0"
- Spec rejects all other versions ↔ Rust rejects all other versions
- The AcceptedCanonVersions predicate is now unambiguous: membership in {"1.0"}
- Evidence canon_version is checked against contract's canon_version (not directly against AcceptedCanonVersions)
- Amendment A eliminated the canon-version semantic ambiguity without introducing a new divergence

**No divergence found.**

---

## Attack B: Empty-evidence correspondence (Amendment B)

| Test | Input | Expected | Result |
|:---|:---|:---|:---|
| B.1 | evidence=[] | InsufficientEvidence, evidence=null | PASS |
| B.2 | non-empty invalid evidence | rejection with Some(evidence) | PASS |
| B.3 | non-empty binding failure | rejection with Some(evidence) | PASS |
| B.4 | evidence=[], min=1 | InsufficientEvidence, evidence=None | PASS |

**Correspondence verified:**
- `|evidence| = 0` → `RejectedEvidence.evidence = null` (None in Rust)
- Non-empty invalid evidence → `RejectedEvidence.evidence = Some(item)` (not null)
- The Amendment B representation (Evidence | null) matches Rust's Option<Evidence>
- The empty domain is explicitly represented, not silently handled

**Converse verified:** non-empty evidence does NOT collapse into the empty-domain representation. The distinction is preserved.

**No divergence found.**

---

## Attack C: Interaction (Amendments × existing predicates)

| Test | Combination | Expected stage | Result |
|:---|:---|:---|:---|
| C.1 | empty evidence + valid contract | Completeness (Insufficient) | PASS |
| C.2 | malformed evidence + canon mismatch | Validation (both collected) | PASS |
| C.3 | valid evidence + accepted canon | Admit | PASS |
| C.4 | valid evidence + evidence canon mismatch | Validation (CanonVersionMismatch) | PASS |
| C.5 | incomplete evidence + accepted canon | Completeness (Insufficient) | PASS |
| C.6 | invalid contract + empty evidence | N/A (construction gate) | PASS |
| C.7 | contract invalid canon → construction | ContractInvalid at construction | PASS |
| C.8 | excess evidence + valid types | Completeness (Excess) | PASS |
| C.9 | validation errors halt before binding | Validation only | PASS |
| C.10 | binding errors halt before completeness | Binding only | PASS |
| C.11 | multiple validation failures collected | All collected (4 rejections) | PASS |
| C.12 | evidence canon independent of AcceptedVersions | Evidence vs contract check | PASS |

**Key interaction findings:**

1. **Stage ownership preserved:** Validation failures halt before binding (C.9). Binding failures halt before completeness (C.10). The amendments did not change stage precedence.

2. **Collection within stages preserved:** Multiple validation failures (structure + canon) are collected, not first-fail (C.2, C.11). The amendments did not change intra-stage collection behavior.

3. **Contract validation is a construction-time gate (C.6, C.7):** Invalid contracts (including non-accepted canon versions) are rejected at `Contract::new()`, not at `admit()` time. This means `admit()` always receives a valid contract. The spec's Stage 1 contract validity check is structurally guaranteed by the type system — an invalid contract cannot exist as a value. This is an implementation strength, not a divergence.

4. **Amendment A scope (C.12):** AcceptedCanonVersions governs contract construction only. Evidence canon_version is checked against the contract's canon_version, not against AcceptedCanonVersions directly. This is correct — the spec says "Each Evidence item's explicit canon_version must match the contract's canon_version."

5. **No accidental precedence change (C.1, C.5):** Empty evidence with a valid contract reaches Stage 3 (Completeness), not Stage 1 (Validation). The empty-domain fix did not alter stage flow. Incomplete evidence with accepted canon also reaches Completeness.

**No divergence found. No precedence or ownership changes introduced by amendments.**

---

## Ordering analysis

Aria asked: "If both validation failures exist, does the spec tell us whether canon mismatch is detected before or after structural validation—or is ordering intentionally unspecified?"

**Finding:** Within Stage 1, the implementation checks: (1) contract validity (first-fail), then (2) per-evidence structure + canon version (collected). The spec §3.2 lists contract validity as step 1 with "if any fails, reject" (first-fail language), then evidence structure as step 2 and canon version as step 3 (both "each" — collect language).

The implementation matches: contract validity is first-fail, evidence-level checks are collected. The spec's ordering is: contract → evidence structure → canon version, but within the evidence-level checks, all are collected (not first-fail per §3.4).

**The error ordering IS part of the correspondence claim** — the spec specifies it via the numbered steps in §3.2 and the "collect, not first-fail" rule in §3.4. The implementation matches this ordering. This is not an accidental claim; it's a spec-defined property.

---

## Specification gaps still outstanding (unchanged from Round 3b)

| Gap | Status | Disposition |
|:---|:---|:---|
| Duplicate admission | QUARANTINED | Spec defines error code, no mechanism. Implementation makes no claim. |
| admitted_at format | MINOR | Static string vs RFC 3339 clock. Doesn't affect decision or determinism. |

Neither gap is an ambiguity introduced by the amendments. Both were identified and recorded in Round 3b. Neither blocks correspondence.

---

## Final Disposition

```
Round 3c attack vectors:
  A. Canon-version converse:          NO DIVERGENCE
  B. Empty-evidence correspondence:    NO DIVERGENCE
  C. Interaction (amendments × existing): NO DIVERGENCE

Amendment A (AcceptedCanonVersions):
  - Eliminated canon-version ambiguity
  - No new divergence introduced
  - Bidirectional correspondence verified

Amendment B (Evidence | null):
  - Eliminated empty-domain type ambiguity
  - No new divergence introduced
  - Converse verified (non-empty doesn't collapse to null)

Stage ownership and precedence:
  - Unchanged by amendments
  - Validation → Binding → Completeness ordering preserved
  - Collection within stages preserved

Outstanding specification gaps:
  - Duplicate admission (quarantined — no mechanism)
  - admitted_at format (minor — doesn't affect decision)

CORRESPONDENCE = PASS
within the frozen Rust threat model and explicitly defined admissibility domain
```

This is NOT a universal security guarantee. It is a correspondence claim:

> The Rust implementation faithfully instantiates the specified admissibility predicate over the explicitly defined domain, under the frozen safe-Rust threat model.

The frozen threat model is: safe Rust, no serde, no FFI, no unsafe, no reflection, no persistence. The explicitly defined domain is: contracts with canon_version ∈ {"1.0"}, evidence with 64-char lowercase hex digests, four-role enum, three-stage admission (validation → binding → completeness).

The qualification "within the frozen Rust threat model and explicitly defined domain" is permanently attached to this claim.

---

## Test summary

| Suite | Count | Status |
|:---|:---|:---|
| Regression (tests.rs) | 31/31 | PASS |
| Round 3b bidirectional (round3b_tests.rs) | 38/38 | PASS |
| Round 3c amendment (round3c_tests.rs) | 26/26 | PASS |
| **Total** | **95/95** | **PASS** |

The 95/95 result is regression evidence. The correspondence claim rests on the absence of divergence across three rounds of bidirectional interrogation, not on the test count.

---

## Chain of custody

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
[this commit]  Round 3c: 95/95, PASS
```

The process worked as designed: Round 3b found ambiguities → spec was amended → Round 3c attacked the amendments → no new divergence → PASS.
