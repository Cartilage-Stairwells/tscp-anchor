# TSCP ZK Surface Exploration v1.0

**Document:** TSCP_ZK_SURFACE_EXPLORATION_v1.0  
**Date:** 2026-10-07  
**Status:** DECLARED exploration package (observation + definition + mapping)  
**Relation:** Companion to TSCP_NAMED_ROLES_v1.1  
**Scope:** Human documentation of zero-knowledge concepts and their observed relationship to the TSCP / Base44 Lean–BabyBear surface. Does not alter machine artifacts, does not promote projected performance, and does not complete formal proofs.

---

## 1. Purpose and Epistemic Contract

This document does three things:

1. Defines zero-knowledge proofs and ZK-SNARKs in standard cryptographic terms.
2. Maps those definitions onto the **observed** TSCP-related formal and kernel surface (Drive folder “Base 44 Lean ZK Proofs”, public GitHub under Cartilage-Stairwells).
3. Records candidate next steps that remain inside claim_scope ⊆ evidence_scope.

**Non-negotiable rules applied throughout**

| Rule | Application here |
|------|------------------|
| Observation ≠ Interpretation | Measured tests, Lean compile status, and public repo presence are OBSERVED. Framing narratives are INTERPRETATION. |
| claim_scope ⊆ evidence_scope | No claim of “formally proven end-to-end prover” or “fastest kernel” is made. |
| Evidence ≠ authority | A passing test or a Lean theorem does not admit a custody transition. |
| Verification ≠ promotion | Computational evidence and machine-checked lemmas remain distinct from production deployment or funding claims. |
| Identity ≠ custody | Named roles (VoxArchon … VexScale) are documentation handles only. |

---

## 2. Zero-Knowledge Proofs — Core Definition

A **zero-knowledge proof** (ZKP) is an interactive or non-interactive protocol in which a prover convinces a verifier that a statement is true without revealing information beyond the validity of the statement.

Standard completeness, soundness, and zero-knowledge properties apply:

- **Completeness:** If the statement is true and both parties follow the protocol, the verifier accepts.
- **Soundness:** If the statement is false, a cheating prover cannot convince the verifier except with negligible probability.
- **Zero-knowledge:** The verifier learns nothing about the witness beyond the fact that the statement holds.

ZKPs are used to prove knowledge of a secret, correctness of a computation, or membership in a language while withholding the underlying data.

---

## 3. ZK-SNARKs — Definition and Distinctions

**ZK-SNARK** = Zero-Knowledge Succinct Non-interactive ARgument of Knowledge.

| Property | Meaning |
|----------|---------|
| Zero-knowledge | As above |
| Succinct | Proof size and verification time are small relative to the size of the computation (often polylogarithmic or constant-size proofs) |
| Non-interactive | After a possible trusted setup (or transparent setup in later variants), a single message from prover to verifier suffices |
| Argument of knowledge | Computational soundness; relies on computational assumptions rather than information-theoretic soundness alone |

Common construction families (illustrative, not exhaustive):

- Pairing-based SNARKs (e.g., Groth16) — often require a circuit-specific or universal trusted setup.
- Transparent or updatable systems and related succinct arguments.
- Polynomial-commitment based systems that sit near the SNARK / STARK boundary.

**SNARK vs STARK (high-level contrast)**

| Aspect | Typical SNARK | Typical STARK |
|--------|---------------|---------------|
| Setup | Often trusted or universal | Transparent (public randomness) |
| Proof size | Very small (hundreds of bytes possible) | Larger (kilobytes) |
| Field / arithmetic | Often elliptic-curve friendly fields | Often large prime fields; FRI / hash-based |
| Quantum resistance framing | Pairing-based constructions are not generally considered post-quantum | Hash-based STARKs are often discussed as post-quantum candidates |
| Dominant cost drivers | FFT / MSM / pairings depending on system | NTT / FRI / hashing |

Plonky2 / Plonky3-style systems combine recursive techniques, FRI or related polynomial IOPs, and efficient fields (including BabyBear). They are frequently discussed in the STARK / SNARK-adjacent literature rather than as classical pairing SNARKs.

---

## 4. Observed TSCP / Base44 Surface (OBSERVED)

The following are recorded as present in the inspected Drive folder and public GitHub account Cartilage-Stairwells. Presence does not equal completed formal verification of an entire proving system.

### 4.1 Public repositories (OBSERVED)

| Repository | Description (from GitHub metadata) |
|------------|-------------------------------------|
| Cartilage-Stairwells/tscp-anchor | Rust-based deterministic cryptographic proof system with WASM smoke tests, CI verification, and reproducible build pipeline |
| Cartilage-Stairwells/zksha-rx-reviewer-access | Frozen reviewer snapshot for zkSHA-Rx Fly v0.1.0 |
| Cartilage-Stairwells/canonical-lexicon | Terminology / nomenclature authority |
| Cartilage-Stairwells/tscp.store | Public site |

### 4.2 Drive formal / kernel artifacts (OBSERVED titles)

- `BabyBearVerified.lean` — Lean formalization of BabyBear Montgomery arithmetic
- `FORMAL_PROOF_SURFACE.md` — mapping of Rust functions to Lean specifications; multiple proof obligations historically marked with `sorry` in the 2026-07-05 surface document
- `EVIDENCE.md`, `VERIFIED_CLAIMS.md`, `STATUS_CORRECTED.md`, `PLONKY3_COMPARISON.md`, `REPRODUCTION_PROTOCOL.md`, criterion benchmarks, release manifests
- TSCP Experiment A kernel evidence documents
- TSCP-PL BabyBear Lean4 Formal Verification Summary (2026-07-07) — reports machine-checked lemmas including `redc_cancellation`, modular add/sub correctness, butterfly range properties; some congruence bookkeeping deferred

### 4.3 Status discipline already present (OBSERVED)

Documents in the surface explicitly separate:

- Computational evidence (tests, differential checks, criterion benchmarks)
- Formal proof status (Lean theorems vs `sorry` / deferred)
- Verified vs projected performance claims
- Hardware dependence (AVX-512 availability varies by sandbox instance)

Speedup figures have been revised across documents (e.g., earlier 9.x× figures vs later criterion or corrected measurements). Any performance statement must cite the specific evidence document and hardware context; this exploration does not re-assert a single canonical speedup.

### 4.4 What is NOT established by the inspected surface alone

- End-to-end production Plonky3 (or other) prover integration as a completed, audited product
- Universal claim “formally proven correct” for the entire kernel stack without remaining obligations
- Superiority claims against all alternative ZK kernels
- That the named roles (VoxArchon … VexScale) are implemented as runtime types or appear in hash preimages

---

## 5. Mapping Named Roles to the ZK Surface (DECLARED mapping, not implementation)

The named roles from TSCP_NAMED_ROLES_v1.1 are documentation handles. The following mapping is a **declared orientation aid** for discussion; it does not assert that the Lean or Rust artifacts implement those names.

| Named role | Natural affinity on the ZK surface | Notes |
|------------|------------------------------------|-------|
| **VoxArchon (Layer 0)** | Semantic / statement definition: what is being proved, relationship constraints, admissible vocabularies | Closest to “what statement is true?” |
| **VexProbe (Layer 1)** | Test, differential, property, and sandbox evaluation of field / NTT / butterfly behavior before freeze | Observational feedback only |
| **VexVector (Layer 2)** | Deterministic encoding / canonical representation of field elements, NTT layouts, and proof objects | Freeze of representation, not of meaning |
| **VoxCanon (Conformance)** | Alignment of emitted proof objects or kernel outputs against declared rules and relationship types | Compliance, not performance |
| **VexScale (Benchmark Plane)** | Criterion / throughput / latency measurement against external hardware and reference implementations (e.g., Plonky3 comparison) | ESTABLISHED relative to measured environment |

**Interaction reminder:** VoxArchon emits meaning (statement / policy). VexProbe assays. VexVector freezes encoding. VoxCanon checks alignment. VexScale weighs outcome. Feedback remains observational; semantic mutation on the return path is prohibited by the naming charter.

---

## 6. Where ZK-SNARKs Fit Relative to This Surface

The inspected surface is primarily a **BabyBear arithmetic and NTT / butterfly kernel** effort with Lean formalization and differential testing, positioned next to Plonky3-style proving stacks.

Implications:

1. **Substrate, not the full SNARK.** Field arithmetic and NTT are necessary components of many SNARK / STARK provers; they are not themselves a complete SNARK.
2. **Formal lemmas strengthen the substrate.** Machine-checked REDC cancellation, range, and modular arithmetic properties reduce the trust placed in the arithmetic core. They do not automatically prove soundness of a higher-level argument system.
3. **Comparison documents already enforce honesty.** PLONKY3_COMPARISON.md separates measured scalar results from inferred AVX-512 analysis and declines a “superior alternative kernel” narrative in favor of formal-constraint differentiation.
4. **Admission layer vs proving layer.** If TSCP is treated as an evidence-admission boundary for autonomous workflows, ZK proofs (including SNARK-style succinct arguments) are one class of evidence that may be presented for admission. The admission decision remains distinct from the existence of a valid proof.

---

## 7. Candidate Next Steps (DECLARED proposals only)

These steps do not execute promotions or complete proofs.

### 7.1 Architecture / naming track

1. Draft VoxCanon severe-exception protocol (error handling when alignment fails), labeled DRAFTED, with explicit non-mutation of upstream semantics.
2. Draft VexScale observational telemetry token format (hash, metric identifiers, environment binding), labeled DRAFTED; no performance claim implied by the format alone.
3. Record any adopted FROZEN mnemonic properties (“operator zeroed”, “exhaustive numbers”) only when independent evidence exists for the specific object class.

### 7.2 Formal / ZK substrate track

1. Reconcile Lean surface documents: list every theorem that currently compiles with zero `sorry` versus remaining obligations (e.g., full Montgomery representation congruence if still deferred).
2. Bind each performance claim used externally to a single evidence document + hardware fingerprint + reproduction command.
3. Maintain the verified / projected / not-claimed tables; refuse elevation of projected end-to-end prover speedups without measurement.
4. If GitHub push of Lean or kernel packages is desired, treat repository topology, tags, and CI receipts as separate evidence objects under the evidence-trail discipline.

### 7.3 Cross-layer discipline

1. Never insert named-role strings into canonical encodings or hash preimages.
2. Never treat a successful ZK proof verification as automatic custody transfer or promotion.
3. Keep SNARK / STARK terminology accurate: do not label the BabyBear kernel itself a “ZK-SNARK.”

---

## 8. Evidence Boundary for This Artifact

| Item | Label |
|------|-------|
| Existence of this exploration document | OBSERVED (this file) |
| Public Cartilage-Stairwells repositories listed above | OBSERVED |
| Drive formal and evidence documents summarized above | OBSERVED (titles and quoted status language) |
| Definitions of ZKP and ZK-SNARK | Standard cryptographic definitions (DECLARED explanatory) |
| Mapping of named roles to ZK surface | DECLARED orientation aid |
| Any claim that formal proofs are fully complete for the entire stack | NOT established by this document |
| Any single canonical speedup figure | Not asserted; see surface documents for measured ranges and corrections |
| Drive upload of this file | OBSERVED only after successful upload receipt |

---

## 9. Required Evidence to Elevate

To elevate any of the following, separate packages are required:

- “Machine-checked correctness of the full kernel” → complete Lean obligation list with zero remaining critical `sorry` items and independent verification
- “SNARK-ready production substrate” → integration evidence, security review, and explicit claim_scope table
- “Named roles implemented in code” → explicit prohibition remains; elevation would require governance change and machine-artifact audit
- Performance superiority claims → criterion (or equivalent) results on named hardware with reproduction protocol

---

*End of TSCP_ZK_SURFACE_EXPLORATION_v1.0*
