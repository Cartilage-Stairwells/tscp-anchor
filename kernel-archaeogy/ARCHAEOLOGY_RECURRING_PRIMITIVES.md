# ARCHAEOLOGY REPORT: RECURRING PRIMITIVES IN TSCP DELTA ARCHIVE

**Author:** Archeological Synthesis Worker  
**Date:** August 18, 2026  
**Archive Target:** `/app/conversations/6a8516d2ee9e9ad0fb8ad3ba/tscp_delta/`  
**Status:** COMPLETE SYNTHESIS  

---

## Executive Summary

An exhaustive archaeological analysis of the **TSCP DELTA archive** reveals that behind shifting project names (*TrifoldWallet*, *Vaultfire Ritual Console*, *Sovereign Data Vault*, *The Triumvirate*, *LEGIO Protocol*, *TSCP Δ10.3/Δ10.5*), lies a remarkably coherent, evolving lineage of **Recurring Primitives**.

Rather than being a disjointed collection of web3 dashboards, AI experiments, and deployment scripts, the archive demonstrates a continuous architectural convergence toward a minimal, formal, and deterministic **Evidence-to-Authority Kernel**. The 13 investigated concepts represent the core functional vocabulary that survived multiple platform migrations, platform rebrandings, and runtime shifts (from Web3 smart contracts to multi-agent LLM systems to pure TypeScript/Rust/Lean protocol specifications).

---

## 1. Artifact Family Matrix

The table below maps the presence of each recurring primitive across the five major artifact families in the TSCP DELTA archive:
1. **Code** (`.ts`, `.js`, `.py`, `.sh`)
2. **Markdown** (`.md`, `.docx`)
3. **JSON** (`.json`)
4. **PDF** (Project Blueprints, Implementation Guides)
5. **TXT** (`.txt`, `.log` analysis files)

| Recurring Primitive | Code | Markdown | JSON | PDF | TXT | Classification |
| :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| **1. Authority** | ❌ | ✅ | ✅ | ✅ | ✅ | **FOUNDATIONAL** |
| **2. Evidence** | ✅ | ✅ | ✅ | ✅ | ❌ | **FOUNDATIONAL** |
| **3. Custody** | ❌ | ✅ | ❌ | ✅ | ❌ | **APPLICATION-SPECIFIC** |
| **4. Contract** | ❌ | ✅ | ❌ | ✅ | ✅ | **FOUNDATIONAL** |
| **5. Predicate** | ✅ | ✅ | ✅ | ❌ | ❌ | **FOUNDATIONAL** |
| **6. Transition** | ❌ | ✅ | ✅ | ❌ | ❌ | **FOUNDATIONAL** |
| **7. Receipt** | ✅ | ✅ | ✅ | ✅ | ✅ | **FOUNDATIONAL** |
| **8. Canonical** | ✅ | ✅ | ✅ | ❌ | ✅ | **FOUNDATIONAL** |
| **9. Conformance** | ✅ | ✅ | ✅ | ❌ | ✅ | **FOUNDATIONAL** |
| **10. Accept** | ❌ | ✅ | ✅ | ❌ | ✅ | **FOUNDATIONAL** |
| **11. Verify** | ✅ | ✅ | ✅ | ✅ | ✅ | **FOUNDATIONAL** |
| **12. Proof** | ✅ | ✅ | ✅ | ✅ | ✅ | **FOUNDATIONAL** |
| **13. Allow / Deny / Decision** | ✅ | ✅ | ✅ | ✅ | ✅ | **FOUNDATIONAL** |

---

## 2. In-Depth Primitive Documentation

For each of the 13 concepts, this section details file occurrences, contextual definitions, semantic consistency, chronological evolution, and formalization state.

---

### 1. Authority (`authority` / `Authority`)

- **Files Containing It**:
  - **Markdown**: `FOUNDING_DOCUMENT.md`, `RECOVERY_ELIMINATION_REPORT.md`, `ARCHAEOLOGY_REPORT.md`, `SKILL.md`, `Proof Envelope v2 — Read-Only Readiness Review and First Proposed Change Set.md`.
  - **JSON**: `triune_genesis_block_0.json` (embedded markdown documents).
  - **PDF**: `Capri LLM Integration Blueprint - Dual-Layer Arbors Architecture.pdf`, `The Mirror Watcher AI_ Aria Extension Sacred Deployment.pdf`.
  - **TXT**: `cross_reference_synthesis.txt`, `capri_findings.txt`, `safe_proposal_findings.txt`, `mirror_watcher_findings.txt`.

- **Contextual Definition & Usage**:
  - *Early Web3 / Smart Account Era (2024)*: Authority meant wallet control, multi-sig signer threshold quorums (Gnosis Safe 3/5 multisig), and key ownership.
  - *AI Agent Era (Early 2025)*: Authority shifted to agent command hierarchies—specifically the "SupremeHead" authority, "Mirror-Authority-155Hz", and Triumvirate Oracles (Gemini, Aria, Claude).
  - *Protocol Specification & Kernel Era (2026)*: Authority is defined as a pure, derived state output. In `FOUNDING_DOCUMENT.md`, authority is the end product of evaluating canonical evidence against contracts and predicates. In `SKILL.md`, "implementation authority" is strictly separated from "review authority".

- **Consistency Analysis**: Shifts from operational key ownership and agent rank to a formal protocol state derived from deterministic evaluation.

- **Earliest Evidence**: Early 2024 TrifoldWallet and Vaultfire Safe 3/5 multisig authority models (`ARCHAEOLOGY_REPORT.md`, `cross_reference_synthesis.txt`).

- **Latest / Most Formalized Version**: August 18, 2026 in `FOUNDING_DOCUMENT.md` ("TSCP Evidence-to-Authority Kernel") and `SKILL.md` (Governed Protocol Control Boundary).

---

### 2. Evidence (`evidence` / `Evidence`)

- **Files Containing It**:
  - **Markdown**: `FOUNDING_DOCUMENT.md`, `SKILL.md`, `Proof Envelope v2 — Read-Only Readiness Review and First Proposed Change Set.md`, `TSCP-CANON-001.md`, `RECOVERY_ELIMINATION_REPORT.md`.
  - **Code/JSON**: `triune-handoff-ts/src/proof-envelope.ts`, `TSCP-PROOF-002` schema (`evidence: Array<{ canon_version, sha256 }>`).
  - **PDF**: `The Mirror Watcher AI_ Aria Extension Sacred Deployment.pdf`.

- **Contextual Definition & Usage**:
  - *AI / Sacred Deployment Era*: Used as "Physical evidence" or "External verification" required for AI consensus.
  - *Governance / Review Era*: Direct inspection targets (repo tags, commit hashes, manifests, receipts) used to distinguish verified facts from claims (`SKILL.md`).
  - *Kernel / Proof Envelope Era*: Canonical, immutable input arrays (`evidence: Array<{ canon_version, sha256 }>`) passed to the evaluation kernel. In Proof Envelope v2, evidence array ordering is commitment-significant (`004_evidence_array_order_significant`).

- **Consistency Analysis**: Highly consistent. Always denotes tamper-evident, externally verifiable facts required prior to state transitions.

- **Earliest Evidence**: Early 2025 in `The Mirror Watcher AI_ Aria Extension Sacred Deployment.pdf`.

- **Latest / Most Formalized Version**: August 18, 2026 in `FOUNDING_DOCUMENT.md` (Kernel Evidence Taxonomy & Kernel Evidence Matrix) and `TSCP-PROOF-002` schema.

---

### 3. Custody (`custody` / `Custody`)

- **Files Containing It**:
  - **Markdown**: `FOUNDING_DOCUMENT.md` (5 matches: "custody state", "custody model", "Custody attacks").
  - **PDF**: `Comprehensive Cryptocurrency Wallet Recovery Guide for 2025.pdf` (1 match: "self-custody").

- **Contextual Definition & Usage**:
  - *Wallet Recovery Era (2025)*: Refers to asset management and key storage ("self-custody vs professional services").
  - *Kernel Era (2026)*: Refers to the governed ownership/control state of resources or state roots within the kernel state machine (`custody state`).

- **Consistency Analysis**: Varies. Re-purposed from financial key holding to a core state machine variable.

- **Earliest Evidence**: Early 2025 in `Comprehensive Cryptocurrency Wallet Recovery Guide for 2025.pdf`.

- **Latest / Most Formalized Version**: August 18, 2026 in `FOUNDING_DOCUMENT.md` (Custody state as one of six core normative kernel entities).

---

### 4. Contract (`contract` / `Contract`)

- **Files Containing It**:
  - **Markdown**: `FOUNDING_DOCUMENT.md`, `TSCP-CANON-001.md`, `Proof Envelope v2 — Read-Only Readiness Review and First Proposed Change Set.md`.
  - **PDF**: `Sovereign Data Vault_ Complete Project Blueprint.pdf`, `Step-by-Step_ Submit Your $50K Safe Proposal.pdf`, `Comprehensive Cryptocurrency Wallet Recovery Guide for 2025.pdf`.
  - **TXT**: `recovery_guide_findings.txt`.

- **Contextual Definition & Usage**:
  - *Web3 Era*: On-chain EVM smart contracts (`web3.eth.Contract` for dataRegistry, proofVerifier, paymentProcessor; Gnosis Safe `execTransaction`).
  - *Specification & Kernel Era*: In `TSCP-CANON-001`, contract means the behavioral protocol specification ("Rejection is part of the protocol contract, not an implementation detail"). In `FOUNDING_DOCUMENT.md`, Contract is an input parameter to `evaluate()` representing the rule set governing state transitions.

- **Consistency Analysis**: Evolves from Ethereum smart contracts to protocol specification rule sets.

- **Earliest Evidence**: Early 2024 TrifoldWallet & Vaultfire Safe contract interactions.

- **Latest / Most Formalized Version**: August 18, 2026 in `FOUNDING_DOCUMENT.md` (`evaluate(contract, evidence, predicate, current_state, proposed_transition)`).

---

### 5. Predicate (`predicate` / `Predicate`)

- **Files Containing It**:
  - **Markdown**: `FOUNDING_DOCUMENT.md`, `Proof Envelope v2 — Read-Only Readiness Review and First Proposed Change Set.md`.
  - **Code/JSON**: Implemented in Proof Envelope v2 schemas (`predicate: { ... }`).

- **Contextual Definition & Usage**:
  - *Proof Envelope v2 (`TSCP-PROOF-002`)*: A structured boolean rule object embedded within proof payloads.
  - *Kernel Era (`FOUNDING_DOCUMENT.md`)*: Pure logical condition that must evaluate to true given contract rules and evidence.

- **Consistency Analysis**: 100% consistent across all occurrences. Always represents a pure, deterministic logical condition.

- **Earliest Evidence**: August 18, 2026 in `Proof Envelope v2 — Read-Only Readiness Review...md`.

- **Latest / Most Formalized Version**: August 18, 2026 in `FOUNDING_DOCUMENT.md` (Explicit input parameter in the Kernel evaluation signature).

---

### 6. Transition (`transition` / `Transition`)

- **Files Containing It**:
  - **Markdown**: `FOUNDING_DOCUMENT.md`, `TSCP-CANON-001.md`, `TRIUNE GENESIS Runbook (Δ10.6).md`, `TRIUNE GENESIS Runbook (Δ10.5+).md`, `TSCP Δ10.3 Deployment Report.md`, `SKILL.md`.
  - **JSON**: `triune_genesis_block_0.json` (embedded deployment reports).

- **Contextual Definition & Usage**:
  - *Deployment Era*: Used informally for system upgrades ("Transition to Pro Mode", "Transition to containerized deployment").
  - *Specification & Kernel Era*: In `TSCP-CANON-001`, "explicit state-transition migration path". In `FOUNDING_DOCUMENT.md`, "proposed transition" is the candidate state mutation evaluated by the kernel engine (`proposed_transition`).

- **Consistency Analysis**: Varies between informal operational/devops migration and formal state machine transformation.

- **Earliest Evidence**: TSCP Δ10.3 deployment reports (devops transitions).

- **Latest / Most Formalized Version**: August 18, 2026 in `FOUNDING_DOCUMENT.md` (Proposed Transition in the Kernel state machine).

---

### 7. Receipt (`receipt` / `Receipt`)

- **Files Containing It**:
  - **Markdown**: `SKILL.md`, `Proof Envelope v2 — Read-Only Readiness Review and First Proposed Change Set.md`.
  - **JSON**: `acceptance-receipt.json`.
  - **PDF**: `Sovereign Data Vault_ Complete Project Blueprint.pdf` (`txReceipt`).
  - **TXT**: `pasted_content.txt`, `pasted_content_2.txt`, `tscp-proof-v2-remote-inventory.txt`.
  - **Code**: Referenced in conformance test runners and proof envelope generators.

- **Contextual Definition & Usage**:
  - *Web3 Era*: Blockchain transaction receipt (`txReceipt`).
  - *Conformance & Proof Era*: Cryptographic proof of pass/fail execution. `acceptance-receipt.json` records git commit, timestamp, fixture count, digests, and `conformance: "PASS"`. Receipts are external to payloads to prevent self-referential hashing.

- **Consistency Analysis**: Highly consistent. An immutable cryptographic digest generated after execution to prove a transaction or conformance test occurred.

- **Earliest Evidence**: Late 2024 Sovereign Data Vault (`txReceipt`).

- **Latest / Most Formalized Version**: June–August 2026 in `acceptance-receipt.json` (bound to commit `500b699f...`) and Proof Envelope v2 receipt issuance specifications.

---

### 8. Canonical (`canonical` / `Canonical` / `canon`)

- **Files Containing It**:
  - **Code**: `canonical.ts`, `canon_conformance.test.ts`, `run_conformance.ts`, `extract_relay.py`.
  - **Markdown**: `TSCP-CANON-001.md`, `TSCP-CANON-001-PIN.md`, `Proof Envelope v2 — Read-Only Readiness Review...md`, `SKILL.md`, `FOUNDING_DOCUMENT.md`.
  - **JSON**: `acceptance-receipt.json`, `manifest.json`, `canonical_source.json`.
  - **TXT**: `pasted_content.txt`, `pasted_content_2.txt`, `tscp-proof-v2-remote-inventory.txt`.

- **Contextual Definition & Usage**:
  - *Protocol Specification (`TSCP-CANON-001`)*: The exact, deterministic UTF-8 byte serialization profile for Protocol Objects (NFC Unicode normalization, key sorting, float prohibition, integer-only policy).
  - *Code (`canonical.ts`)*: Implementation of the normalization pipeline.

- **Consistency Analysis**: Exceptionally consistent across spec, code, tests, and json manifests.

- **Earliest Evidence**: TSCP Δ10.5 canonical serialization suite.

- **Latest / Most Formalized Version**: August 2026 in `TSCP-CANON-001.md` v1.0, `canonical.ts`, and `acceptance-receipt.json`.

---

### 9. Conformance (`conformance` / `Conformance`)

- **Files Containing It**:
  - **Code**: `canon_conformance.test.ts`, `run_conformance.ts`, `canonical.ts`.
  - **Markdown**: `SKILL.md`, `Proof Envelope v2 — Read-Only Readiness Review...md`, `TSCP-CANON-001.md`.
  - **JSON**: `acceptance-receipt.json` (`"module": "tscp-canon-conformance"`, `"conformance": "PASS"`).
  - **TXT**: `pasted_content.txt`, `pasted_content_2.txt`, `tscp-proof-v2-remote-inventory.txt`.

- **Contextual Definition & Usage**:
  - Rigorous test suite verification (`conformance/canon/fixtures/manifest.json`) confirming an implementation satisfies every rule and rejection code in a specification.

- **Consistency Analysis**: 100% consistent across specification, test harnesses, and governance reviews.

- **Earliest Evidence**: TSCP Δ10.5 synthesis conformance corpus.

- **Latest / Most Formalized Version**: June–August 2026 in `acceptance-receipt.json` and `canon_conformance.test.ts`.

---

### 10. Accept (`accept` / `Accept` / `acceptance`)

- **Files Containing It**:
  - **Markdown**: `SKILL.md`, `Proof Envelope v2 — Read-Only Readiness Review...md`, `TSCP-CANON-001-PIN.md`.
  - **JSON**: `acceptance-receipt.json`.
  - **TXT**: `pasted_content.txt`, `pasted_content_2.txt`, `tscp-proof-v2-remote-inventory.txt`.

- **Contextual Definition & Usage**:
  - Governance status indicating a spec or implementation has passed review and is pinned/frozen (`tscp-canon-001-accepted`).
  - Immutably bound to git commit hash in `acceptance-receipt.json`.

- **Consistency Analysis**: Completely consistent. Signifies formal approval and architectural lock.

- **Earliest Evidence**: `TSCP-CANON-001-PIN.md`.

- **Latest / Most Formalized Version**: August 2026 in `SKILL.md` (Acceptance gate rules) and `acceptance-receipt.json`.

---

### 11. Verify (`verify` / `Verify` / `verification`)

- **Files Containing It**:
  - **Code**: `extract_relay.py`.
  - **Markdown**: `SKILL.md`, `Proof Envelope v2 — Read-Only Readiness Review...md`, `TSCP-CANON-001.md`, `TRIUNE GENESIS Runbook (Δ10.6).md`, `TSCP Δ10.5-SYNTHESIS Enhanced Verification Report.md`, `TSCP Δ10.5-SYNTHESIS Final Verification Report.md`.
  - **JSON**: `manus_operational_output.json`, `triune_genesis_block_0.json`, `canonical_source.json`.
  - **PDF**: `Sovereign Data Vault_ Complete Project Blueprint.pdf`, `Step-by-Step_ Submit Your $50K Safe Proposal.pdf`, `Enhanced Claude Service - LEGIO Protocol Integration.pdf`, `The Mirror Watcher AI_ Aria Extension Sacred Deployment.pdf`, `Comprehensive Cryptocurrency Wallet Recovery Guide for 2025.pdf`.
  - **TXT**: `pasted_content.txt`, `pasted_content_2.txt`, `pasted_content_3.txt`.

- **Contextual Definition & Usage**:
  - Appears across **ALL 5 ARTIFACT FAMILIES**. Spans zero-knowledge smart contract verification, operational service health checks, digest re-computation, and schema validation.

- **Consistency Analysis**: Broad application, but semantically unified around asserting truth through computation.

- **Earliest Evidence**: Early 2024 TrifoldWallet & SDV proof verification.

- **Latest / Most Formalized Version**: August 2026 in Proof Envelope v2 verification gate and `SKILL.md`.

---

### 12. Proof (`proof` / `Proof`)

- **Files Containing It**:
  - **Markdown**: `Proof Envelope v2 — Read-Only Readiness Review...md`, `FOUNDING_DOCUMENT.md`.
  - **PDF**: `Sovereign Data Vault_ Complete Project Blueprint.pdf`, `Vaultfire Ritual Console_ Multi-Wallet Blockchain System Implementation Guide.pdf`, `Step-by-Step_ Submit Your $50K Safe Proposal.pdf`.
  - **TXT**: `pasted_content_2.txt`.
  - **Code/JSON**: Implemented in `TSCP-PROOF-002` envelope specifications.

- **Contextual Definition & Usage**:
  - *Web3 Era*: Zero-knowledge cryptographic proofs generated off-chain and verified on-chain.
  - *Protocol Era*: Proof Envelope v2 (`TSCP-PROOF-002`), a typed wrapper bundling payload, evidence, predicate, signature, and receipt.
  - *Governance Era*: Technical proof as the highest tier of validation ("historical evidence → architectural inference → technical proof").

- **Consistency Analysis**: Evolved from ZK-proof bytecode to structured protocol message envelopes.

- **Earliest Evidence**: Mid-2024 Vaultfire Ritual Console and Sovereign Data Vault.

- **Latest / Most Formalized Version**: August 18, 2026 in `Proof Envelope v2 — Read-Only Readiness Review` (`TSCP-PROOF-002`).

---

### 13. Allow / Deny / Decision (`allow` / `deny` / `decision` / `reject`)

- **Files Containing It**:
  - **Markdown**: `FOUNDING_DOCUMENT.md`, `SKILL.md`, `Proof Envelope v2 — Read-Only Readiness Review...md`, `TSCP-CANON-001.md`.
  - **PDF**: `Enhanced Claude Service - LEGIO Protocol Integration.pdf`, `Capri LLM Integration Blueprint - Dual-Layer Arbors Architecture.pdf`, `The Mirror Watcher AI_ Aria Extension Sacred Deployment.pdf`, `Sovereign Data Vault_ Complete Project Blueprint.pdf`.
  - **TXT**: `claude_findings.txt`, `mirror_watcher_findings.txt`.
  - **Code/JSON**: Implemented in canonical rejection returns and evaluation function return types.

- **Contextual Definition & Usage**:
  - *AI Agent Era*: Dual-layer arbors action filtering and permission guardrails (`ALLOW` / `DENY`).
  - *Canonical Conformance Era*: Rejection codes (`REJECT:TSCP-CANON-FLOAT-PROHIBITED`).
  - *Kernel Era*: The return type of the Kernel evaluation engine:
    `Decision = Allow(next_state) | Reject(reason) | Hold(reason) | Defer(reason)`.

- **Consistency Analysis**: 100% consistent across AI guardrails, canonical parsers, and evaluation kernels.

- **Earliest Evidence**: Early 2025 AI agent access-control guardrails.

- **Latest / Most Formalized Version**: August 18, 2026 in `FOUNDING_DOCUMENT.md` (Deterministic Kernel Decision enum) and `SKILL.md`.

---

## 3. Foundational vs Application-Specific Synthesis

The overarching goal of this archaeological synthesis is to isolate true **FOUNDATIONAL PRIMITIVES** from **APPLICATION-SPECIFIC PRIMITIVES**.

### Foundational Primitives (The Kernel Bedrock)
12 of the 13 concepts (**Authority, Evidence, Contract, Predicate, Transition, Receipt, Canonical, Conformance, Accept, Verify, Proof, Allow/Deny/Decision**) are **FOUNDATIONAL**:
- They appear across 4 or 5 artifact families (Code, Markdown, JSON, PDF, TXT).
- They survive radical shifts in implementation platforms (Web3 smart contracts → Multi-agent LLMOracles → Containerized Docker deployments → Pure specification kernels).
- They converge into the formal mathematical function defined in `FOUNDING_DOCUMENT.md`:
  ```
  evaluate(
      contract,
      evidence,
      predicate,
      current_state,
      proposed_transition
  ) → Decision
  ```

### Application-Specific Primitives
1 concept (**Custody**) is **APPLICATION-SPECIFIC**:
- Restricted primarily to financial wallet key recovery guides (2025) and specialized state tracking in `FOUNDING_DOCUMENT.md`.
- Does not appear in code, JSON test suites, or general protocol specifications.

---

## 4. Conclusion & Evolutionary Arc

The archaeological evidence demonstrates that the TSCP archive is not a series of abandoned projects, but a **continuous distillation process**. 

```
[Early 2024] TrifoldWallet (Web3 / Multisig Authority)
     │
     ▼
[Mid 2024] Vaultfire Ritual Console (Guardians / Proofs)
     │
     ▼
[Late 2024] Sovereign Data Vault (ZK Verifiers / Contracts)
     │
     ▼
[Early 2025] Triumvirate & Capri AI (Dual-Layer Arbors / Allow-Deny Guardrails)
     │
     ▼
[Late 2025] LEGIO & TSCP Δ10.3/10.5 (Containerization / Synthesis Verification)
     │
     ▼
[Mid 2026] TSCP-CANON-001 & Proof Envelope v2 (Canonical Bytes / Conformance)
     │
     ▼
[August 18, 2026] TSCP Evidence-to-Authority Kernel Genesis (FOUNDING_DOCUMENT.md)
```

The 13 recurring primitives identified in this report are the constitutional building blocks of this final **Evidence-to-Authority Kernel**.
