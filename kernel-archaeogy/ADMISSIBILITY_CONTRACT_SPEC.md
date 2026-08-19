# TSCP Admissibility Contract Specification v0.2

**Status:** PROPOSED — specification artifact, not implementation
**Date:** August 18, 2026
**Authority:** Independent Review of TSCP Kernel Archaeology + Aria Red-Team Review (ARIA_REVIEW_2.md)
**Dependencies:** TSCP-CANON-001 v1.0 (canonical serialization)
**Changes from v0.1:** Added semantic non-implications (§4), sharpened stage semantics with explicit non-establishment claims, added implementation-level boundary concerns (§8), added experimental validation criteria (§9), added forbidden arrows (§4.3)

---

## 0. Purpose

This specification defines the contract between canonical evidence and the decision function. It exists to answer one question:

**What makes canonical evidence admissible to a specific contract for a specific decision?**

Without this contract, the proposed `evaluate()` function cannot be implemented without either:
- Accepting all canonical evidence (smuggling authority into evidence — violating the separation invariant), or
- Embedding admissibility logic inside evaluate() (conflating the contract layer with the decision layer), or
- Leaving admissibility undefined (undocumented behavior)

This specification resolves that by defining admissibility as a **separate, deterministic, type-enforced step** between canonical evidence and the decision function.

---

## 1. Architectural Position

```
                    TSCP-CANON-001
                         │
                         ▼
              canonical evidence (bytes + digest)
                         │
                         ▼
              ╔══════════════════════╗
              ║  ADMISSIBILITY GATE  ◄── this specification
              ╚══════════════════════╝
                         │
                    ┌────┴────┐
                    ▼         ▼
              AdmittedEvd  RejectedEvidence
                    │         (reason)
                    ▼
              ╔══════════════════╗
              ║   evaluate(...)  ◄── downstream (not this spec)
              ╚══════════════════╝
                         │
                         ▼
                    Decision
                         │
                         ▼
              ╔══════════════════╗
              ║  custody apply   ◄── downstream (not this spec)
              ╚══════════════════╝
                         │
                         ▼
                   CustodyState
                         │
                         ▼
                    Authority
```

**Forbidden arrows (per Aria review §24, §25):**
- `AdmittedEvidence → Authority` — admission is NOT an authority-producing operation
- `AdmittedEvidence → Truth` — admission does NOT establish truth
- `AdmittedEvidence → Correctness` — admission does NOT establish correctness
- `AdmittedEvidence → Authenticity` — admission does NOT establish authenticity

Authority arises only through the downstream transition/custody machinery. This negative space is part of the design, not an omission.

---

## 2. Core Types

### 2.1 CanonicalDigest

```typescript
type CanonicalDigest = string;  // 64-char lowercase hex SHA-256
```

A SHA-256 digest of canonical bytes produced by TSCP-CANON-001 canonicalization. This is the only form in which evidence enters the admissibility gate. Raw objects, untyped bytes, or non-canonical representations are NOT evidence.

**Invariant:** A CanonicalDigest MUST be the output of `SHA-256(canonicalize(obj))` where `canonicalize` conforms to TSCP-CANON-001 v1.0. The admissibility gate does not re-canonicalize; it trusts the digest binding established upstream.

### 2.2 Evidence

```typescript
interface Evidence {
  readonly digest: CanonicalDigest;
  readonly artifact_type: string;       // what kind of artifact produced this evidence
  readonly media_type: string | null;    // MIME type of the source artifact, if applicable
  readonly role: EvidenceRole;
}

type EvidenceRole = "input" | "output" | "attestation" | "witness";
```

Evidence is a **claim** that a canonical artifact exists and has a specific digest. Evidence is NOT the artifact itself — it is a reference to an artifact, verified through its canonical digest.

**Critical constraint — Evidence contains NO authority fields:**
- Evidence has no `signature` field
- Evidence has no `threshold` field
- Evidence has no `authorization` field
- Evidence has no `weight` or `priority` field
- Evidence has no `decision` field

Evidence is purely descriptive: it says WHAT something is (artifact type, digest, role), not WHETHER it authorizes anything. The absence of authority fields is the first mechanical enforcement of the separation invariant.

### 2.3 Contract

```typescript
interface Contract {
  readonly id: string;                  // stable contract identifier
  readonly version: string;             // contract version (semver)
  readonly evidence_types: readonly string[];  // which artifact_types are admissible
  readonly evidence_roles: readonly EvidenceRole[];  // which roles are admissible
  readonly min_evidence_count: number;   // minimum number of evidence items required
  readonly max_evidence_count: number;   // maximum number of evidence items allowed
  readonly required_roles: readonly EvidenceRole[];  // roles that MUST be present
  readonly canon_version: string;       // TSCP-CANON-001 version this contract requires
}
```

A Contract is a **specification of admissibility rules**. It declares what evidence it will accept, in what roles, and in what quantities. A Contract is NOT evidence — a Contract is a rule set.

**Contract validity conditions:**
1. `id` and `version` must be non-empty strings
2. `evidence_types` must be non-empty
3. `evidence_roles` must be non-empty
4. `min_evidence_count` must be ≥ 1
5. `max_evidence_count` must be ≥ `min_evidence_count`
6. `required_roles` must be a subset of `evidence_roles`
7. `canon_version` must match an accepted TSCP-CANON-001 version

**Contract immutability:** Contracts are read-only after creation. A Contract cannot be modified by evidence, by decisions, or by custody state changes. This prevents evidence from rewriting the rules under which it is evaluated.

### 2.4 AdmittedEvidence

```typescript
interface AdmittedEvidence {
  readonly contract_id: string;          // binding to the contract that admitted this evidence
  readonly contract_version: string;
  readonly evidence: readonly Evidence[];
  readonly admitted_at: string;         // RFC 3339 UTC timestamp (informational only)
  readonly admission_digest: CanonicalDigest;  // digest of the admission record
}
```

AdmittedEvidence is a **custody classification**, not a quality upgrade. It records that evidence crossed a specified boundary. It is not "better evidence" — it is evidence that has been classified as admissible relative to a specific contract.

**Critical constraint — AdmittedEvidence is a one-way type:**
- AdmittedEvidence cannot be converted back to Evidence
- AdmittedEvidence cannot be constructed without passing through the admissibility gate
- AdmittedEvidence cannot be embedded in new Evidence (no circular admission)
- AdmittedEvidence is the ONLY input type that evaluate() accepts for evidence

This is the second mechanical enforcement: there is no code path from Evidence to evaluate() that bypasses the admissibility gate.

### 2.5 RejectedEvidence

```typescript
interface RejectedEvidence {
  readonly evidence: Evidence;
  readonly contract_id: string;
  readonly reason: RejectionReason;
  readonly error_code: AdmissibilityErrorCode;
  readonly error_stage: AdmissibilityStage;
}

type RejectionReason = string;

type AdmissibilityStage = "VALIDATION" | "BINDING" | "COMPLETENESS";

type AdmissibilityErrorCode =
  | "TSCP-ADMIT-TYPE-NOT-ADMISSIBLE"      // artifact_type not in contract.evidence_types
  | "TSCP-ADMIT-ROLE-NOT-ADMISSIBLE"      // role not in contract.evidence_roles
  | "TSCP-ADMIT-CANON-VERSION-MISMATCH"  // evidence canon_version ≠ contract canon_version
  | "TSCP-ADMIT-INSUFFICIENT-EVIDENCE"    // fewer evidence items than min_evidence_count
  | "TSCP-ADMIT-EXCESS-EVIDENCE"         // more evidence items than max_evidence_count
  | "TSCP-ADMIT-MISSING-REQUIRED-ROLE"    // a required role is not present in the evidence set
  | "TSCP-ADMIT-DUPLICATE-DIGEST"        // same digest appears more than once in evidence set
  | "TSCP-ADMIT-CONTRACT-INVALID"        // contract fails validity conditions
  | "TSCP-ADMIT-DUPLICATE-ADMISSION"     // this evidence has already been admitted to this contract
  ;
```

Rejection follows the TSCP-CANON-001 pattern: every rejection is classified by stage and error code. Independent implementations must reject for the same reason.

---

## 3. Admissibility Function

### 3.1 Signature

```typescript
function admit(
  contract: Contract,
  evidence: Evidence[]
): AdmittedEvidence | RejectedEvidence[];
```

### 3.2 Execution Stages

The admissibility function executes in three stages. Each stage can reject evidence. Rejection at any stage halts processing — later stages do not execute.

#### Stage 1: VALIDATION

**Establishes:** Evidence satisfies required structural/contractual validity conditions.
**Does NOT establish:** Evidence is true, correct, or trustworthy.

1. **Contract validity:** Check all Contract validity conditions (§2.3). If any fails, reject with `TSCP-ADMIT-CONTRACT-INVALID`.
2. **Evidence structure:** Each Evidence item must have a valid `digest` (64-char lowercase hex), a non-empty `artifact_type`, and a valid `role`. Structural failures at this stage are rejected with the appropriate error code.
3. **Canon version:** Each Evidence item's implicit canon version (established upstream during canonicalization) must match the contract's `canon_version`. Mismatches are rejected with `TSCP-ADMIT-CANON-VERSION-MISMATCH`.

**Semantic boundary:** Validation checks structural properties only. If a predicate supplied by an external source returns true during validation, that establishes only "the supplied predicate returned true" — not "the proposition represented by the evidence is true." Validation must not become a hidden truth oracle.

#### Stage 2: BINDING

**Establishes:** Evidence is associated with the correct canonical specification/context under the defined binding relation.
**Does NOT establish:** The specification endorses the claim contained in the evidence.

1. **Type admissibility:** Each evidence item's `artifact_type` must appear in `contract.evidence_types`. Rejected with `TSCP-ADMIT-TYPE-NOT-ADMISSIBLE`.
2. **Role admissibility:** Each evidence item's `role` must appear in `contract.evidence_roles`. Rejected with `TSCP-ADMIT-ROLE-NOT-ADMISSIBLE`.
3. **Duplicate detection:** No two evidence items in the same submission may have the same `digest`. Rejected with `TSCP-ADMIT-DUPLICATE-DIGEST`.

**Semantic boundary:** Binding establishes association, not endorsement. "E is bound to S" means "E is associated with S under the specified binding relation" — not "S sanctions E" or "S vouches for E." The specification must explicitly state what relation is established and what relations are NOT established.

#### Stage 3: COMPLETENESS

**Establishes:** Evidence contains everything required by the admission contract relative to the specified completeness criterion (`complete_relative_to_schema`).
**Does NOT establish:** Nothing relevant is missing from reality (`epistemic completeness`).

1. **Minimum count:** `evidence.length >= contract.min_evidence_count`. Rejected with `TSCP-ADMIT-INSUFFICIENT-EVIDENCE`.
2. **Maximum count:** `evidence.length <= contract.max_evidence_count`. Rejected with `TSCP-ADMIT-EXCESS-EVIDENCE`.
3. **Required roles:** Every role in `contract.required_roles` must be present at least once in the evidence set. Rejected with `TSCP-ADMIT-MISSING-REQUIRED-ROLE`.

**Semantic boundary:** Completeness is parameterized by its reference frame: `complete_relative_to_schema`, not a generic notion of "complete." A complete evidence package means "all fields/records/components required by the admission contract are present" — not "we have all the evidence necessary to establish the proposition."

### 3.3 Composition Non-Implication

The three stages compose as: `VALIDATION ∧ BINDING ∧ COMPLETENESS → ADMISSIBLE`

This produces **AdmittedEvidence**, which means: "The admission contract was satisfied."

**Explicit non-implications (the semantic firewall):**

```
AdmittedEvidence
    ⇏ True
    ⇏ Correct
    ⇏ Authentic
    ⇏ Authoritative
    ⇏ Trusted
    ⇏ Endorsed
    ⇏ Sanctioned
    ⇏ Verified (in the epistemic sense)
```

This negative semantic contract is not cosmetic. It is part of the boundary. Without it, composition can produce a stronger semantic object than any individual stage claims — the "emergent semantic strengthening" problem where `V∧B∧C` is read as "trustworthy evidence" even though no individual predicate established trustworthiness.

The semantic laundering chain that this firewall breaks:

```
structurally admissible          ← VALIDATION establishes this
    ↓
bound to specification          ← BINDING establishes this
    ↓
complete relative to schema    ← COMPLETENESS establishes this
    ↓
AdmittedEvidence                ← the fact: admission contract was satisfied
    ↓
"trusted"                      ← FORBIDDEN — not established by any stage
    ↓
"correct"                       ← FORBIDDEN — not established by any stage
    ↓
"authorized"                   ← FORBIDDEN — not established by any stage
```

The kernel succeeds if this chain is mechanically broken. This specification breaks it conceptually. The experimental implementation must determine whether a real language/runtime preserves that break.

### 3.4 Determinism

The admissibility function MUST be a pure function:
- Same contract + same evidence set → same result (AdmittedEvidence or same RejectedEvidence[])
- No side effects
- No external state reads
- No time-dependent behavior (the `admitted_at` field is informational only and must not affect the admission decision)
- No ordering dependence within a stage (all items in a stage are checked; rejection reasons are collected, not first-fail)

### 3.5 Admission Digest

The `admission_digest` in AdmittedEvidence is computed as:

```
admission_digest = SHA-256(canonicalize(admission_record))
```

where `admission_record` is:

```json
{
  "contract_id": "<contract.id>",
  "contract_version": "<contract.version>",
  "evidence_digests": ["<digest1>", "<digest2>", ...],
  "canon_version": "<contract.canon_version>"
}
```

This follows the TSCP-CANON-001 canonicalization rules. The admission digest is NOT self-referential — it is computed from the admission record, which does not include the admission digest itself.

---

## 4. Semantic Non-Implications (NEW — per Aria review)

### 4.1 What AdmittedEvidence Means

The existence of an AdmittedEvidence value means exactly:

> **The admission contract was satisfied.**

That is, the evidence satisfied the structural, binding, and completeness requirements defined by the contract.

### 4.2 What AdmittedEvidence Does NOT Mean

| Interpretation | Status | Reason |
|:---|:---|:---|
| "Evidence satisfied structural requirements to enter evaluation domain" | ✅ CORRECT | This is what admission establishes |
| "Evidence is valid" | ⚠️ DANGEROUS | "Valid" is epistemically overloaded — use "admissible" instead |
| "Evidence is true/correct" | ❌ NOT ESTABLISHED | Admission does not establish truth |
| "Evidence is authorized to influence the system" | ❌ CATASTROPHIC | Collapses admissibility into authority |

### 4.3 Forbidden Arrows

```
AdmittedEvidence → Authority    FORBIDDEN — admission is not authority-producing
AdmittedEvidence → Truth        FORBIDDEN — admission does not establish truth
AdmittedEvidence → Correctness  FORBIDDEN — admission does not establish correctness
AdmittedEvidence → Authenticity FORBIDDEN — admission does not establish authenticity
```

The correct flow is:

```
Evidence
   ↓ admission contract (V∧B∧C)
AdmittedEvidence (admissibility fact)
   ↓ evaluation
Decision
   ↓ transition contract
CustodyState
   ↓ authority binding
Authority
```

Authority appears only through the downstream custody transition. Admission is one step. It is not the whole chain.

### 4.4 AdmittedEvidence as Custody Classification

AdmittedEvidence should be understood as a **custody classification**, not a quality transformation:

```
Evidence
   |
   | admission contract
   v
Evidence ∈ AdmissibleDomain
```

The type is a computational encoding of that classification. AdmittedEvidence is not "better evidence." It is evidence that has crossed a specified boundary.

---

## 5. Separation Enforcement

### 5.1 Type-Level Enforcement

The separation invariant ("Evidence NEVER creates authority by itself") is enforced through three type-level constraints:

**Constraint 1: Evidence has no authority fields.**
Evidence (§2.2) contains only descriptive fields. There is no field that can express authority, authorization, threshold, weight, or decision.

**Constraint 2: AdmittedEvidence is a one-way wrapper.**
AdmittedEvidence (§2.4) can only be constructed by the `admit()` function. Evidence enters evaluate() only as AdmittedEvidence, and AdmittedEvidence can only come from `admit()`.

**Constraint 3: Decisions are not Evidence.**
The Decision type (downstream) must not be convertible to Evidence. A Decision cannot be wrapped in a digest and re-submitted as evidence.

### 5.2 What This Prevents

| Attack | How it's prevented |
|:---|:---|
| Evidence claims authority directly | Evidence type has no authority fields (Constraint 1) |
| Evidence bypasses the contract | AdmittedEvidence is one-way; evaluate() only accepts AdmittedEvidence (Constraint 2) |
| Decision is recycled as evidence | Decision is not convertible to Evidence (Constraint 3) |
| Contract is modified by evidence | Contracts are immutable (§2.3) |
| Evidence re-admitted to the same contract | Duplicate admission detection (§3.2, Stage 2) |
| Non-canonical evidence accepted | Only CanonicalDigest enters the gate (§2.1) |
| Semantic laundering (V∧B∧C → "trusted") | Explicit non-implications firewall (§3.3, §4) |

### 5.3 What This Does NOT Prevent (Acknowledged Limits)

This specification does NOT prevent:
- **Contract fraud:** A contract could be authored with overly permissive admissibility rules. Governance problem, not type-system problem.
- **Evidence fabrication:** Evidence contains a digest but not the artifact itself. Artifact-binding problem, not admissibility problem.
- **Predicate manipulation:** What evaluate() does with AdmittedEvidence depends on the predicate, which is not specified here.
- **Custody state corruption:** If the custody state machine allows invalid transitions, separation can be violated downstream.

---

## 6. Relationship to TSCP-CANON-001

### What this spec inherits:

1. **Canonical serialization:** The `canonicalize()` function used to compute `admission_digest` is the TSCP-CANON-001 canonicalizer.
2. **Error taxonomy pattern:** Stage/error-code classification follows the TSCP-CANON-001 pattern.
3. **Determinism principle:** Pure function, same determinism requirement as canonicalizer.
4. **Float prohibition:** Admission records contain no floating-point values.

### What this spec does NOT inherit:

1. **Conformance corpus:** Not yet built.
2. **Cross-runtime verification:** Verified in zero runtimes.
3. **Receipt binding:** Does not define a receipt structure.

---

## 7. Relationship to the Formal-Custody Model

| Stage | Status | Evidence |
|:---|:---|:---|
| Specification | ✅ THIS DOCUMENT | Types, function signature, execution stages, error codes, non-implications |
| Identity Binding | ❌ NOT YET | Contract ID/version binding to git commit |
| Integrity | ❌ NOT YET | SHA-256 digests of admission records |
| Conformance | ❌ NOT YET | Conformance corpus with fixtures |
| External Reproduction | ❌ NOT YET | Independent implementation in a second language |

---

## 8. Implementation-Level Boundary Concerns (NEW — per Aria review)

Aria's review identified critical implementation-level concerns that cannot be resolved at the specification level alone. These are HOLD items that the experimental implementation must address.

### 8.1 Serialization/Deserialization

**Risk:** If AdmittedEvidence can be serialized to bytes and deserialized directly back to AdmittedEvidence, a second admission path exists that bypasses `admit()`.

**Required resolution:** A serialized representation of AdmittedEvidence must NOT automatically be treated as proof that the current runtime has performed admission. Options:
- **Revalidation:** `serialized_AdmittedEvidence → deserialize → revalidate → new AdmittedEvidence`
- **Separate type:** `PersistedAdmissionRecord` which is evidence about a prior admission, not the admitted object itself

**Experimental test:** Attempt to construct AdmittedEvidence via deserialization. If successful, the boundary is broken.

### 8.2 Construction Path Enumeration

**Risk:** If any construction mechanism other than `admit()` can produce AdmittedEvidence, the sole-constructor boundary is violated.

**Required enumeration:** The implementation must verify that NO alternate construction path exists:
- No `AdmittedEvidence::new()`, `::from()`, `::parse()`, `::deserialize()`, `::unchecked()`, `::from_bytes()`
- No macros or generated code that emit `AdmittedEvidence { ... }`
- No reflection or dynamic construction
- No unsafe/transmute/reinterpret operations
- No FFI boundary that accepts foreign bytes as AdmittedEvidence

**Experimental test:** Attempt every known construction mechanism. Document which are blocked and which succeed.

### 8.3 Language Dependency

**Risk:** Type-level enforcement depends on the implementation language's visibility model, safety model, and compilation model.

**Required statement:** The specification's type-level constraints are security properties only in a language whose safety model enforces them. In languages with unsafe escape hatches, FFI, reflection, or dynamic construction, the constraints are conventions, not guarantees.

**Experimental test:** Choose an implementation language, document its safety model, and explicitly classify which constraints are enforced vs. conventional.

### 8.4 Persistence Boundaries

**Risk:** If AdmittedEvidence is persisted and reconstructed on application restart, the reconstruction may not preserve the admission boundary.

**Required distinction:** The specification must distinguish:
- **Admitted object:** the typed value in the current runtime
- **Record asserting prior admission:** evidence that admission previously occurred (a different, weaker type)

**Experimental test:** Persist AdmittedEvidence, restart, attempt to reconstruct. Verify whether reconstruction requires re-admission or silently manufactures the type.

### 8.5 Unsafe / FFI / Reflection

**Risk:** These mechanisms can bypass type-level constraints in most languages.

**Required classification:** If unsafe code, FFI, or reflection are inside the trusted computing base (TCB), they must be explicitly included in the TCB definition. The correct statement becomes: "The type invariant holds for safe code under the language's defined safety model."

**Experimental test:** Attempt to construct AdmittedEvidence via unsafe code, FFI, and reflection. Document results.

---

## 9. Experimental Validation Criteria (NEW — per Aria review)

The experimental implementation is authorized as a **falsification-oriented experiment**: an attempt to break the proposed boundary in a concrete implementation environment.

The experiment is considered validated only if the implementation demonstrates ALL of the following:

1. Evidence cannot directly enter evaluate()
2. AdmittedEvidence cannot be constructed externally
3. Every construction path is enumerated
4. Serialization cannot silently manufacture admission
5. Persistence cannot silently manufacture admission
6. FFI cannot silently manufacture admission
7. Unsafe facilities are explicitly inside/outside the TCB
8. Generated code cannot introduce an unauthorized constructor path
9. Type erasure cannot bypass re-admission
10. Validation establishes no stronger proposition than "structural/contractual validity"
11. Binding establishes no stronger proposition than "association with specification"
12. Completeness establishes no stronger proposition than "complete relative to schema"
13. AdmittedEvidence contains no authority semantics
14. Authority appears only through the downstream custody transition
15. Specification meaning is preserved rather than inferred from implementation behavior

If any of these fail, the architecture requires revision. If all survive, the kernel has crossed from "strongly indicated architecture" to "experimentally validated boundary."

---

## 10. Conformance Requirements

An implementation of the admissibility contract is conformant if and only if:

1. It implements all types defined in §2 with the exact field names and constraints specified
2. It implements the `admit()` function with the exact signature in §3.1
3. It executes the three stages in order (VALIDATION → BINDING → COMPLETENESS)
4. It rejects with the exact error codes defined in §2.5
5. It is a pure function (same inputs → same outputs, no side effects)
6. It does not provide any public constructor for AdmittedEvidence other than `admit()`
7. The Decision type (downstream) is not convertible to Evidence
8. Evidence contains no authority fields
9. It includes the explicit non-implications from §4 as documentation and, where possible, as type-level constraints
10. It passes all 15 experimental validation criteria from §9

A conformance corpus must include at minimum:
- A valid admission case
- A type rejection case
- A role rejection case
- An insufficient evidence case
- A missing required role case
- A duplicate digest case
- An invalid contract case
- A canon version mismatch case

---

## 11. Non-Goals

- This spec does NOT define the evaluate() function (downstream)
- This spec does NOT define the custody state machine (downstream)
- This spec does NOT define the predicate language (downstream)
- This spec does NOT define contract authorship or governance (upstream)
- This spec does NOT define artifact binding or verification (upstream)
- This spec does NOT produce authority — it produces AdmittedEvidence, a custody classification
- This spec does NOT define network protocols, storage, or deployment
- This spec does NOT assume any specific programming language, runtime, or platform
- This spec does NOT claim that type-level enforcement is a security guarantee in all languages
- This spec does NOT claim that AdmittedEvidence establishes truth, correctness, authenticity, or authority

---

## 12. Open Questions

1. **Should AdmittedEvidence be replay-protected?** Within a single `admit()` call: yes (duplicate detection). Cross-call: custody state concern, not admissibility.

2. **Should contracts support evidence ordering requirements?** Evidence array order is preserved but not validated by the admissibility gate. Ordering is a predicate concern.

3. **Should the admission digest be bound to a git commit?** Following the acceptance-receipt.json pattern, yes — but this is a receipt-specification concern, not admissibility.

4. **What happens when TSCP-CANON-001 versions diverge?** Evidence with wrong canon_version is rejected. Version compatibility is a TSCP-CANON-001 governance concern.

5. **What is the trusted computing base?** The implementation language must define which mechanisms (unsafe, FFI, reflection, macros) are inside the TCB. This is an implementation decision, not a specification decision.

6. **Which serialization model?** Revalidation vs. PersistedAdmissionRecord. This is an implementation decision that the experimental implementation must resolve.

---

## 13. Next Steps

1. **✅ DONE** — Admissibility Contract Specification v0.1
2. **✅ DONE** — Independent review (self-review)
3. **✅ DONE** — Aria red-team review
4. **✅ DONE** — Specification updated to v0.2 incorporating review findings
5. **NEXT** — Choose implementation language and document its safety model
6. **THEN** — Implement `admit()` as pure protocol logic (no infrastructure)
7. **THEN** — Run the 15-point experimental validation
8. **THEN** — Specify evaluate() on AdmittedEvidence inputs (if validation passes)
9. **THEN** — Specify custody state machine
10. **THEN** — Adversarial testing of the full chain

---

## Appendix A: Minimal TypeScript Type Sketch

```typescript
// Types only — no implementation

type CanonicalDigest = string;

type EvidenceRole = "input" | "output" | "attestation" | "witness";

interface Evidence {
  readonly digest: CanonicalDigest;
  readonly artifact_type: string;
  readonly media_type: string | null;
  readonly role: EvidenceRole;
}

interface Contract {
  readonly id: string;
  readonly version: string;
  readonly evidence_types: readonly string[];
  readonly evidence_roles: readonly EvidenceRole[];
  readonly min_evidence_count: number;
  readonly max_evidence_count: number;
  readonly required_roles: readonly EvidenceRole[];
  readonly canon_version: string;
}

interface AdmittedEvidence {
  readonly contract_id: string;
  readonly contract_version: string;
  readonly evidence: readonly Evidence[];
  readonly admitted_at: string;
  readonly admission_digest: CanonicalDigest;
}

// Semantic non-implications (enforced via type system where possible,
// enforced via documentation and testing where type system is insufficient):
//   AdmittedEvidence ⇏ Truth
//   AdmittedEvidence ⇏ Correctness
//   AdmittedEvidence ⇏ Authenticity
//   AdmittedEvidence ⇏ Authority
// AdmittedEvidence means: "the admission contract was satisfied." Nothing more.

type AdmissibilityStage = "VALIDATION" | "BINDING" | "COMPLETENESS";

type AdmissibilityErrorCode =
  | "TSCP-ADMIT-TYPE-NOT-ADMISSIBLE"
  | "TSCP-ADMIT-ROLE-NOT-ADMISSIBLE"
  | "TSCP-ADMIT-CANON-VERSION-MISMATCH"
  | "TSCP-ADMIT-INSUFFICIENT-EVIDENCE"
  | "TSCP-ADMIT-EXCESS-EVIDENCE"
  | "TSCP-ADMIT-MISSING-REQUIRED-ROLE"
  | "TSCP-ADMIT-DUPLICATE-DIGEST"
  | "TSCP-ADMIT-CONTRACT-INVALID"
  | "TSCP-ADMIT-DUPLICATE-ADMISSION"
  ;

interface RejectedEvidence {
  readonly evidence: Evidence;
  readonly contract_id: string;
  readonly reason: string;
  readonly error_code: AdmissibilityErrorCode;
  readonly error_stage: AdmissibilityStage;
}

// The ONLY way to produce AdmittedEvidence
declare function admit(
  contract: Contract,
  evidence: Evidence[]
): AdmittedEvidence | RejectedEvidence[];

// evaluate() accepts AdmittedEvidence, NOT raw Evidence
// declare function evaluate(
//   contract: Contract,
//   admitted: AdmittedEvidence,
//   predicate: Predicate,
//   current_state: CustodyState,
//   proposed_transition: Transition
// ): Decision;
```

## Appendix B: Glossary

| Term | Definition |
|:---|:---|
| Evidence | A claim that a canonical artifact exists, with a specific digest and role. Descriptive only — contains no authority. |
| Contract | A specification of admissibility rules. Immutable. |
| AdmittedEvidence | A custody classification: evidence that has crossed a specified boundary. NOT "better evidence." NOT "true evidence." NOT "authorized evidence." |
| Admissibility | The deterministic determination of whether evidence satisfies a contract's requirements. Pure function. |
| Separation Invariant | Evidence cannot create authority by itself. Enforced through type constraints and semantic non-implications. |
| Semantic Laundering | The risk that V∧B∧C → AdmittedEvidence → "trusted" → "authorized." Prevented by explicit non-implications firewall. |
| Forbidden Arrow | A semantic implication that the specification explicitly prohibits. E.g., `AdmittedEvidence → Authority`. |
| Canonical Digest | SHA-256 digest of TSCP-CANON-001 canonical bytes. The only form in which evidence enters the gate. |
| Trusted Computing Base (TCB) | The set of mechanisms (unsafe code, FFI, reflection) that can bypass type-level constraints. Must be explicitly defined per implementation. |
