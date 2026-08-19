# Kernel Invariant Registry v0.1

**Status:** PROPOSED — extracted from archaeological evidence, not yet formally enforced
**Date:** August 18, 2026
**Authority:** TSCP Kernel Charter v0.1

---

## Registry Format

Each invariant is recorded with:
- **ID**: Stable identifier
- **Statement**: The invariant in one sentence
- **Archaeological Evidence**: Where this invariant appears in the archive
- **Enforcement Status**: How (or whether) it is currently enforced
- **Attack Surface**: What happens if violated

---

## I-01: Evidence ≠ Authority (CENTRAL INVARIANT)

**Statement:** Evidence NEVER creates authority by itself. Evidence is necessary input; authority requires deterministic state evaluation against active contracts.

**Evidence:**
- FOUNDING_DOCUMENT.md (Aug 2026): "Evidence NEVER creates authority by itself. Everything else derives from that."
- LEGIO Activation Workflow (Jan 2026): 3/5 threshold — evidence (signatures) must be evaluated against contract (threshold) to produce authority
- ARCHAEOLOGY_REPORT.md (Jan 2026): "authority is distributed and decoupled... authority requires re-engaging the LEGIO protocol"
- TrifoldWallet (early 2024): Oracle quorum — evidence feeds into authority decisions but does not equal authority

**Enforcement:** DECLARED, NOT ENFORCED. No code currently checks this invariant.

**Attack Surface:** If violated, any agent that discovers evidence (files, keys, logs) could manufacture authority, collapsing the entire custody model.

---

## I-02: Canonical Determinism

**Statement:** Identical protocol objects MUST produce identical canonical bytes and SHA-256 digests across all runtimes, languages, and implementations.

**Evidence:**
- TSCP-CANON-001.md v1.0: Formal specification of canonical JSON serialization
- canonical.ts: TypeScript implementation
- acceptance-receipt.json: Cross-runtime verification (Python, Rust, TypeScript all pass 15/15)
- canon_conformance.test.ts: 17 test cases with exact digest matching

**Enforcement:** ENFORCED via conformance test suite and acceptance-receipt.json. Three independent implementations agree.

**Attack Surface:** If violated, two semantically identical objects could hash to different digests (or two different objects to the same digest), breaking evidence comparability and receipt verification.

---

## I-03: Non-Self-Referential Proof

**Statement:** A proof payload MUST NOT contain its own calculated digest. Digests are emitted in separate external receipt objects.

**Evidence:**
- Proof Envelope v2 (TSCP-PROOF-002): Explicit non-self-referential design
- acceptance-receipt.json: Receipt object separate from the artifacts it verifies
- SKILL.md: Receipt generation protocol requires external binding to git commit

**Enforcement:** SPECIFIED, PARTIALLY ENFORCED. The acceptance-receipt.json follows this pattern. The Proof Envelope v2 specification enforces it but is on HOLD.

**Attack Surface:** If violated, circular hashing could create self-validating evidence that cannot be independently verified.

---

## I-04: Float Prohibition

**Statement:** Floating-point values are PROHIBITED in protocol objects. Fractional values MUST be represented as scaled integers with explicit scale factors.

**Evidence:**
- TSCP-CANON-001.md §3.1.3: "Floating-point values are strictly prohibited"
- canonical.ts: Rejects inputs containing `.`, `e`, or `E` in numeric literals
- canon_conformance.test.ts: Tests `TSCP-CANON-FLOAT-PROHIBITED` error code

**Enforcement:** ENFORCED in canonical.ts and conformance test suite.

**Attack Surface:** If violated, IEEE-754 precision drift could cause identical logical values to produce different canonical bytes across runtimes.

---

## I-05: NFC Normalization

**Statement:** All strings — both keys and values — MUST be normalized to Unicode Normalization Form C (NFC) before key sorting and serialization.

**Evidence:**
- TSCP-CANON-001.md §3.1.2: Formal NFC normalization requirement
- canonical.ts: Implementation enforces NFC normalization
- canon_conformance.test.ts: Tests `TSCP-CANON-KEY-COLLISION` error code

**Enforcement:** ENFORCED in canonical.ts and conformance test suite.

**Attack Surface:** If violated, semantically identical strings (precomposed vs. decomposed) would serialize to different bytes, breaking cross-runtime determinism.

---

## I-06: Top-Level Map Constraint

**Statement:** The top-level input to canonicalization MUST be a map (JSON object). Bare primitives, arrays, or null at the root MUST be rejected.

**Evidence:**
- TSCP-CANON-001.md §2.1: "Protocol Objects are always maps"
- canonical.ts: Enforces `TSCP-CANON-TOPLEVEL-NONMAP` error
- canon_conformance.test.ts: Tests top-level non-map rejection

**Enforcement:** ENFORCED in canonical.ts and conformance test suite.

**Attack Surface:** If violated, non-map roots could bypass key-sorting and normalization, producing ambiguous canonical representations.

---

## I-07: Explicit Null Distinction

**Statement:** A field explicitly set to `null` is DISTINCT from a field that is absent. Implementations MUST NOT conflate "absent" with "null."

**Evidence:**
- TSCP-CANON-001.md §3.1.5: "A field that is entirely absent from a Protocol Object is distinct from a field explicitly set to null"
- canonical.ts: Preserves explicit null in output

**Enforcement:** ENFORCED in canonical.ts.

**Attack Surface:** If violated, the absence of evidence could be treated as explicit null evidence, or vice versa, creating semantic ambiguity in decision evaluation.

---

## I-08: Read-Only by Default

**Statement:** System states and repositories are read-only by default. Transitioning to implementation-authorized state requires two independent explicit signals: (1) implementation authority and (2) integration-base selection (commit SHA).

**Evidence:**
- SKILL.md: "Two-Key Authorization" — `HOLD` to `IMPLEMENTATION AUTHORIZED` requires both `implementation_authority` and `integration_base`
- Proof Envelope v2 readiness review: Placed on HOLD (not auto-authorized)
- governed-protocol-readiness-review.skill: Read-only governance gate enforcement

**Enforcement:** SPECIFIED in SKILL.md. NOT enforced in any implementation.

**Attack Surface:** If violated, unauthorized transitions could be committed without governance review, bypassing the separation invariant.

---

## I-09: Deterministic Error Taxonomy

**Statement:** Rejection is part of the protocol contract. Every rejection MUST be classified by stage (`VALIDATION`, `NORMALIZATION`, `NUMERIC`) and error code. Independent implementations MUST reject for the same reason.

**Evidence:**
- TSCP-CANON-001.md §8: Formal error taxonomy with stages and codes
- canonical.ts: Returns typed `CanonicalizationError` with `error_code` and `error_stage`
- canon_conformance.test.ts: Asserts exact error code and stage on REJECT cases
- acceptance-receipt.json: Records error classifications per fixture

**Enforcement:** ENFORCED in canonical.ts and conformance test suite.

**Attack Surface:** If violated, two implementations could both "reject" the same input for different reasons, masking incompatibilities.

---

## I-10: Cross-Runtime Conformance Inheritance

**Statement:** Implementations inherit meaning from the specification. They do not compete with each other. Conformance is verified by executing a canonicalizer against fixture inputs and comparing digests — never by hand-authored assertions.

**Evidence:**
- TSCP-CANON-001.md §5: "A manifest containing fabricated, hand-written, or otherwise unverified digest values does not constitute a valid conformance suite"
- acceptance-receipt.json: Three runtimes (Python, Rust, TypeScript) all pass 15/15 with matching digests
- run_conformance.ts: CLI runner executes manifest fixtures

**Enforcement:** ENFORCED via acceptance-receipt.json and run_conformance.ts.

**Attack Surface:** If violated, implementations could diverge silently while claiming conformance, creating incompatible "valid" systems.

---

## I-11: Array Order Significance

**Statement:** In proof envelopes, evidence array ordering is commitment-significant. Reordering evidence changes the canonical digest.

**Evidence:**
- Proof Envelope v2: `004_evidence_array_order_significant`
- TSCP-CANON-001.md §3.1: Array order is preserved (not sorted)

**Enforcement:** SPECIFIED in Proof Envelope v2. ENFORCED in canonical.ts (arrays preserve order).

**Attack Surface:** If violated, an attacker could reorder evidence to produce a different digest while claiming the same evidence set.

---

## I-12: Git Commit Binding

**Statement:** Receipts and conformance results are bound to specific git commit hashes. This binding is immutable — a receipt proves conformance at a specific code state, not in general.

**Evidence:**
- acceptance-receipt.json: `git_commit: "500b699f59cdca2bb976f91340e2cdc0eefa304d"`
- TSCP-CANON-001-PIN.md: Dependency lock binding to `tscp-canon-001-accepted`

**Enforcement:** ENFORCED in acceptance-receipt.json (commit hash recorded).

**Attack Surface:** If violated, a receipt from one code state could be presented as evidence for a different code state, creating a version attack.

---

## Summary

| ID | Invariant | Enforcement |
|:---|:---|:---|
| I-01 | Evidence ≠ Authority | DECLARED, NOT ENFORCED |
| I-02 | Canonical Determinism | ✅ ENFORCED (3 runtimes verified) |
| I-03 | Non-Self-Referential Proof | PARTIALLY ENFORCED |
| I-04 | Float Prohibition | ✅ ENFORCED |
| I-05 | NFC Normalization | ✅ ENFORCED |
| I-06 | Top-Level Map Constraint | ✅ ENFORCED |
| I-07 | Explicit Null Distinction | ✅ ENFORCED |
| I-08 | Read-Only by Default | SPECIFIED, NOT ENFORCED |
| I-09 | Deterministic Error Taxonomy | ✅ ENFORCED |
| I-10 | Cross-Runtime Conformance Inheritance | ✅ ENFORCED |
| I-11 | Array Order Significance | SPECIFIED, PARTIALLY ENFORCED |
| I-12 | Git Commit Binding | ✅ ENFORCED |

**Enforced: 7/12** | **Partially enforced: 2/12** | **Specified only: 3/12**

The kernel's canonicalization layer (I-02 through I-07, I-09, I-10, I-12) is well-specified and verified. The authority/decision layer (I-01, I-08, I-11) is specified but not yet enforced in code.
