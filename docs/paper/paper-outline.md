---
title: TSCP Technical Paper Outline
summary: Draft outline for a paper targeting IACR ePrint or an AI safety venue
---

# TSCP Technical Paper — Draft Outline

## Title (options)
- "TSCP: Cryptographic Custody Verification for Multi-Agent AI Systems"
- "From Assertion to Verification: A Categorical Framework for Artifact Provenance in ZK Proof Systems"
- "The Triune Structured Codex Protocol: Formal Verification of Computation Custody"

## Abstract (draft)

We present TSCP, a protocol for cryptographic custody verification of artifacts produced by multi-agent computation pipelines. TSCP introduces a 5-stage custody pipeline — Resolve, Observe, Bind, Independent Verify, Promote — that transforms declared hashes from assertions into independently verified claims. The protocol is built on a categorical formal backbone in Lean 4, proving that verification properties (admissibility, preservation, reflection) compose correctly across systems. We demonstrate the protocol's implementation on the Plonky3 proof system, including an AVX-512 NTT optimization achieving 9.15x speedup with 61 verification points, backed by 83 formal theorems and 102 test vectors. We identify and address 12 boundary-level security findings through adversarial discovery (ARCHER methodology), establishing that the protocol's strictness concentrates in the happy path while gaps appear at configuration defaults, input validation, and interface boundaries.

## Sections

### 1. Introduction
- The problem: AI agents produce artifacts. How do downstream systems verify provenance?
- Current approaches: trust declared hashes (unsafe), full recomputation (expensive), or no verification (common)
- Our contribution: a structured custody pipeline with formal guarantees

### 2. Background
- ZK proof systems (Plonky3, FRI, sumcheck)
- Multilinear extensions and oracle-based proofs
- Formal verification in Lean 4
- NTT and its role in FRI-based proof systems

### 3. The TSCP Protocol
#### 3.1 Custody Pipeline
- 5 stages: Resolve → Observe → Bind → Independent Verify → Promote
- What each stage does and why it's necessary
- The "declared hash is an assertion" principle

#### 3.2 Categorical Formal Backbone
- Kernel/Universe/Bridge structure
- Admissibility preservation under composition (theorem)
- The 2 axioms as engineering boundaries
- What's formally proven vs. what's empirically verified

#### 3.3 ZK Implementation
- Sumcheck protocol with self-checking prover
- FRI commitment scheme (commit, query, verify)
- Merkle commitments via Plonky3's MMCS
- OWSL entropy health gating

#### 3.4 NTT Optimization
- AVX-512 butterfly kernel
- 9.15x speedup over baseline
- Formal verification of NTT correctness (83 Lean4 theorems)
- 102 Rust test vectors

### 4. Security Analysis
#### 4.1 ARCHER Methodology
- Adversarial discovery framework
- Generate → Perturb → Execute → Observe → Compare → Hypothesize → Falsify → Reproduce → Report

#### 4.2 Findings
- 12 findings across P0 baseline and ZK layer
- Pattern: strictness concentrates in happy path, gaps at boundaries
- All findings addressed with fixes

#### 4.3 Remaining Boundaries
- 2 formal axioms (engineering boundaries)
- No external audit yet
- Testnet-only deployment

### 5. Related Work
- ZK proof systems (Plonky3, RISC Zero, StarkWare)
- Software supply chain security (SBOM, SLSA)
- AI output verification (watermarking, detection)
- Formal verification of cryptographic protocols

### 6. Discussion
- The custody verification concept as a new primitive
- Generalizability beyond Plonky3
- Limits of self-verification (the "who verifies the verifier" problem)

### 7. Conclusion and Future Work
- Full formalization of the 2 remaining axioms
- External audit
- Production FRI verifier (on-chain)
- Multi-proof-system support

## Target Venues (in order of fit)
1. **IACR ePrint** — preprint, fast turnaround, cryptology community
2. **NeurIPS AI Safety Workshop** — AI verification angle
3. **ICML Accountability workshop** — provenance angle
4. **IEEE S&B (Security & Blockchain)** — ZK + on-chain angle
5. **USENIX Security** — broader security angle (longer review cycle)
