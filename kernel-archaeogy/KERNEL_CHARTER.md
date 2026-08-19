# TSCP Evidence-to-Authority Kernel — Charter v0.1

**Status:** PROPOSED (archaeological extraction, read-only)
**Date:** August 18, 2026
**Method:** 6-agent parallel archaeological investigation of TSCP DELTA.zip (70 top-level files, 7 nested zips, 7,849 total files, ~96.5 MB uncompressed)
**Governing baseline:** FOUNDING_DOCUMENT.md (Aria, Aug 18 2026)

---

## 1. Problem Statement

The archive contains years of work across multiple domains — Web3 wallets, AI agent orchestration, cryptographic custody, canonical serialization, and formal verification. These appear as distinct projects but share recurring structural patterns. The problem: **determine whether there is a real, minimal, technically defensible kernel that explains why all these artifact families exist, and that provides a principled path from the historical work to the current TSCP/formal-custody architecture.**

---

## 2. What the Kernel Is

### Candidate: The Evidence-to-Authority Decision Engine

The kernel is **the minimal machinery that separates evidence from authority and produces deterministic decisions about state transitions.**

It is NOT:
- An AI agent (the Triumvirate is an application of the kernel)
- A blockchain/wallet system (TrifoldWallet, Vaultfire are applications)
- A governance product (LEGIO is an application of the kernel)
- A particular repository (the kernel is protocol-independent)
- The TSCP name itself (TSCP is the specification framework around the kernel)

The kernel IS four components, each with archaeological evidence:

### Component A: Canonical Serialization (TSCP-CANON-001)
Ensures that evidence is deterministic — identical objects produce identical bytes across all runtimes. This is the most formally specified and implemented component.

### Component B: Non-Self-Referential Proof/Receipt Structure
Ensures that evidence can be verified without circular hashing. Proof payloads never contain their own calculated digest; digests are emitted in separate external receipt objects.

### Component C: The Decision Function
```
evaluate(
    contract,
    evidence,
    predicate,
    current_state,
    proposed_transition
) → Decision

Decision = Allow(next_state) | Reject(reason) | Hold(reason) | Defer(reason)
```
Maps canonical evidence + active contracts + predicates against custody state to produce a deterministic authorization decision.

### Component D: The Separation Invariant
**Evidence NEVER creates authority by itself.** Evidence is necessary input; authority requires deterministic state evaluation against active contracts. This is the invariant that makes the other three components coherent.

---

## 3. Archaeological Evidence Summary

### 3.1 Provenance Map

| Era | Dates | Key Artifacts | Significance |
|:---|:---|:---|:---|
| Phase 0: Antecedents | Early 2024 – Dec 2025 | TrifoldWallet, Vaultfire, SDV PDFs | Application-layer projects with implicit kernel patterns |
| Phase 1: Archaeology | Jan 2026 | ARCHAEOLOGY_REPORT.md, RECOVERY_ELIMINATION_REPORT.md, LEGIO Activation | First explicit identification of authority/evidence/custody patterns |
| Phase 2: Deployment | Feb – Mar 2026 | TSCP Δ10.3 through Δ10.5+ packages | Operational systems implementing the patterns |
| Phase 3: Specification | Jun – Aug 2026 | TSCP-CANON-001, Proof Envelope v2, acceptance-receipt.json | Formal specification of kernel primitives |
| Phase 4: Kernel Thesis | Aug 18 2026 | FOUNDING_DOCUMENT.md | Explicit kernel identification and charter |

### 3.2 Recurring Primitives Matrix

| Primitive | Code | Markdown | JSON | PDF | TXT | Classification |
|:---|:---:|:---:|:---:|:---:|:---:|:---|
| Authority | ❌ | ✅ | ✅ | ✅ | ✅ | FOUNDATIONAL |
| Evidence | ✅ | ✅ | ✅ | ✅ | ❌ | FOUNDATIONAL |
| Custody | ❌ | ✅ | ❌ | ✅ | ❌ | BRIDGE (application→foundation) |
| Contract | ❌ | ✅ | ❌ | ✅ | ✅ | FOUNDATIONAL |
| Predicate | ✅ | ✅ | ✅ | ❌ | ❌ | FOUNDATIONAL |
| Transition | ❌ | ✅ | ✅ | ❌ | ❌ | FOUNDATIONAL |
| Receipt | ✅ | ✅ | ✅ | ✅ | ✅ | FOUNDATIONAL |
| Canonical | ✅ | ✅ | ✅ | ❌ | ✅ | FOUNDATIONAL |
| Conformance | ✅ | ✅ | ✅ | ❌ | ✅ | FOUNDATIONAL |
| Accept | ❌ | ✅ | ✅ | ❌ | ✅ | FOUNDATIONAL |
| Verify | ✅ | ✅ | ✅ | ✅ | ✅ | FOUNDATIONAL |
| Proof | ✅ | ✅ | ✅ | ✅ | ✅ | FOUNDATIONAL |
| Allow/Deny/Decision | ✅ | ✅ | ✅ | ✅ | ✅ | FOUNDATIONAL |

12 of 13 primitives are foundational (appear across 3+ artifact families and survive platform/naming transitions). Custody is classified as BRIDGE — it appears in early application contexts (wallet custody) and later as a normative kernel concept (custody state), but the bridge between these uses is not yet formally specified.

### 3.3 Application vs. Foundation Separation

| Application Layer (built ON the kernel) | Foundational Mechanism (IS the kernel) |
|:---|:---|
| TrifoldWallet AI-Oracle Dashboard | Oracle quorum = early evidence→authority pattern |
| Vaultfire Ritual Console | Ritual Guardians = early custody state |
| Sovereign Data Vault | Cryptographic receipts = early proof/receipt structure |
| Triumvirate (Gemini/Aria/Capri) | Multi-agent consensus = application of threshold authority |
| LEGIO Protocol | Governance protocol = application of decision engine |
| ML Anomaly Detection (Isolation Forest) | Monitoring = evidence generation, not authority |
| TSCP Δ10.3–Δ10.6 deployments | Operational runtime = application context |
| Gnosis Safe 3/5 multisig | Threshold signing = application of evidence≠authority |
| Canonical serialization (TSCP-CANON-001) | **IS the kernel** — Component A |
| Non-self-referential receipts (Proof Envelope v2) | **IS the kernel** — Component B |
| evaluate() decision function | **IS the kernel** — Component C |
| "Evidence ≠ authority" invariant | **IS the kernel** — Component D |

---

## 4. Evolution Trace

For each kernel component: earliest evidence → intermediate forms → later formalization → current expression.

### Component A: Canonical Serialization
- **Earliest** (mid-2024): Smart contract artifacts requiring deterministic encoding
- **Intermediate** (Feb–Mar 2026): TSCP Δ10.5 canonical serialization suite
- **Formalized** (Jun 2026): TSCP-CANON-001.md v1.0 — formal spec with NFC normalization, key sorting, float prohibition, error taxonomy
- **Current**: canonical.ts (TypeScript implementation), conformance test suite (17 test cases), acceptance-receipt.json (cross-runtime verification: Python, Rust, TypeScript all pass 15/15)

### Component B: Non-Self-Referential Receipts
- **Earliest** (late 2024): Sovereign Data Vault txReceipt objects
- **Intermediate** (Jun 2026): acceptance-receipt.json — git-commit-bound cryptographic receipt
- **Formalized** (Aug 2026): Proof Envelope v2 (TSCP-PROOF-002) — explicit non-self-referential design
- **Current**: Specified, not fully implemented. Status: HOLD (per readiness review)

### Component C: Decision Function
- **Earliest** (early 2025): AI agent dual-layer arbors (allow/deny action filtering)
- **Intermediate** (Jun 2026): TSCP-CANON-001 error taxonomy (accept/reject with classified error codes)
- **Formalized** (Aug 2026): FOUNDING_DOCUMENT — evaluate() → Decision enum
- **Current**: Proposed only. No implementation exists.

### Component D: Separation Invariant
- **Earliest** (early 2024): TrifoldWallet oracle quorum (evidence feeds into, but does not equal, authority decision)
- **Intermediate** (Jan 2026): LEGIO protocol 3/5 threshold (authority requires consensus, not just evidence)
- **Formalized** (Aug 2026): "Evidence NEVER creates authority by itself"
- **Current**: Stated as central invariant. Not yet enforced in code.

---

## 5. Comparison Against Formal-Custody Framing

Current conceptual model: `specification → formally proven model → evidence-bound implementation → optimized implementation`

| Layer | Archive Status | Evidence |
|:---|:---|:---|
| Specification | ✅ EXISTS | TSCP-CANON-001.md v1.0, Proof Envelope v2 spec |
| Formally proven model | ❌ NOT YET | Lean proofs exist for NTT (B0-B2, C1-C3), NOT for kernel components |
| Evidence-bound implementation | ⚠️ PARTIAL | canonical.ts (implemented + verified), acceptance-receipt.json (cross-runtime), evaluate() (not implemented) |
| Optimized implementation | ❌ NOT YET | No optimized kernel implementation exists |

**"Inheritance, not competition" principle**: Already visible in acceptance-receipt.json — Python, Rust, and TypeScript implementations all inherit from TSCP-CANON-001 specification and produce identical canonical bytes. They do not compete; they inherit.

---

## 6. Kernel Candidate Assessment

### Candidate 1: Evidence-to-Authority Decision Engine (COMPOSITE)
- **What**: The four-component kernel described above (canonical serialization + non-self-referential receipts + decision function + separation invariant)
- **Earliest evidence**: Early 2024 (oracle quorum patterns in TrifoldWallet)
- **Strongest later evidence**: TSCP-CANON-001 cross-runtime verification (acceptance-receipt.json), Proof Envelope v2
- **Explains**: Why canonical serialization exists (evidence must be deterministic), why receipts are non-self-referential (circular hashing hazard), why conformance testing exists (implementations inherit from spec), why governance gates are read-only by default (prevent unauthorized transitions), why the Triumvirate is a consensus mechanism (authority ≠ evidence), why LEGIO has severity levels (different contract/predicate configurations)
- **Does NOT explain**: The specific ML anomaly detection architecture (application-specific), the specific blockchain wallet implementations (application-specific), the Swift/mobile integration details
- **Technically coherent?**: YES for Components A and B (specified, implemented, verified). PARTIALLY for Component C (well-defined but not implemented). DECLARED but UNENFORCED for Component D.
- **What must be formalized**: (1) Contract/predicate language, (2) custody state machine, (3) evidence binding protocol, (4) receipt verification protocol, (5) the connection between canonical serialization and decision evaluation, (6) Lean formal model

### Candidate 2: Canonical Serialization Alone (REDUCTIVE)
- **What**: TSCP-CANON-001 as the sole kernel primitive
- **Assessment**: Technically real and verified, but insufficient as a kernel — it explains serialization but not why serialization matters (authority decisions). It is a primitive the kernel depends on, not the kernel itself.

### Candidate 3: Multi-Agent Threshold Consensus (APPLICATION)
- **What**: 3/5 multisig consensus as the kernel
- **Assessment**: This is an application of the kernel, not the kernel itself. It uses evidence (signatures) and contracts (threshold rules) to produce authority decisions, but the consensus mechanism is one instantiation, not the general principle.

---

## 7. Disposition

### Classification: STRONGLY INDICATED

**Rationale:**
- 12 of 13 recurring primitives are foundational and appear across 3+ artifact families
- Component A (canonical serialization) is fully specified, implemented in 3 languages, and cross-verified
- Component B (non-self-referential receipts) is specified with clear cryptographic rationale
- Component C (decision function) is well-defined but not implemented
- Component D (separation invariant) is explicitly stated and consistent across the entire archive
- Application layers clearly separate from foundational mechanisms
- Evolution shows a clear distillation trajectory from applications → foundations
- The "inheritance, not competition" principle is already demonstrated in practice

**Why not FOUND:**
- The decision function (Component C) is proposed but not implemented
- The contract/predicate language is not yet specified
- The custody state machine is not yet formalized
- The connection between canonical serialization and decision evaluation is not proven
- No formal (Lean) model exists for any kernel component
- No adversarial testing has been performed

**Why not PLAUSIBLE:**
- The canonical serialization component is beyond plausible — it is specified, implemented, and cross-verified
- The acceptance-receipt.json demonstrates real cross-runtime agreement, not just architectural inference
- The recurrence patterns are too consistent across too many independent artifact families to be coincidental

---

## 8. Smallest Next Experiment

**Implement the evaluate() function as pure protocol logic.**

```typescript
type Decision =
  | { type: "Allow"; next_state: CustodyState }
  | { type: "Reject"; reason: string }
  | { type: "Hold"; reason: string }
  | { type: "Defer"; reason: string }

function evaluate(
  contract: Contract,
  evidence: CanonicalEvidence[],
  predicate: Predicate,
  current_state: CustodyState,
  proposed_transition: Transition
): Decision
```

This experiment would:
1. Prove the kernel is technically coherent (not just metaphor)
2. Test whether the four components compose correctly
3. Provide a target for adversarial testing (Stage 6 of founding document)
4. Give us something to compare against the archive
5. Discriminate between "the kernel is real" and "the kernel is a compelling narrative"

No dashboard. No database. No network. No blockchain. Pure protocol logic.

---

## 9. Repository-Placement Recommendation

Based on the archaeology:

| Option | Fit | Rationale |
|:---|:---|:---|
| A. New repository (`tscp-evidence-authority-kernel`) | ⚠️ Premature | Kernel not yet implemented; creating a repo before the charter is validated risks semantic drift |
| B. TSCP-PL | ⚠️ Conflated | The kernel is not the protocol language; it is a primitive the protocol language uses |
| C. TSCP Anchor | ⚠️ Conflated | The kernel is not the operational implementation; it is what the implementation inherits from |
| **D. Split architecture** | ✅ BEST FIT | Matches the "one canonical source of truth" principle from the founding document |

**Recommendation: Option D, but defer the decision until the evaluate() experiment succeeds.**

The first meaningful commit should be the Kernel Charter (this document + invariant registry + evidence matrix), establishing the constitutional layer before any implementation.

---

## 10. Non-Goals

- The kernel is NOT an AI agent framework
- The kernel is NOT a blockchain system
- The kernel is NOT a governance product
- The kernel is NOT a dashboard, database, or deployment platform
- The kernel does NOT depend on any specific programming language, runtime, or platform
- The kernel does NOT produce "grand narratives" — it produces deterministic decisions

## 11. Completion Criteria

The kernel is complete when:
1. All four components are formally specified
2. A Lean formal model exists and proves the separation invariant
3. A reference implementation exists in at least one language
4. A conformance corpus exists with fixtures and expected digests
5. The adversarial attack corpus (Stage 6) has been executed and all attacks are defended
6. An independent audit has verified the implementation against the specification
7. The kernel governs its own construction (recursive property, disciplined not mystical)

---

## Appendix: Investigation Artifacts

All archaeological analysis reports are stored in the sandbox:
- `ARCHAEOLOGY_INVENTORY.md` — Complete file inventory, nested zip extraction, chronology
- `ARCHAEOLOGY_MD_ANALYSIS.md` — All 13 Markdown files analyzed
- `ARCHAEOLOGY_CODE_ANALYSIS.md` — All TypeScript/Python code analyzed
- `ARCHAEOLOGY_DATA_ANALYSIS.md` — All JSON/TXT files analyzed
- `ARCHAEOLOGY_PDF_ANALYSIS.md` — All 9 PDFs and special-format files analyzed
- `ARCHAEOLOGY_RECURRING_PRIMITIVES.md` — Cross-cutting primitive synthesis with artifact family matrix
- `FOUNDING_DOCUMENT.md` — Aria's governing foundational document

All reports are read-only. No implementation changes were made. The original archive remains canonical on Google Drive.
