# Independent Review of Kernel Archaeology Results

**Reviewer:** Lyra (self-review, attacking own findings)
**Date:** August 18, 2026
**Subject:** KERNEL_CHARTER.md, KERNEL_INVARIANT_REGISTRY.md, KERNEL_EVIDENCE_MATRIX.md
**Mode:** Adversarial — treating own archaeology as a hypothesis to attack

---

## 0. Acknowledged Contamination

Before reviewing, I must disclose a methodological issue in my own archaeology.

The FOUNDING_DOCUMENT.md was saved to `tscp_delta/` BEFORE the six sub-agents were launched. The sub-agents therefore analyzed a 71-file directory (70 original files + the founding document). The recurring primitives analysis reports searching 134 non-node_modules files across the full extracted archive including nested zips, and the founding document was one of them.

This means the recurring primitives analysis may have been influenced by the founding document's own framing. The founding document explicitly proposes an "Evidence-to-Authority Kernel" with the four components I identified. If the sub-agents found those concepts recurring, some of that recurrence could be because the founding document was in the search space.

**Mitigation:** I verified that the key primitives (authority, evidence, canonical, receipt, proof, verify, allow/deny/decision) also appear in original archive files that predate the founding document (TSCP-CANON-001.md, acceptance-receipt.json, ARCHAEOLOGY_REPORT.md, LEGIO Activation Workflow.md, triune_genesis_block_0.json). The recurrence is not solely dependent on the founding document's presence. However, the specific FOUR-COMPONENT FRAMING (A/B/C/D) is an artifact of the founding document, not an independent archaeological discovery.

---

## 1. Does the archaeology justify calling this a kernel?

### Evidence that supports the conclusion

- TSCP-CANON-001 is a real, specified, implemented, cross-verified primitive. It is not retrospective interpretation — it is working code with matching SHA-256 digests across three runtimes.
- The acceptance-receipt.json binds this verification to a specific git commit, demonstrating the "inheritance, not competition" principle in practice.
- The non-self-referential receipt design (Proof Envelope v2) addresses a genuine cryptographic hazard (circular hashing), not a decorative pattern.
- The separation between evidence and authority is expressed consistently across multiple independent artifact families (multisig threshold, error taxonomy, governance gates) that were created at different times by different processes.

### Evidence that is merely retrospective interpretation

- The classification of TrifoldWallet's "oracle quorum" as an early form of "evidence→authority" is interpretation. TrifoldWallet was a Web3 dashboard; calling its oracle quorum a precursor to a kernel invariant is a narrative imposition.
- The classification of the AI agent "allow/deny" guardrails as a precursor to the Decision function is similarly retrospective. Those guardrails were application-level content filtering, not protocol-level state transition logic.
- The "distillation trajectory" narrative (applications → foundations) is a pattern I imposed on the chronology. The archive could equally represent a series of independent projects that share vocabulary but not architecture.

### Are the four components genuinely foundational?

- **Component A (Canonical Serialization):** GENUINELY FOUNDATIONAL. It is specified, implemented, and verified. Other artifacts depend on it. Removing it breaks the verification chain.
- **Component B (Non-Self-Referential Receipts):** GENUINELY FOUNDATIONAL as a design constraint, but PARTIALLY IMPLEMENTED. The constraint is real (circular hashing is a genuine hazard), but only one instance exists (acceptance-receipt.json).
- **Component C (Decision Function):** NOT FOUNDATIONAL. It is PROPOSED. There is no specification, no implementation, and no evidence in the archive that such a function has ever existed or been needed. It is an architectural proposal in the founding document, not an archaeological discovery.
- **Component D (Separation Invariant):** FOUNDATIONAL AS A PRINCIPLE, but its specific formulation ("Evidence NEVER creates authority by itself") is from the founding document, not from the archive. The archive contains APPLICATIONS of the principle (multisig threshold, governance gates) but not the principle itself as a formal statement.

### Verdict on "kernel"

The archaeology justifies calling canonical serialization a **foundational primitive**. It does NOT justify calling the four-component composite a **kernel**. The composite is an architectural inference from the founding document, supported but not demonstrated by the archive. The word "kernel" implies that the components compose into a minimal coherent whole — but the composition has never been demonstrated.

---

## 2. Independent evaluation of the four components

### Component A — Canonical Serialization

**Classification: FOUND**

**Strongest evidence:** acceptance-receipt.json — three independent implementations (Python, Rust, TypeScript) produce byte-identical canonical output across 15 test cases, verified against an externally generated manifest. This is not interpretation; this is a cryptographic fact.

**Largest uncertainty:** Is canonical serialization a KERNEL component or a UTILITY? Many systems use canonical JSON (RFC 8785, CBOR, Protocol Buffers). TSCP-CANON-001's specific rules (NFC normalization, float prohibition, scaled integers, error taxonomy) are well-designed, but they are not unique to a kernel that separates evidence from authority. The canonicalization could serve many purposes unrelated to authority decisions.

**Implication:** Component A is real but may be a DEPENDENCY of the kernel, not a COMPONENT of it. The kernel might depend on canonical serialization without canonical serialization being part of the kernel — just as a kernel depends on a memory allocator without the allocator being part of the kernel.

### Component B — Non-Self-Referential Receipts

**Classification: STRONGLY INDICATED**

**Strongest evidence:** Proof Envelope v2 specification defines the non-self-referential constraint with a clear cryptographic rationale. acceptance-receipt.json demonstrates the pattern in practice (receipt is separate from the artifacts it verifies). The SKILL.md governance protocol enforces the boundary as a design rule.

**Largest uncertainty:** The non-self-referential property is a CONSTRAINT (what not to do), not a MECHANISM (what to do). It tells us that receipts must be external, but it does not tell us what a receipt VERIFIES or how it BINDS to authority. The acceptance-receipt.json verifies canonicalization conformance — it does not verify authority decisions. A receipt for authority decisions does not exist in the archive.

### Component C — Decision Function

**Classification: UNSUPPORTED**

**Strongest evidence:** The founding document proposes `evaluate(contract, evidence, predicate, current_state, proposed_transition) → Decision`. The TSCP-CANON-001 error taxonomy (accept/reject with classified error codes) is a distant structural precursor.

**Largest uncertainty:** Everything. The function signature is defined but its semantics are entirely unspecified:
- What is a Contract? No type, no structure, no examples in the archive.
- What is a Predicate? No type, no structure, no examples.
- What is a CustodyState? No state machine, no states, no transitions.
- What is a Transition? No type, no structure.
- What is Evidence in this context? The archive has canonical serialization for evidence, but no definition of what makes canonical bytes "evidence" for a specific decision.

**Critical finding:** I searched the archive's TypeScript and JSON files for "evaluate", "decision", "Allow", "Reject", "Hold", "Defer" — ZERO results. The decision function exists ONLY in Markdown proposals. It has no presence in any code or data artifact.

### Component D — Separation Invariant

**Classification: PLAUSIBLE**

**Strongest evidence:** The invariant is consistently expressed across multiple independent contexts:
- LEGIO 3/5 multisig: evidence (signatures) does not equal authority (threshold decision)
- TSCP-CANON-001 error taxonomy: acceptance is a classified decision, not an automatic consequence of valid input
- SKILL.md: read-only by default — evidence cannot self-authorize
- ARCHAEOLOGY_REPORT: "authority is distributed and decoupled... consensus state among agents"

**Largest uncertainty:** Is "evidence ≠ authority" a TECHNICAL INVARIANT or a GOVERNANCE POLICY? In the archive, it is expressed as a policy (you need 3/5 signatures; you need two-key authorization). Policies can be violated by authorized actors. A technical invariant would be enforced by the mechanism itself — the evaluate() function would structurally prevent evidence from producing authority. But since evaluate() doesn't exist, the invariant is not technically enforced.

**Additional context from current work:** The TSCP anchor repository (not in the archive) contains the formal theorem `Reachable(Custody, AcceptanceReceipt) = true ∧ Reachable(Custody, Authority) = false`. This is a FORMAL REPRESENTATION of the separation invariant. But it lives in current work, not in the archive. The archive shows the principle's evolution; the current repos show its formalization.

---

## 3. Attack on the proposed boundary

### Is "evaluate(...) → Decision" the missing primitive?

**No. It is the right TARGET, but it is not the missing PRIMITIVE.**

The proposed evaluate() function assumes that its inputs (contract, evidence, predicate, current_state, proposed_transition) are already well-defined and that the function's job is to compose them into a decision. But none of these inputs are defined. The function signature is a TYPE SIGNATURE, not a SPECIFICATION.

### The deeper missing contract

There is a missing layer between canonical evidence and the decision function:

```
canonical evidence → ADMISSIBILITY → admissible evidence → evaluate(...) → Decision → CUSTODY TRANSITION → authority
```

The **admissibility contract** is the missing primitive. It defines:
1. What canonical evidence is ADMISSIBLE for a specific contract
2. How evidence BINDS to a contract (not all evidence is relevant to all contracts)
3. How evidence is REJECTED as inadmissible (wrong contract, wrong type, wrong binding)

Without admissibility, evaluate() faces an impossible choice:
- **Accept all canonical evidence** → evidence smuggles into the decision, violating the separation invariant
- **Define admissibility inside evaluate()** → the contract layer and decision layer are conflated
- **Define admissibility outside evaluate() without specification** → undocumented behavior

None of these are acceptable. The admissibility contract must be specified BEFORE evaluate() can be meaningfully implemented.

### Similarly, the output side is undefined

A Decision (Allow/Reject/Hold/Defer) is not authority. There must be a **custody transition contract** that defines how a Decision becomes a custody state change, and how a custody state change constitutes authority. Without this, evaluate() produces decisions that go nowhere.

The full missing chain is:
```
canonical evidence → admissibility contract → admissible evidence → evaluate() → Decision → custody transition contract → custody state → authority binding → authority
```

The founding document's Stage 5 proposal collapses this entire chain into a single function call. That is architecturally attractive but specification- premature.

---

## 4. Evidence vs. authority separation

### Current status in the archive

| Level | Status | Evidence |
|:---|:---|:---|
| Documented | ✅ YES | Founding document, LEGIO, ARCHAEOLOGY_REPORT, SKILL.md |
| Structurally represented | ⚠️ PARTIALLY | 3/5 multisig (evidence ≠ authority by threshold), error taxonomy (accept ≠ automatic), governance gates (read-only by default) |
| Mechanically enforced | ❌ NO | No code prevents evidence from being presented as authority. No function rejects evidence-only authority claims. |
| Formally proven | ❌ NO (in archive) | The TSCP anchor theorem `Reachable(Custody, Authority) = false` exists in current repos, not in the archive. Even there, it proves custody cannot reach authority — it does not prove that a proposed evaluate() function would preserve this property. |

### Critical assessment

The separation is DOCUMENTED and PARTIALLY STRUCTURALLY REPRESENTED. It is NOT MECHANICALLY ENFORCED or FORMALLY PROVEN within the archive.

If we implement evaluate() without an explicit, mechanical enforcement of the separation, we would be building a system that CLAIMS to separate evidence from authority while relying on convention to maintain the separation. That is exactly the failure mode the founding document warns against: "Do not turn historical recurrence into technical validity."

The separation must be mechanically enforced in the evaluate() function's type system or execution semantics. Specifically:
- The Evidence type must not contain any field that could be interpreted as authority (no signatures, no thresholds, no authorization claims)
- The Decision type must not be representable as Evidence (a Decision cannot be fed back as evidence for another decision)
- The Contract type must not be constructable from Evidence (a contract cannot be established by evidence alone)

These are TYPE-LEVEL constraints that must be specified before implementation.

---

## 5. Comparison against the formal-custody model

### The formal-custody model

From the current TSCP work (not the archive):
- 5-stage chain: Specification → Identity Binding → Integrity → Conformance → External Reproduction
- FCO Transition Algebra v1.1
- Key theorem: `Reachable(Custody, AcceptanceReceipt) = true ∧ Reachable(Custody, Authority) = false`
- Principle: "No proof may gain authority by depending on a layer that is weaker than its conclusion"
- 5 failure modes: False Shoreline, Self-Attestation, Authority Confusion, Provenance Gap, Semantic Drift

### Does the proposed kernel realize this principle?

**For Component A (canonical serialization): YES.**
- Specification exists (TSCP-CANON-001)
- Identity binding exists (git commit hash in acceptance-receipt.json)
- Integrity exists (SHA-256 digests)
- Conformance exists (15/15 cross-runtime pass)
- External reproduction exists (3 independent implementations)
- "Inheritance, not competition" is DEMONSTRATED

**For Component B (receipts): PARTIALLY.**
- Specification exists (Proof Envelope v2)
- Identity binding exists (git commit hash)
- Integrity exists (SHA-256)
- Conformance: NOT YET (on HOLD)
- External reproduction: NOT YET

**For Component C (decision function): NO.**
- No specification, no identity binding, no integrity, no conformance, no reproduction
- The principle is not realized; it is not even specified

**For Component D (separation invariant): PARTIALLY.**
- The principle is DOCUMENTED and STRUCTURALLY REPRESENTED in the current TSCP anchor work
- But it is not MECHANICALLY ENFORCED in any implementation
- The formal theorem exists in current repos but not in the archive
- The proposed kernel does not yet contain a mechanism that enforces the separation

### Verdict

The proposed kernel RESEMBLES the formal-custody model in its canonicalization layer but does NOT REALIZE it for the decision/authority layer. The canonicalization layer is a genuine instance of the formal-custody model. The decision/authority layer is a proposal that is consistent with the model's principles but does not yet instantiate them.

---

## 6. Should implementation proceed?

### Assessment

The evaluate() function is the correct TARGET — it is the smallest experiment that would discriminate between "the kernel is real" and "the kernel is a compelling narrative." However, it is PREMATURE because:

1. **The admissibility contract is undefined.** Without specifying what makes canonical evidence admissible to a specific contract, evaluate() cannot be implemented without either smuggling authority into evidence or conflating layers.

2. **The input types are unspecified.** Contract, Predicate, CustodyState, and Transition have no type definitions, no examples, and no instances in the archive. Implementing evaluate() with undefined inputs is not an experiment — it is a type signature with empty semantics.

3. **The separation invariant is not mechanically enforceable yet.** Without type-level constraints that prevent Evidence from containing authority, any implementation would rely on convention rather than mechanism.

4. **The custody transition is undefined.** A Decision is not authority. Without a custody transition contract, evaluate() produces decisions that cannot become authority.

### The smallest specification artifact that must exist first

**The Admissibility Contract Specification** — a document that defines:

1. What is a Contract (type, structure, validity conditions)
2. What is Evidence in the context of a Contract (binding rules, type constraints)
3. What makes Evidence ADMISSIBLE to a specific Contract (relevance, binding, canonicalization)
4. What makes Evidence INADMISSIBLE (wrong contract, wrong type, insufficient binding)
5. How admissibility is DETERMINED (deterministic function, not human judgment)
6. How admissibility ENFORCES the separation invariant (evidence cannot self-admit)

This is a SPECIFICATION artifact, not an implementation. It must exist before evaluate() can be meaningfully implemented. It is the "missing primitive" between canonical evidence and the decision function.

### Minimum contract evaluate() must satisfy before implementation is authorized

If the admissibility contract is specified, then evaluate() must satisfy:

1. **Input completeness:** All five inputs (contract, evidence, predicate, current_state, proposed_transition) must have typed definitions with validation rules
2. **Admissibility enforcement:** evaluate() must reject evidence that has not passed the admissibility contract — it must not evaluate inadmissible evidence
3. **Separation enforcement:** The Evidence type must not contain any field that could be interpreted as authority; the Decision type must not be representable as Evidence
4. **Determinism:** evaluate() must be a pure function — same inputs always produce the same Decision
5. **Error classification:** Rejections must be classified by stage and code, following the TSCP-CANON-001 pattern
6. **Non-self-referential:** A Decision must not be embeddable in Evidence for a subsequent evaluate() call that concerns the same transition

---

## 7. Disposition

### **HOLD**

The evaluate() experiment is the correct next discriminating move, but its contract cannot be defined without first specifying the admissibility layer that sits between canonical evidence and the decision function.

**The smallest next artifact is not code. It is a specification: the Admissibility Contract.**

This specification must define what makes canonical evidence admissible to a specific contract, how admissibility is determined, and how admissibility enforces the separation invariant. Once this specification exists, evaluate() can be implemented as a pure function that operates on admissible inputs and produces classified decisions.

**What I got right in the archaeology:**
- Component A is genuinely FOUND — this is the strongest result and is not retrospective interpretation
- The recurring primitives analysis is sound for the original archive files (minus the founding document contamination)
- The evidence matrix accurately maps which artifacts evidence which components
- The disposition of STRONGLY INDICATED for the overall kernel is defensible IF the kernel is understood as "canonical serialization + a proposed but unspecified decision architecture"

**What I got wrong:**
- I proposed evaluate() as the "smallest next experiment" without recognizing that its inputs are undefined
- I classified the kernel as a four-component composite when only one component is actually implemented
- I did not identify the admissibility gap — the missing contract between canonical evidence and the decision function
- I did not adequately distinguish between the archive's evidence and the current TSCP anchor's formal work when comparing against the formal-custody model
- I allowed the founding document's framing to structure my archaeology rather than letting the evidence structure itself

**What this means for the project:**

The archaeology confirms that there IS something real in the archive — canonical serialization is a genuine, verified primitive. But the "kernel" as a four-component whole is an architectural proposal, not an archaeological discovery. The next step is not to implement the proposal but to specify the missing layer that would make the proposal coherent: the admissibility contract.

That specification is the first artifact of the new era — not the Kernel Charter (which I already wrote), but the Admissibility Contract that defines how canonical evidence becomes admissible evidence for a specific decision. This is where the separation between evidence and authority becomes mechanical rather than documented.
