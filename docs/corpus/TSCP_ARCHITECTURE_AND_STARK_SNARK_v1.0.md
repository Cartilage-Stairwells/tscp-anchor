# TSCP Architecture and SNARK–STARK Comparison v1.0

**Document:** TSCP_ARCHITECTURE_AND_STARK_SNARK_v1.0  
**Date:** 2026-10-07  
**Status:** DECLARED exploration package  
**Sources (OBSERVED):**  
- `Cartilage-Stairwells/tscp-anchor` (README, PROJECT_FACTS.md, repository tree)  
- `Cartilage-Stairwells/canonical-lexicon` (NOMENCLATURE.md, README)  
- Prior session artifacts: TSCP_NAMED_ROLES_v1.1, TSCP_ZK_SURFACE_EXPLORATION_v1.0  
- Drive formal surface (BabyBear / Lean / evidence documents)

**Epistemic rule:** claim_scope ⊆ evidence_scope. Public repository text is OBSERVED. Interpretive synthesis is labeled. Performance and formal-completeness claims follow PROJECT_FACTS.md frozen claims table.

---

## Part I — TSCP Architecture (Detail)

### 1. What TSCP Is (canonical definition)

From the canonical lexicon (Active / Frozen family):

| Field | Value |
|-------|--------|
| **Literal** | Triune Structured Codex Protocol |
| **Definition** | Protocol for cryptographic custody verification of artifacts produced by multi-agent computation pipelines; governs state transitions, governance events, and evidence/transition semantics |
| **Practical meaning** | Principal framework through which system transitions and their evidence are constrained and verified: **a declared hash is an assertion, not a verification** |
| **Domain** | Protocol / Verification |
| **Authority pointer** | tscp-anchor governing artifacts; lexicon records the name and points there |

Related constitutional objects:

| Term | Role | Critical property |
|------|------|-------------------|
| **FCO** (Field Coherence Object) | Evidence boundary | `Authority(FCO) = 0` — packages evidence; does not authorize |
| **AdmittedEvidence** | Evidence that passed the admissibility predicate | Admissible ≠ true; does not grant authority, execution, or promotion |
| **TSCP-AUDIT-CANON** | Identity-first admission plane | Cannot approve, authorize, execute, or promote |
| **Custody Plane** | Constrains evidence/state custody through transitions | Does not inherently grant authority to evidence objects |
| **Evolution Plane** | System change / transformation | Separated from custody so transformation does not silently become authority |

### 2. TSCP Anchor — Proving and Verification Layer

From `tscp-anchor` README (OBSERVED):

**Role:** Public verification and documentation layer for the zkSHA-Rx Fly validation effort. Cryptographic proving layer that makes protocol claims independently verifiable (*sovereign verifiability*).

**Three provided capabilities:**

1. **ZK proving stack** — FRI-based transparent commitment, sumcheck, DEEP-ALI constraint verification, Poseidon2 Fiat-Shamir transcript; Rust on Plonky3 0.6.1  
2. **On-chain anchor** — `TSCPAnchor.sol` immutable registry of keccak256 artifact hashes (Sepolia deployment recorded; development only)  
3. **Migration protocol** — versioned `ProofEnvelope`, upgrade driver, golden corpus, mixed-version rejection gates

### 3. Stack Architecture (from README)

```
prover-server
  HTTP /prove/sumcheck
  Sumcheck prover + verifier
  ProofEnvelope versioning
        │
oracle-layer
  FRI commit / query / verify
  DEEP-ALI constraint checking
  Multilinear oracle (MleOracle)
  Poseidon2 Fiat-Shamir transcript
        │
batch-merkle
  Plonky3 BatchMerkle primitives
  Tamper-evident commitment layer
        │
commitment
  TSCP polynomial commitment scheme
  DFT + MMCS + FRI parameters
```

**Supporting systems (OBSERVED):**

- **OWSL** (Oracle Witness Status Layer) — operational health gating; atomic status file; Rust bridge  
- **Lean 4** formal modules under `TSCP/Formal/` and `BabyBear/` — protocol semantics, Montgomery, NTT stage, butterfly, evidence binding, reviewer semantics  
- **CI workflows** — acceptance, anchor, release, stage2-verify, supply-chain governance, tscp-attestation, wasm-smoke, verify-receipts  

### 4. Crate Structure

| Crate | Purpose |
|-------|---------|
| `oracle-layer` | FRI, DEEP-ALI, MLE oracle, Fiat-Shamir |
| `batch-merkle` | Plonky3 BatchMerkle wrapper |
| `commitment` | TSCP PCS (polynomial commitment) |
| `prover-server` | HTTP proving service, sumcheck, ProofEnvelope |

### 5. Soundness Properties Covered by Adversarial Tests (README)

FRI (tampered fold, wrong beta, forged opening), sumcheck (wrong sum, tampered round, honest proof verifies), DEEP-ALI end-to-end, BatchMerkle tampered leaf, version gate (0.6.2 envelope rejected by 0.6.1 verifier).

**Explicit limitation (README):** Implementation-level soundness via test suite. Protocol-level soundness not formally proven. External cryptographic audit not performed. Suitable for research evaluation and technical review; not for production without independent audit and operational hardening.

### 6. Frozen Claims Boundary (PROJECT_FACTS.md)

| Claim | Status |
|-------|--------|
| BabyBear field implementation exists | Verified |
| AVX-512 backend exists | Verified (existence); SIMD formal correctness not claimed from this repo alone |
| Montgomery arithmetic formalized | Verified (Lean) |
| Entire NTT formally verified | **Not claimed** |
| Kernel benchmark measurements exist | Historical / registered baselines |
| End-to-end zk prover acceleration | **Not claimed** |
| External cryptographic audit | **Not claimed** |
| Prover integration of AVX-512 kernel | **Not claimed** |

Registered evidence baselines (different scopes, hardware, methods) are complementary, not competitive. Kernel speedup does not linearly compose into end-to-end prover speedup.

### 7. Relation to Named Roles (DECLARED orientation)

From TSCP_NAMED_ROLES_v1.1 — human documentation handles only:

```
VoxArchon (L0)  — semantic / statement definition
VexProbe  (L1)  — evaluation / interrogation (observational)
VexVector (L2)  — deterministic encoding / freeze of representation
VoxCanon        — conformance / alignment
VexScale        — benchmark measurement plane
```

These names **do not** appear as runtime types in the observed `tscp-anchor` crate structure. Mapping remains documentation orientation: meaning emission → assay → freeze → alignment → measurement, with unidirectional semantic flow and observational feedback only.

### 8. Architectural Thesis (INTERPRETATION, bounded)

TSCP separates **evidence** from **authority**. Hashes assert; verification establishes. FCOs carry evidence with zero authority. Admission does not imply truth or promotion. The proving stack (FRI / sumcheck / DEEP-ALI on Plonky3) supplies cryptographic evidence objects that may be presented to admission contracts; the admission decision remains a distinct plane.

---

## Part II — ZK STARKs

### 1. Definition

**STARK** = Scalable Transparent ARgument of Knowledge.

A STARK is a non-interactive argument system (typically after Fiat-Shamir) that proves integrity of a computation with:

- **Scalability:** Prover time roughly quasi-linear in the computation; verifier time polylogarithmic (or better) in the computation size  
- **Transparency:** No trusted setup ceremony; public randomness (or hash-derived challenges) only  
- **Argument of knowledge:** Computational soundness under stated assumptions  

STARKs are commonly built from:

1. Algebraic intermediate representation (AIR) or related constraint systems over a finite field  
2. Polynomial commitment via Reed-Solomon codes and **FRI** (Fast Reed-Solomon Interactive Oracle Proof of Proximity)  
3. Fiat-Shamir transformation for non-interactivity (e.g., Poseidon / Poseidon2 transcripts in modern stacks)

### 2. FRI (central STARK primitive)

FRI reduces a claim about a polynomial’s degree (or proximity to low degree) through iterative folding and Merkle-style commitments. Verifier queries random positions; soundness relies on coding-theoretic bounds and collision-resistant hashing.

The observed `tscp-anchor` oracle-layer implements FRI commit / query / verify with adversarial tests for tampered folds, wrong folding challenges, and forged openings — consistent with a STARK-style transparent PCS path rather than a pairing-based SNARK path.

### 3. Sumcheck and DEEP-ALI (observed stack)

- **Sumcheck:** Interactive (or Fiat-Shamir) protocol reducing a multivariate sum claim to a univariate evaluation claim; used heavily in modern polynomial IOPs  
- **DEEP-ALI:** Domain Extension for Eliminating Pretenders via Algebraic Linking Identity — constraint-checking technique used in the observed oracle-layer to bind the proved trace to the claimed computation  

Together with BatchMerkle and a Poseidon2 transcript, the stack aligns with **transparent, FRI-based, STARK-family** design choices (via Plonky3), not with classical Groth16-style pairing SNARKs.

### 4. Field choice (BabyBear)

BabyBear (\(p = 2^{31} - 2^{27} + 1 = 2013265921\)) is a STARK-friendly small prime field used in Plonky3. Montgomery arithmetic, NTT/butterfly kernels, and Lean formalization of field properties appear in the observed surface as **substrate** for such provers, not as a complete STARK product claim by themselves.

---

## Part III — SNARK versus STARK Trade-offs

### 1. Side-by-side comparison

| Dimension | Typical SNARK | Typical STARK |
|-----------|---------------|---------------|
| **Setup** | Often trusted (circuit-specific or universal); some newer systems reduce trust | Transparent — public randomness / hashes only |
| **Proof size** | Very small (e.g., ~hundreds of bytes for Groth16-class) | Larger (often tens to hundreds of KB) |
| **Verification cost** | Extremely cheap (few pairings or small arithmetic) | Higher than smallest SNARKs; still polylog in computation size |
| **Prover cost** | Can be high; MSM / FFT heavy depending on system | Quasi-linear; FRI + NTT heavy |
| **Cryptographic assumptions** | Pairings, knowledge-of-exponent, etc. | Collision-resistant hashes, coding theory; fewer algebraic group assumptions |
| **Post-quantum posture** | Pairing-based SNARKs are not generally post-quantum | Hash-based STARKs are widely discussed as post-quantum candidates |
| **Recursion / composition** | Mature pairing and IVC ecosystems | Strong recursion story via FRI and modern IOP stacks (Plonky2/3, etc.) |
| **Implementation complexity** | Circuit / R1CS or Plonk-style arithmetization | AIR / polynomial constraints + FRI parameter tuning |
| **Transparency vs succinctness** | Favors extreme succinctness | Favors transparency and quantum-resistant framing |

### 2. When SNARKs are often preferred

- On-chain verification where every byte of calldata and every pairing matters  
- Applications that already accept a trusted or universal setup  
- Ecosystems with mature tooling for a specific SNARK (e.g., Groth16, Plonk variants)

### 3. When STARKs are often preferred

- Desire to avoid trusted setup  
- Post-quantum risk posture  
- Very large computations where transparent scalability matters more than minimal proof size  
- Stacks already invested in FRI / AIR / hash-based transcripts (including Plonky3-class systems)

### 4. Hybrid and adjacent systems

Many production systems sit between pure classical SNARK and pure STARK:

- Transparent SNARK-like arguments  
- SNARKs with updatable or universal setup  
- STARK-like provers with aggressive proof compression or recursive wrapping into a SNARK for on-chain verification  

**Plonky3** (the base of the observed TSCP Anchor stack) is best described as a modern polynomial-IOP / FRI-oriented toolkit used for STARK-style and recursive proving, not as a classical pairing SNARK library.

### 5. Trade-off summary for TSCP-shaped systems

For an **evidence-admission** architecture (TSCP):

| Need | SNARK lean | STARK lean |
|------|------------|------------|
| Minimal on-chain verifier cost | Stronger fit | Weaker unless recursively wrapped |
| No trusted setup for evidence objects | Weaker (unless transparent SNARK) | Stronger fit |
| Formal friendliness of field arithmetic | Both possible | BabyBear / small fields well studied in STARK stacks |
| External audit surface | Setup ceremony + circuit | FRI parameters + hash + AIR |
| Alignment with observed tscp-anchor | Partial | **Direct** (FRI, sumcheck, DEEP-ALI, Poseidon2, Plonky3) |

The observed public stack has chosen the **transparent FRI / STARK-family** path for its proving layer. That choice does not prohibit SNARK wrappers for specific deployment targets; it does mean the native evidence objects described in README are STARK-style arguments, not Groth16 proofs.

---

## Part IV — Integrated Picture

```
                    ┌─────────────────────────────┐
                    │  VoxArchon-class semantics  │  (what is claimed)
                    │  statements, policies, DAG  │
                    └──────────────┬──────────────┘
                                   │ meaning (unidirectional)
                    ┌──────────────▼──────────────┐
                    │  VexProbe-class evaluation  │  (tests, differential checks)
                    └──────────────┬──────────────┘
                                   │ assayed object
                    ┌──────────────▼──────────────┐
                    │ VexVector-class freeze      │  (canonical encoding, hashes)
                    └──────────────┬──────────────┘
                                   │
          ┌────────────────────────┼────────────────────────┐
          │                        │                        │
          ▼                        ▼                        ▼
   STARK-style proof         On-chain hash anchor      Lean formal bounds
   (FRI, sumcheck,           (TSCPAnchor.sol           (field, custody,
    DEEP-ALI, Poseidon2)      Sepolia / registry)       invariants)
          │                        │                        │
          └────────────────────────┼────────────────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │ VoxCanon-class alignment    │  (does evidence match rules?)
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │ VexScale-class measurement  │  (benchmarks, baselines)
                    └─────────────────────────────┘

Admission / custody plane: FCO carries evidence with Authority = 0.
AdmittedEvidence ≠ truth ≠ promotion.
```

**Critical boundary (from lexicon and PROJECT_FACTS):**  
A valid STARK (or SNARK) proof is powerful evidence. Under TSCP discipline it still does not become authority, custody transfer, or automatic promotion.

---

## Part V — Evidence Boundary for This Document

| Item | Label |
|------|-------|
| tscp-anchor README architecture, crates, soundness tests, limitations | OBSERVED |
| PROJECT_FACTS frozen claims table | OBSERVED |
| Canonical lexicon TSCP / FCO / AdmittedEvidence definitions | OBSERVED |
| Named-role mapping | DECLARED orientation (documentation only) |
| SNARK vs STARK comparison table | Standard cryptographic comparison (explanatory) |
| Claim that tscp-anchor is a complete production STARK product | **Not made** |
| Claim of end-to-end prover acceleration | **Not made** (PROJECT_FACTS) |
| External audit completion | **Not claimed** |

---

## Part VI — Suggested Next Steps (proposals only)

1. **Architecture track:** Draft VoxCanon exception protocol and VexScale telemetry token schema as DRAFTED specifications.  
2. **Proving track:** Maintain FRI / sumcheck adversarial coverage; treat Plonky3 version migration as a gated, golden-corpus-bound process (already described in README).  
3. **Formal track:** Keep Lean obligation inventory explicit (what is machine-checked vs not claimed for full NTT / protocol-level soundness).  
4. **Comparison track:** If SNARK wrappers are considered for on-chain cost, document them as a **separate** deployment path that does not redefine the native transparent evidence object.  
5. **Lexicon track:** Ensure VoxArchon / VexProbe / VexVector / VoxCanon / VexScale and the FROZEN mnemonic remain documentation handles unless governance explicitly elevates them.

---

*End of TSCP_ARCHITECTURE_AND_STARK_SNARK_v1.0*
