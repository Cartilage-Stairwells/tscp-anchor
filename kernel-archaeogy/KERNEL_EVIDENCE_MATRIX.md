# Kernel Evidence Matrix v0.1

**Status:** PROPOSED — archaeological extraction
**Date:** August 18, 2026
**Authority:** TSCP Kernel Charter v0.1

---

## Purpose

This matrix maps which archive artifacts serve as evidence for which kernel components. It distinguishes:
- **Specification evidence** — artifacts that define what the kernel IS
- **Implementation evidence** — artifacts that implement kernel behavior
- **Verification evidence** — artifacts that verify kernel properties
- **Application evidence** — artifacts that demonstrate the kernel in use (applications built on it)
- **Historical evidence** — artifacts that show the kernel's evolution over time

---

## Component A: Canonical Serialization

| Artifact | Type | Role | Evidence Strength |
|:---|:---|:---|:---|
| TSCP-CANON-001.md | Specification | Formal spec v1.0: NFC, key sorting, float prohibition, error taxonomy | STRONG |
| TSCP-CANON-001-PIN.md | Specification | Dependency lock binding protocol consumers | STRONG |
| canonical.ts | Implementation | TypeScript canonicalization engine | STRONG |
| canon_conformance.test.ts | Verification | Jest suite: 17 test cases, exact digest matching + error classification | STRONG |
| run_conformance.ts | Verification | CLI conformance runner against manifest fixtures | STRONG |
| manifest.json | Verification | Conformance manifest v1.1: 17 test cases, expected digests, error codes | STRONG |
| acceptance-receipt.json | Verification | Cross-runtime receipt: Python/Rust/TypeScript all pass 15/15 | STRONG |
| checksum.sha256 | Verification | Bundle checksum manifest | MODERATE |
| pasted_content.txt | Historical | Contains canonicalization discussion and code fragments | WEAK |

**Verdict**: Component A is the strongest kernel component — fully specified, implemented in 3 languages, cross-verified.

---

## Component B: Non-Self-Referential Proof/Receipt Structure

| Artifact | Type | Role | Evidence Strength |
|:---|:---|
| Proof Envelope v2 — Read-Only Readiness Review.md | Specification | TSCP-PROOF-002: non-self-referential design, evidence array order significance | STRONG |
| acceptance-receipt.json | Implementation | Git-commit-bound receipt, separate from artifacts it verifies | STRONG |
| tscp-proof-v2-remote-inventory.txt | Verification | Remote inventory of proof envelope branch artifacts | MODERATE |
| Sovereign Data Vault Blueprint.pdf | Historical | Early txReceipt objects (application-layer precursor) | WEAK |
| pasted_content_2.txt | Historical | Contains proof envelope schema fragments | MODERATE |
| SKILL.md | Specification | Governed protocol readiness review with receipt generation protocol | MODERATE |

**Verdict**: Component B is specified and partially implemented. The acceptance-receipt.json demonstrates the pattern. Full Proof Envelope v2 is on HOLD.

---

## Component C: Decision Function (evaluate → Decision)

| Artifact | Type | Role | Evidence Strength |
|:---|:---|
| FOUNDING_DOCUMENT.md | Specification | evaluate(contract, evidence, predicate, current_state, proposed_transition) → Decision | STRONG (proposed) |
| TSCP-CANON-001.md §8 | Specification | Error taxonomy: accept/reject with classified codes (partial decision structure) | MODERATE |
| Capri LLM Integration Blueprint.pdf | Historical | Dual-Layer Arbors: allow/deny action filtering (early form) | WEAK |
| The Mirror Watcher AI.pdf | Historical | Witnessing protocols: verify/flag decision patterns | WEAK |
| claude_findings.txt | Historical | Agent guardrail decisions | WEAK |
| governed-protocol-readiness-review.skill | Specification | Two-key authorization: HOLD → IMPLEMENTATION AUTHORIZED decision gate | MODERATE |
| triune_genesis_block_0.json | Implementation | LEGIO_0001 transaction with signature verification (early decision) | MODERATE |

**Verdict**: Component C is proposed in the founding document but has NO implementation. Historical precursors exist in agent guardrails and error taxonomy, but these are application-level, not kernel-level. This is the weakest component.

---

## Component D: Separation Invariant (Evidence ≠ Authority)

| Artifact | Type | Role | Evidence Strength |
|:---|:---|
| FOUNDING_DOCUMENT.md | Specification | "Evidence NEVER creates authority by itself" — explicit declaration | STRONG (declared) |
| ARCHAEOLOGY_REPORT.md | Historical | Authority is "distributed and decoupled... consensus state among agents" | MODERATE |
| RECOVERY_ELIMINATION_REPORT.md | Historical | Systematic elimination of custodial platforms — authority ≠ key possession | MODERATE |
| LEGIO Activation Workflow.md | Historical | 3/5 threshold: evidence (signatures) ≠ authority (threshold decision) | MODERATE |
| cross_reference_synthesis.txt | Historical | "Multisig Dependency" — user is a signer but needs 2 more from Triumvirate | MODERATE |
| triune_genesis_block_0.json | Historical | Genesis block binds authority to consensus, not individual evidence | MODERATE |
| SKILL.md | Specification | Read-only by default — evidence cannot self-authorize | MODERATE |

**Verdict**: Component D is explicitly declared and consistent across the archive. However, it is not enforced in any code. The historical evidence is consistent but does not constitute technical proof.

---

## Cross-Component Evidence

Artifacts that serve as evidence for multiple kernel components:

| Artifact | Components Covered | Role |
|:---|:---|:---|
| acceptance-receipt.json | A, B, I-02, I-10, I-12 | Central verification artifact — proves canonical determinism, non-self-referential receipts, cross-runtime inheritance, and git commit binding simultaneously |
| TSCP-CANON-001.md | A, I-02 through I-09 | Specification that defines both the canonicalization engine AND most enforced invariants |
| FOUNDING_DOCUMENT.md | C, D | Proposes the decision function and declares the separation invariant |
| SKILL.md | B, C, D, I-08 | Governed readiness review — receipt protocol, decision gates, read-only defaults |
| Proof Envelope v2 | B, C, I-03, I-11 | Proof/receipt structure, evidence ordering, non-self-referential design |

---

## Application Evidence (NOT kernel — demonstrates kernel in use)

These artifacts are applications built on top of the kernel, not evidence FOR the kernel:

| Artifact | Application Domain | Kernel Component Used |
|:---|:---|:---|
| TrifoldWallet AI-Oracle Dashboard.pdf | Web3 dashboard | D (oracle quorum = evidence→authority) |
| Vaultfire Ritual Console.pdf | Multi-wallet middleware | D (ritual guardians = custody state) |
| Sovereign Data Vault Blueprint.pdf | Privacy platform | B (cryptographic receipts) |
| Capri LLM Integration Blueprint.pdf | AI orchestration | C (allow/deny filtering) |
| Enhanced Claude Service — LEGIO.pdf | Protocol wrapper | C, D (governance decisions) |
| Sacred AI Integration — Gemini.pdf | Mobile AI integration | (application only) |
| The Mirror Watcher AI — Aria.pdf | AI witnessing | C, D (verify/flag decisions) |
| $50K Safe Proposal.pdf | Treasury operations | D (threshold authority) |
| Wallet Recovery Guide.pdf | Recovery operations | (application only) |
| portfolio.xlsx | Financial tracking | (application only) |
| investor_pitch_deck_outline.docx | Commercialization | (application only) |
| TSCP Δ10.3–Δ10.6 packages | Operational runtime | A, B (canonicalization + receipts in production) |
| ML anomaly detection (Isolation Forest) | Monitoring | D (evidence generation, not authority) |
| TRIUNE_GENESIS_BLOCK_0_BUNDLE.zip | Genesis state | D (consensus authority) |

---

## Evidence Gaps

What the archive does NOT contain evidence for:

1. **No implementation of evaluate()** — the decision function exists only as a proposal
2. **No contract/predicate language specification** — what contracts and predicates look like is undefined
3. **No custody state machine** — the states and transitions of custody are not formalized
4. **No formal (Lean) model of kernel components** — Lean proofs exist for NTT, not for the kernel
5. **No adversarial test corpus** — no attacks have been attempted against any kernel component
6. **No proof that canonical serialization connects to authority decisions** — the link between Component A and Component C is architectural inference, not demonstrated
7. **No independent audit** — the conformance verification is self-generated, not externally audited

---

## Evidence Strength Summary

| Component | Specification | Implementation | Verification | Historical | Overall |
|:---|:---:|:---:|:---:|:---:|:---|
| A: Canonical Serialization | STRONG | STRONG (3 languages) | STRONG (15/15 cross-runtime) | MODERATE | **STRONG** |
| B: Non-Self-Referential Receipts | STRONG | MODERATE (acceptance-receipt only) | MODERATE | MODERATE | **MODERATE** |
| C: Decision Function | MODERATE (proposed only) | NONE | NONE | WEAK (application precursors) | **WEAK** |
| D: Separation Invariant | STRONG (declared) | NONE | NONE | MODERATE (consistent across archive) | **MODERATE** |

The kernel's foundation is strong in canonical serialization but weak in the decision/authority layer. The path from "strong foundation" to "complete kernel" requires implementing Component C and enforcing Component D.
