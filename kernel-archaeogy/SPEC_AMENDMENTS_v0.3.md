# Specification Amendments A & B — v0.3

**Date:** August 18, 2026
**Spec:** ADMISSIBILITY_CONTRACT_SPEC.md v0.2 → v0.3
**Per:** Aria's Round 3b disposition — two unresolved ambiguities requiring spec amendment

---

## Amendment A — Canon version semantics

**Problem (Round 3b Gap 1):** Spec §2.3 condition 7 said "canon_version must match an accepted TSCP-CANON-001 version" without defining "accepted." Rust checked `is_empty()` only. This was a specification ambiguity — the spec required a check it didn't define.

**Resolution:** Added §2.3.1 AcceptedCanonVersions — a fixed enumerated set:

```typescript
type AcceptedCanonVersions = "1.0";  // pinned enumerated set
```

The acceptance predicate is now concrete: `canon_version ∈ AcceptedCanonVersions`.

Future versions are added by specification amendment only, not by implementation decision.

**Implementation change:** Rust `validate_contract()` now checks membership in `ACCEPTED_CANON_VERSIONS: &["1.0"]` instead of `is_empty()`. The implementation inherits the accepted set from the spec.

**Tests updated:**
- V1.5: Now asserts non-accepted versions are rejected (was: asserted accepted)
- V5.1: Fixed test contract to use "1.0" (was: "2.0", now rejected)
- V6.5: Now asserts non-accepted versions are rejected + accepted version "1.0" is valid

**Evidence field:** Evidence now has an explicit `canon_version: string` field (was: "implicit, established upstream"). Stage 1.3 updated to reference the explicit field. This formalizes what the implementation already does.

---

## Amendment B — Empty evidence domain

**Problem (Round 3b Gap 2):** Spec §2.5 defined `RejectedEvidence.evidence: Evidence` (non-optional). When `evidence = []`, there is no Evidence object to place in this field. Rust used `Option<Evidence>`. This was a specification ambiguity — the spec type didn't account for the empty domain.

**Resolution:** `RejectedEvidence.evidence` is now `Evidence | null`. Null is the defined representation when `|evidence| = 0`. The rejection model explicitly includes the empty-domain case:

- `error_code`: `TSCP-ADMIT-INSUFFICIENT-EVIDENCE`
- `error_stage`: `COMPLETENESS`
- `evidence`: `null`

The specification does not require a separate rejection type for empty evidence — `null` in the `evidence` field is the defined representation.

**Implementation change:** None — Rust already uses `Option<Evidence>`, which corresponds to `Evidence | null`. The spec now formalizes what the implementation already does.

**Tests:** No changes needed — existing tests already cover this case.

---

## What these amendments resolve

| Gap | Round 3b classification | Amendment | Resolution |
|:---|:---|:---|:---|
| Canon version acceptance | SPECIFICATION AMBIGUITY | A | AcceptedCanonVersions = {"1.0"} — concrete predicate |
| RejectedEvidence empty domain | SPECIFICATION AMBIGUITY | B | evidence: Evidence \| null — nullable for empty domain |
| Evidence canon_version representation | REPRESENTATION DIVERGENCE | A (consequence) | Made explicit in §2.2 — now matches implementation |
| Duplicate admission | SPECIFICATION GAP | — | Correctly deferred (no mechanism in spec §3) |
| admitted_at format | MINOR DIVERGENCE | — | Recorded (static string, doesn't affect decision) |

---

## What remains after these amendments

- **Duplicate admission:** Still quarantined. Spec defines error code but no mechanism. Implementation makes no claim.
- **admitted_at format:** Minor. Static string instead of real RFC 3339 clock. Doesn't affect determinism or correctness.
- **Round 3c:** Must attack the amendments themselves, not merely rerun the 69 tests. Question: "Did the amendments actually eliminate the semantic ambiguity without introducing a new divergence?"

---

## Test results after amendments

- Regression tests: 31/31 pass
- Round 3b tests: 38/38 pass (3 tests updated for Amendment A behavior)
- Total: 69/69 pass
