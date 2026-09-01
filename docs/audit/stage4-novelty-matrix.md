# Stage 4 — Novelty Comparison Matrix

**Date:** August 31, 2026
**Question:** Does the claimed TSCP contribution remain distinguishable from existing systems when the comparison is made at the level of actual mechanisms rather than terminology?
**Output format:** Finding/evidence matrix, not persuasive prose.

---

## TSCP's Three Contribution Axes

1. **Predicate separation:** Artifact identity (H(A)=h), resolution (Resolve(h)=A), and provenance binding (A=Canonicalize(Output(C))) are distinct predicates that existing systems frequently conflate. TSCP makes them explicit.

2. **Promotion semantics:** Custody promotion is modeled as a sequence of predicate-dependent state transitions, where promotion requires all preceding predicates to hold. Failed predicates cannot result in promotion.

3. **Certified composition:** A mechanically verified composition theorem (Lean 4) proves that certified bridges preserve admissibility, preservation, and reflection properties across verification systems.

**Critical context:** Axis 1 and 2's binding component (Bound(A,C,W)) is currently OPEN — not yet implemented. The novelty assessment is of the *framework*, not a completed system.

---

## Systems Compared

| # | System | Category | Key mechanism |
|---|---|---|---|
| S1 | in-toto | Supply chain attestation | Signed attestations binding metadata to artifact subjects, verified by policy engines |
| S2 | SLSA | Supply chain verification | Build provenance with verification levels (L1-L3), trusted builder model, expectation checking |
| S3 | PCD (Chiesa & Tromer) | Recursive proof composition | Proof-carrying data for distributed computations, recursive proof composition across steps |
| S4 | IVC (Nova/SuperNova/HyperNova) | Folding schemes | Folding multiple instances of CCS into one proof, within one constraint system |
| S5 | MAIF (2025) | Artifact-centric AI provenance | AI-native file format with embedded provenance chains, lifecycle states, cryptographic binding |
| S6 | zkVerify / zkBridge | Cross-system ZK verification | Universal verification layer for proofs from different systems; trustless cross-chain bridges |
| S7 | fiat-crypto / EverCrypt | Formal verification | End-to-end mechanical verification of cryptographic implementations |
| S8 | Clean / LambdaClass Lean 4 | ZK formal verification | Lean 4 formalization of ZK circuit correctness |

---

## Axis 1: Predicate Separation

**TSCP mechanism:** Five distinct predicates — identity, resolution, binding, verification, promotion — with explicit invariants and failure conditions for each.

| System | Separates identity from provenance? | Separates binding from verification? | Separates resolution from identity? | Has promotion as distinct predicate? | Verdict |
|---|---|---|---|---|---|
| S1 in-toto | Yes — Statement layer binds subject (digest) separately from Predicate (metadata) | No — binding and verification are both handled by the policy engine | No — resolution and identity are conflated (subject = digest set) | No — acceptance is a policy decision, not a predicate | PARTIALLY DISTINGUISHABLE |
| S2 SLSA | Yes — subject digest separate from build provenance | No — verification checks both provenance and expectations in one process | No — artifact identity and resolution are conflated | Partial — verification levels (L1-L3) act as gating but not formal predicates | PARTIALLY DISTINGUISHABLE |
| S3 PCD | No — no artifact identity concept | N/A — no artifact binding | N/A | No — no promotion concept | DISTINGUISHABLE |
| S4 IVC/HyperNova | No — no artifact identity concept | N/A — proves computation correctness, not artifact binding | N/A | No — no promotion concept | DISTINGUISHABLE |
| S5 MAIF | Yes — artifact has ID, root hash, provenance chain as separate fields | Partial — cryptographic binding is separate from access control verification | No — artifact identity and resolution are embedded in the file format | Partial — lifecycle states exist but are not predicate-dependent promotion | PARTIALLY DISTINGUISHABLE |
| S6 zkVerify | No — verifies proofs, not artifacts | N/A | N/A | No | DISTINGUISHABLE |
| S7 fiat-crypto | N/A — verifies implementations, not artifact provenance | N/A | N/A | N/A | DISTINGUISHABLE |
| S8 Clean | N/A — verifies circuit correctness, not artifact provenance | N/A | N/A | N/A | DISTINGUISHABLE |

**Finding (Axis 1):** TSCP's five-predicate separation (identity, resolution, binding, verification, promotion) is distinguishable from all compared systems. in-toto, SLSA, and MAIF partially separate some predicates (identity from provenance), but none separate binding from verification, and none have promotion as a distinct formal predicate. PCD/IVC/folding systems have no artifact identity concept at all.

**Caveat:** The *general idea* of separating concerns is not novel — every well-designed system separates some concerns. TSCP's specific five-predicate separation with formal invariants is the distinguishable element, not the concept of separation itself.

**Confidence:** MEDIUM-HIGH that this axis is distinguishable. The specific separation of binding from verification and promotion as a formal predicate appears genuinely novel. The separation of identity from resolution is a refinement, not a new concept.

---

## Axis 2: Promotion Semantics

**TSCP mechanism:** Formal state machine: Asserted → Resolve → Observe → Bind → Verify → Promote. Promotion invariant: Promote(x) ⟺ Resolve(x) ∧ Observe(x) ∧ Bind(x) ∧ Verify(x). Fail-closed: failure at any stage prevents promotion. State machine is formally modeled (FORMALLY_MODELED status).

| System | Has state machine? | Fail-closed? | Predicate-dependent? | Formally modeled? | Verdict |
|---|---|---|---|---|---|
| S1 in-toto | No — policy engine makes accept/reject decisions | Yes in practice (Binary Authorization blocks on failure) | No — policy checks are not formal predicates | No | PARTIALLY DISTINGUISHABLE |
| S2 SLSA | Partial — verification levels (L1-L3) act as gating stages | Yes — unrecognized parameters cause failure | Partial — each level requires different checks | No — process documented but not formalized as state machine | PARTIALLY DISTINGUISHABLE |
| S3 PCD | No | N/A | N/A | N/A | DISTINGUISHABLE |
| S4 IVC/HyperNova | No — folding is a computation, not a promotion | N/A | N/A | N/A | DISTINGUISHABLE |
| S5 MAIF | Yes — artifact has lifecycle states with adaptation rules | Unknown — adaptation rules may or may not fail-closed | Partial — transitions between states via adaptation rules | Partial — lifecycle metadata exists but formal model unclear | PARTIALLY DISTINGUISHABLE |
| S6 zkVerify | No — verifies proofs, no promotion | N/A | N/A | N/A | DISTINGUISHABLE |
| S7 fiat-crypto | N/A | N/A | N/A | N/A | DISTINGUISHABLE |
| S8 Clean | N/A | N/A | N/A | N/A | DISTINGUISHABLE |

**Finding (Axis 2):** Fail-closed verification exists in practice (in-toto/Binary Authorization, SLSA verification). MAIF has artifact lifecycle states. However, no compared system formalizes promotion as a state machine with predicate-dependent transitions and a formal promotion invariant. TSCP's formalization is more structured than existing approaches, but the *concept* of "accept only if all checks pass" is not novel.

**Caveat:** The formal state machine is a structuring contribution, not a deep mathematical result. The promotion invariant is straightforward. The novelty is in the formalization, not the complexity.

**Confidence:** MEDIUM that this axis is distinguishable. The formal state machine with predicate-dependent promotion is more structured than existing approaches, but fail-closed verification is well-established in supply chain security.

---

## Axis 3: Certified Composition

**TSCP mechanism:** Kernel/Universe/Bridge abstraction. Certified bridges prove preservation, reflection, and admissibility. Composition theorem (mechanically proven in Lean 4): if f: U → V and g: V → W have certificates, then g ∘ f: U → W has a certificate. This composes *verification properties across different proof systems*, not proofs across computation steps.

| System | Composes across proof systems? | Composes verification properties? | Has certified bridges? | Mechanically proven? | Verdict |
|---|---|---|---|---|---|
| S1 in-toto | No — single attestation framework | N/A | No | No | DISTINGUISHABLE |
| S2 SLSA | No — single verification framework | N/A | No | No | DISTINGUISHABLE |
| S3 PCD | No — composes proofs within one system, across computation steps | No — composes computational proofs, not verification properties | No | N/A | DISTINGUISHABLE |
| S4 IVC/HyperNova | No — folding within one CCS representation, not across systems | No — folds computation instances, not verification systems | No | N/A | DISTINGUISHABLE |
| S5 MAIF | No — single file format | No | No | No | DISTINGUISHABLE |
| S6 zkVerify | Partial — verifies proofs from different systems on one platform, but does not compose verification properties | No — verification aggregation, not property composition | No | No | PARTIALLY DISTINGUISHABLE |
| S6 zkBridge | Partial — bridges state across chains using ZK proofs, but does not compose verification properties across proof systems | No — cross-chain state verification, not property composition | No | No | PARTIALLY DISTINGUISHABLE |
| S7 fiat-crypto | No — verifies one implementation at a time | N/A | No | Yes (Coq) | DISTINGUISHABLE |
| S8 Clean | No — verifies one circuit at a time | N/A | No | Yes (Lean 4) | DISTINGUISHABLE |

**Finding (Axis 3):** This is the most distinguishable axis. No compared system formalizes cross-proof-system composition of verification properties. PCD/IVC compose proofs within one system across computation steps. zkVerify aggregates verification from different systems but does not compose verification properties. zkBridge bridges state across chains but does not compose verification properties. TSCP's certified bridges — proving that preservation, reflection, and admissibility compose across different verification universes — appears genuinely novel.

**Caveat:** The composition theorem itself is straightforward transitivity. The mathematical depth is low. The novelty is in the *abstraction* (Kernel/Universe/Bridge with certificates), not in the proof technique. A reviewer could argue this is "ordinary compositional reasoning formalized in Lean 4" rather than a new cryptographic contribution.

**Important distinction:** TSCP composes verification *properties across proof systems* (e.g., from a Plonky3 universe to a RISC Zero universe). PCD/IVC composes *proofs across computation steps within one system*. These are different operations at different abstraction levels. This distinction is TSCP's strongest novelty claim.

**Confidence:** MEDIUM-HIGH that this axis is distinguishable. The cross-system property composition abstraction appears novel, but the mathematical depth is limited.

---

## Cross-Axis Analysis: Does the *combination* distinguish TSCP?

| Property | in-toto/SLSA | PCD/IVC | MAIF | TSCP |
|---|---|---|---|---|
| Artifact identity separation | Partial | None | Partial | Yes (5 predicates) |
| Promotion state machine | No (policy) | No | Partial (lifecycle) | Yes (formal) |
| Cross-system certified composition | No | No (within-system) | No | Yes (Lean 4) |
| ZK proof of computation | No | Yes | No | Partial (binding OPEN) |
| Formal verification | No | No | Partial | Yes (Lean 4) |

**Finding (cross-axis):** No single compared system provides all three axes simultaneously. in-toto/SLSA provide partial predicate separation and practical fail-closed verification, but no cross-system composition and no ZK proofs. PCD/IVC provides recursive proof composition within one system, but no artifact identity, no promotion, and no cross-system bridges. MAIF provides artifact lifecycles and provenance chains, but no formal state machine, no cross-system composition, and no ZK proofs. TSCP's contribution is the *specific combination* of all three axes.

**Caveat:** "Combining existing ideas" is not by itself novel. The novelty must be in the technically necessary property that emerges from the combination — not merely that the combination hasn't been done before. The candidate emergent property is: a formal framework for composing custody verification across heterogeneous proof systems, with fail-closed promotion. Whether this is "technically necessary" (as opposed to merely convenient) is the open question for Stage 4.

---

## MAIF — Closest Comparable System

MAIF (November 2025) is the closest existing system to TSCP. Comparison at the mechanism level:

| Dimension | MAIF | TSCP | Distinguishable? |
|---|---|---|---|
| Approach | Artifact-embedded provenance (file format) | Protocol-layer provenance (separate from artifact) | Yes — different architectural approach |
| Provenance mechanism | Hash-chaining + signatures | ZK proofs of computation (OPEN) | Partially — both use cryptographic provenance, but different mechanisms |
| State transitions | Lifecycle states with adaptation rules | Formal state machine with predicate-dependent promotion | Yes — TSCP's is formally modeled with invariant |
| Artifact binding | Provenance chain embedded in artifact | Artifact-to-computation binding via ZK proof (OPEN) | Partially — different binding concepts |
| Cross-system composition | No | Certified bridges (Lean 4) | Yes |
| Formal verification | Partial (formal security model) | Yes (Lean 4, 83 theorems) | Partially |
| ZK proofs | No | Yes (partial implementation) | Yes |
| Target domain | AI agent trustworthiness / regulatory compliance | Multi-agent computation pipeline custody | Partially — overlapping but different focus |

**Finding (MAIF):** MAIF is the closest comparable but differs at the mechanism level: (1) MAIF embeds provenance in the artifact; TSCP separates it as a protocol layer. (2) MAIF uses hash-chaining; TSCP uses ZK proofs (when implemented). (3) MAIF has lifecycle states; TSCP has a formal state machine with predicate-dependent promotion. (4) MAIF has no cross-system composition; TSCP has certified bridges. The systems are distinguishable but address overlapping problems.

---

## Verdict

### Axis 1 (Predicate separation): DISTINGUISHABLE (MEDIUM-HIGH confidence)
The specific five-predicate separation with formal invariants is distinguishable. in-toto/SLSA/MAIF partially separate some predicates, but none separate binding from verification, and none have promotion as a formal predicate. The general concept of separation is not novel; the specific separation is.

### Axis 2 (Promotion semantics): DISTINGUISHABLE (MEDIUM confidence)
The formal state machine with predicate-dependent promotion is more structured than existing approaches. Fail-closed verification is not novel, but formalizing it as a state machine with a promotion invariant is a structuring contribution. No compared system formalizes promotion this way.

### Axis 3 (Certified composition): DISTINGUISHABLE (MEDIUM-HIGH confidence)
Cross-proof-system composition of verification properties appears genuinely novel. PCD/IVC compose within one system; zkVerify aggregates but doesn't compose properties; no system has certified bridges. The theorem is mathematically straightforward, but the abstraction is novel.

### Combination: DISTINGUISHABLE (MEDIUM confidence)
No single system provides all three axes. The candidate emergent property is a formal framework for composing custody verification across heterogeneous proof systems. Whether this is "technically necessary" remains an open question.

### Overall novelty assessment: QUALIFIED — DISTINGUISHABLE

TSCP's contribution is distinguishable from existing systems at the mechanism level, but the novelty is in the specific combination and formalization, not in any individual component. The contribution is best described as:

> A formal framework that separates artifact custody into five predicate-dependent stages with fail-closed promotion semantics, and provides mechanically verified certified bridges for composing verification properties across heterogeneous proof systems.

This is narrower than "ZK provenance" or "cryptographic custody verification" — it is the specific custody-state composition with cross-system certified bridges.

### Residual risk

A reviewer could still argue:
1. The combination, while novel, is not *technically necessary* — existing systems could achieve similar properties by composition without TSCP's formal framework.
2. The binding gap (OPEN) means the strongest claimed property (artifact-to-computation provenance) is not yet established, making the novelty assessment provisional.
3. The composition theorem's mathematical depth is low — "formalized transitivity" may not constitute a sufficient cryptographic contribution for a top venue.

These are Stage 4 risks, not Stage 3 adversarial findings. They are documented for completeness.
