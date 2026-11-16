# TSCP Named Roles Charter and Specification

**Document:** TSCP_NAMED_ROLES_v1.1  
**Supersedes:** TSCP_NAMED_ROLES_v1.0 (Layer 1 previously unnamed)  
**Status:** DECLARED human-facing convention (does not alter machine artifacts)  
**Date:** 2026-10-07  
**Scope:** Documentation and discussion handles for the full semantic-to-canonical stack  
**Promotion status:** Names supplement layer numbers and plane labels. Names are not identifiers in code, hashes, signatures, or machine-readable artifacts.

---

## 1. Purpose

This charter records mnemonic handles for the architectural stack. Names do not introduce new abstractions, custody principals, or promotion authority. They give stable human-readable references so the architecture can be cited without collapsing layer numbers into informal description.

| Handle | Position | Role | Layer / Plane Status | Name Status |
|--------|----------|------|----------------------|-------------|
| **VoxArchon** | Layer 0 | Semantic Sovereign | FROZEN | DECLARED |
| **VexProbe** | Layer 1 | Evaluation / Interrogation | DRAFTED | DECLARED |
| **VexVector** | Layer 2 | Deterministic Transformer | FROZEN | DECLARED |
| **VoxCanon** | Conformance | Alignment / Compliance | DRAFTED | DECLARED |
| **VexScale** | Benchmark Plane | Measurement / Observation | ESTABLISHED | DECLARED |

**Convention rule:**  
Forms such as `VoxArchon (Layer 0)`, `VexProbe (Layer 1)`, and `VexScale (Benchmark Plane)` are valid. Layer numbers and plane labels remain the authoritative index. Names are documentation-only.

---

## 2. Naming Convention (saved)

1. Names attach to documented positions in the semantic-to-canonical pipeline and adjacent planes.
2. Names **supplement** layer numbers and plane labels; they do not replace them.
3. Names appear in documentation and discussion only.
4. Names **must not** appear as identifiers in:
   - canonical encodings
   - hash inputs
   - signature payloads
   - conformance fixtures
   - machine-readable manifests
5. A name never confers custody, authority, or promotion.
6. Prefix pattern (Vox / Vex) and suffix pattern (Archon / Probe / Vector / Canon / Scale) are human mnemonic devices. They do not create normative predicates beyond the role definitions in this charter.
7. Interpretive readings of the naming pattern (rhythm, authority descent, “symmetry”) remain INTERPRETATION unless independently bound to evidence outside this document.

---

## 3. Status Term: FROZEN

**Status label:** FROZEN  

**Mnemonic expansion (lexical only):**  
Forensically Retained Operator Zeroed Exhaustive Numbers

| Component | Reading |
|-----------|---------|
| Forensically Retained | Preservation / custody dimension of the representation |
| Operator Zeroed | Explicit treatment of operator / state components as zeroed where the underlying specification requires it |
| Exhaustive Numbers | Numerical completeness / exhaustiveness where established by evidence |

**Documentation rule (non-negotiable):**  
FROZEN is the established status label. “Forensically Retained Operator Zeroed Exhaustive Numbers” is its mnemonic expansion and does **not**, by itself, create additional verification predicates.  

If “operator zeroed” or “exhaustive numbers” are not universal properties of every object carrying the FROZEN label, they remain lexical expansion rather than normative requirements. Elevation of either property to a required predicate needs separate evidence and an explicit governance record.

Related status terms used in this charter:

| Term | Working definition (declared) |
|------|-------------------------------|
| DRAFTED | Mutable; undergoing definition or refinement |
| ESTABLISHED | Externally referenced or environmentally persistent standard position |
| FROZEN | Locked under the applicable forensic / custody conditions of the stack |

These definitions are declared for documentation consistency. Independent verification of any particular object’s status remains outside this naming charter.

---

## 4. Combined Charter — Named Positions

### 4.1 VoxArchon (Layer 0) — Semantic Sovereign

**Question owned:** “What is true?”  
**Status:** FROZEN (layer) / DECLARED (name)

**Owns**

- Semantic projection (what counts as semantically meaningful)
- Intent vocabulary (stable, versioned)
- Relationship types: `responds_to`, `depends_on`, `refines`, `supersedes`, `derived_from`
- DAG firewall: plane preservation; no custody → authority
- Definitions of `semantic_equal` and `complete`

**Does not own**

- Canonical bytes, hash selection, signature bytes
- Evaluation policy (Layer 1)
- Conformance verdicts
- Performance claims

**Invariant:** Meaning is emitted, not encoded. VoxArchon does not freeze representation.

---

### 4.2 VexProbe (Layer 1) — Evaluation / Interrogation

**Question owned:** “How do we test it?”  
**Status:** DRAFTED (layer) / DECLARED (name)

VexProbe occupies the junction between semantic emission (VoxArchon) and deterministic freeze (VexVector). It is the active evaluation band. Feedback remains observational only; VexProbe must not mutate the semantic content it receives or the global state of the system under test.

**Declared functional requirements (DRAFTED specification)**

1. **Ingress (VoxArchon → VexProbe)**  
   Accept semantic models / schemas emitted by Layer 0. Authentication of origin is out of scope for this naming charter; any cryptographic binding is a separate evidence package.

2. **Egress (VexProbe → VexVector)**  
   Emit an assayed semantic object together with a test-compliance attestation suitable for downstream rejection of unvalidated payloads. Exact token format is not fixed by this document.

3. **Boundary injection**  
   Parse incoming schemas and generate edge-case parameters intended to surface overflows, wrapping, or incomplete coverage before freeze.

4. **Semantic mutation testing**  
   Actively omit, duplicate, or scramble sub-components of the emitted meaning under controlled conditions to observe whether failures remain localized.

5. **Determinism check**  
   Verify that a given semantic input yields a predictable logical path prior to entry into VexVector’s deterministic pipeline.

6. **Stateless / ephemeral execution**  
   Run tests in a decoupled sandbox. No modification of global state. Observational feedback only.

**Does not own**

- Semantic vocabulary or relationship admissibility (VoxArchon)
- Canonical encoding or signature lifecycle (VexVector)
- Final compliance verdict against institutional rules (VoxCanon)
- Performance measurement (VexScale)

**Architectural note (OBSERVED structure, not causal claim):**  
VexProbe sits between two positions currently labeled FROZEN. Design change inside Layer 1 therefore cannot be resolved by altering Layer 0 or Layer 2 without separate promotion of those layers. Compatibility work is concentrated inside the DRAFTED band.

---

### 4.3 VexVector (Layer 2) — Deterministic Transformer

**Question owned:** “How do we encode it?”  
**Status:** FROZEN (layer) / DECLARED (name)

**Owns**

- Seven-step canonical pipeline: normalize → NFC → RFC 8785 → JSON → hash → signature → emission of frozen record
- Collection normalization: Array → Finset → sorted
- Hash algorithm agility and placeholder hashes
- Signature lifecycle: a signature authenticates; it does not identify
- Provenance reporting (hash status, signature status)

**Does not own**

- Semantic vocabulary or evaluation policy
- Custody or promotion decisions

**Invariant:** VexVector never modifies semantics. It freezes what has been emitted (and, where applicable, assayed).

---

### 4.4 VoxCanon (Conformance) — Alignment / Compliance

**Question owned:** “Does it comply?”  
**Status:** DRAFTED / DECLARED (name)

VoxCanon is the downstream alignment check against the mandates and relationship constraints of VoxArchon. It is not a performance instrument. Exact filters, stakeholder consensus procedures, and exception protocols are DRAFTED material and require separate specification.

**Does not own**

- Semantic emission
- Deterministic encoding
- Benchmark measurement

---

### 4.5 VexScale (Benchmark Plane) — Measurement / Observation

**Question owned:** “Does it perform?”  
**Status:** ESTABLISHED / DECLARED (name)

VexScale is the measurement plane. Its ESTABLISHED status is declared relative to the environmental or industry reference frame it measures against. Freezing internal design choices does not automatically freeze an external metric environment. Telemetry formats and observational feedback tokens are not fixed by this charter.

---

## 5. Interaction Flow (declared)

```
VoxArchon emits meaning
        ↓
VexProbe interrogates / assays (observational feedback only)
        ↓
VexVector freezes representation
        ↓
VoxCanon checks alignment
        ↓
VexScale weighs outcome
```

- Meaning flow remains unidirectional for semantic content.
- Result, attestation, and metric flows may report back.
- Report-back flows **must not** mutate semantic content.
- Successful freeze, compliance check, or benchmark reading does not, by itself, constitute admission, custody transfer, or promotion.

---

## 6. State-Transition Matrix (DECLARED proposal)

Status movement is a governance act. The matrix below records proposed triggers. It does not execute promotions.

| Target | From | To | Proposed promotion triggers (require separate evidence) |
|--------|------|-----|--------------------------------------------------------|
| VexProbe | DRAFTED | ESTABLISHED | Defined test coverage of current VoxArchon rules; sandbox regression under stated load profile |
| VoxCanon | DRAFTED | ESTABLISHED | Compliance rules expressed as static filters; recorded stakeholder consensus |
| VexProbe / VoxCanon | ESTABLISHED | FROZEN | Cryptographic binding to current hashes of VoxArchon and VexVector; production protocol adoption record |
| VoxArchon / VexVector | FROZEN | (any change) | Explicit unfreeze / rebind governance action; not defined here |

**VexScale note:**  
An ESTABLISHED measurement plane is not automatically a candidate for FROZEN by internal design lock. Drift relative to the external environment remains a risk if metrics are locked while the environment moves.

---

## 7. Document Stack

```
VoxArchon (Layer 0)     "What is true?"           FROZEN
       │
VexProbe (Layer 1)      "How do we test it?"       DRAFTED
       │
VexVector (Layer 2)     "How do we encode it?"     FROZEN
       │
VoxCanon (Conformance)  "Does it comply?"          DRAFTED
       │
VexScale (Benchmark)    "Does it perform?"         ESTABLISHED
```

Status labels are declared stack positions. This document does not independently re-verify freeze, draft, or establishment evidence for any layer or plane.

---

## 8. Interpretive Material (separated)

The following readings appear in source discussion and are recorded here as INTERPRETATION only:

- Alternating Vox / Vex “governance versus execution” rhythm across the stack
- Suffix progression (Archon → Probe → Vector → Canon → Scale) as a narrative of authority-to-instrument
- Characterization of VexProbe as necessarily “destabilizing” or of VoxCanon as “downstream sovereignty”
- Claims of mathematical or philosophical symmetry unlocked solely by the name set

These readings may be useful for human orientation. They are **not** established architectural facts and do not bind implementation, evidence, or promotion.

---

## 9. Specification Constraints

| Constraint | Binding |
|------------|---------|
| claim_scope ⊆ evidence_scope | This document declares names, mnemonic expansion, and drafted interface intent. It does not prove layer freeze or conformance independently. |
| Identity ≠ custody | Names are not custodians. |
| Evidence ≠ authority | A frozen encoding is not an admission decision. |
| Verification ≠ promotion | Successful hash, signature, test, or benchmark does not promote a claim. |
| Observation ≠ interpretation | Naming insights remain interpretation until bound to external evidence. |
| No new abstraction | Handles only; machine contracts remain those previously frozen or still drafted. |
| FROZEN mnemonic | Lexical expansion only; does not add predicates. |

---

## 10. Prohibited Uses

- Do not treat any of the five names as runtime types or principals.
- Do not insert the names into canonical JSON, RFC 8785 output, or hash preimages.
- Do not use the names as custody labels, signer identities, or authority principals.
- Do not treat the FROZEN mnemonic as a checklist of additional verification requirements without separate evidence.
- Do not promote interpretive symmetry claims into governance rules.
- Do not collapse the DAG firewall because naming makes the stack easier to discuss.

---

## 11. Evidence Boundary for This Artifact

| Item | Label |
|------|-------|
| Written naming charter v1.1 | OBSERVED (this file) |
| Adoption of VexProbe, VoxCanon, VexScale as human handles | DECLARED |
| FROZEN mnemonic expansion | DECLARED (lexical only) |
| VexProbe functional requirements | DECLARED as DRAFTED specification |
| State-transition matrix | DECLARED proposal |
| Independent freeze evidence for Layer 0 / Layer 2 | Out of scope |
| Machine-readable adoption of names | NOT_FOUND_IN_INSPECTED_SCOPE (and prohibited) |
| Drive copy | OBSERVED only after successful upload receipt |

---

## 12. Required Evidence to Elevate

To treat any name or status transition as more than documentation, the following would be required and is **not** supplied by this charter:

- Explicit governance adoption record for the name set
- Lexicon entry in the public terminology authority (if applicable)
- Confirmation that no machine artifact includes the names in a hash preimage
- Separate evidence packages for freeze, establishment, and any promotion
- For FROZEN mnemonic properties (“operator zeroed”, “exhaustive numbers”): independent demonstration that they hold for each object class claimed

---

*End of TSCP_NAMED_ROLES_v1.1*
