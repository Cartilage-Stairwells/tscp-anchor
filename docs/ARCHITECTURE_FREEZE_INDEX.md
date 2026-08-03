# TSCP / zkSHA-Rx Architecture Freeze Index v1.0

**Date:** 2026-08-03
**Status:** Specification frozen. Changes require amendment process.
**Maintainer:** Sean Christopher Southwick
**GPG:** E747C3AF22573539

---

## Purpose

This document is the canonical pointer layer between the Google Drive specification corpus and the GitHub verification surfaces. It resolves the question: "Where is the authoritative specification?"

**Answer:** The specification lives on Google Drive. The executable verification artifacts live on GitHub. This document links them.

---

## Freeze Declaration

The TSCP custody plane architecture is frozen as of 2026-08-03. No further architectural documents will be produced. Changes to the specification require a formal amendment process.

### Architecture Lifecycle

```
1. Specification (DONE)     — defines semantics and invariants
2. Formalization (NEXT)    — proves mathematical properties
3. Implementation (NEXT)    — realizes the specification
4. Conformance (NEXT)      — checks compliance
5. Benchmarking (DONE)     — measures performance without influencing semantics
```

---

## Document Inventory

### Google Drive — Architecture Specification Package

**Location:** https://drive.google.com/drive/folders/17ogKPlh6qrsMoedW6zH_sWvD295AmMU_
**Access:** Shared (anyone with link can view)

| # | Document | Status | SHA256 |
|---|---|---|---|
| 1 | REVIEWER_SEMANTICS_v1.0.md | FROZEN | 7a2d3d822a6677966291de9f2df4b80e48347a9998c1f08e345b3f4188d448a3 |
| 2 | REVIEWER_PROTOCOL_v0.1.md | DRAFTED | 1cbbc96855e2192133ad4687851749413bffd8e0c67dd20e60d68e73dfa129af |
| 3 | REVIEWER_DATA_MODEL_v1.0.md | FROZEN | f5539fb3ce88a1b9ae237dc05c5dc48069cf943048605a7102088aec024a51df |
| 4 | CANONICALIZATION_MANIFEST_v1.0.md | ACTIVE | 4cc3ec22198d04e9133f0c71531725486ad78daf7b60c1e1aaef74850f891f8a |
| 5 | SPECIFICATION_HARDENING_v0.1.md | DRAFTED | 356ba56cebdfca25fbcf659e2dd645ea1871bd4d9f8fbde23e2bfa9143c72684 |
| 6 | CONFORMANCE_SPECIFICATION_v0.1.md | DRAFTED | 5f3dd6d2697cea95b6acc9063d60419843625151fa2b6d5a8471d2bcec5d0969 |
| 7 | TSCP_NAMED_ROLES_v1.0.md | ACTIVE | 75f736778af5dc9b393ffe7042f1ca79e89937084cdeaa5e522be0ac13c78f20 |
| 8 | PROOF_OBLIGATIONS_v0.1.md | DRAFTED | ea3864ec9318bcb6abd9547ba99cf26205a1c2e0988daf1327ba9d87482accca |
| 9 | FORMAL_DEPENDENCY_INDEX_v1.0.md | ACTIVE | 7bf62f8c7f28492a495b70350a13681e0327e568128cd81e0ece350659e52518 |
| 10 | IMPLEMENTATION_TRACEABILITY_v1.0.md | ACTIVE | e03c1e4ffdc8f316e27133cf49b7f31644fb0c2b3dcb90cc67e641b33869a3bf |
| 11 | BENCHMARK_PROTOCOL.md | ESTABLISHED | 539367739e4d6d5a400c1876b6535bbe1046cb3998d947ab73a47be6d8cf1aa3 |
| 12 | PROOF_ROADMAP_SUMMARY_v1.0.md | ACTIVE | d8a59ecf8902eebc5188477a2a8e4d1f9f63d3a2ab2b8a7f77cb2996e57d65c3 |
| 13 | FINANCIAL_POSSIBILITY_MATRIX_v1.0.md | ACTIVE | c180fd66d70243ccb153143a84f8b11c129f3a9430b880c5f492ab4f53dcf3b6 |
| 14 | PROJECT_CLOSURE_SUMMARY_v1.0.md | ACTIVE | 6daf64f0cdfce32027dc1453936554326fbeda21a896dae1dcfe16434b6be70f |

### Layer Mapping

| Document | Layer | Role |
|---|---|---|
| REVIEWER_SEMANTICS_v1.0.md | Layer 0 (VoxArchon) | Defines meaning, semantic equality, state equivalence |
| REVIEWER_PROTOCOL_v0.1.md | Layer 1 | Evaluation procedures, FCO transitions, failure presentation |
| REVIEWER_DATA_MODEL_v1.0.md | Layer 2 (VexVector) | Canonical representation, serialization, hash derivation |
| CANONICALIZATION_MANIFEST_v1.0.md | Layer 2 | 7-step canonicalization pipeline normative reference |
| SPECIFICATION_HARDENING_v0.1.md | Cross-layer | Six amendments to strengthen precision |
| CONFORMANCE_SPECIFICATION_v0.1.md | Conformance | L1-L4 compliance ladder |
| TSCP_NAMED_ROLES_v1.0.md | Cross-layer | VoxArchon and VexVector named roles |
| PROOF_OBLIGATIONS_v0.1.md | Cross-layer | Theorems T1-T8, assumptions A1-A4 |
| FORMAL_DEPENDENCY_INDEX_v1.0.md | Cross-layer | Theorem dependency graph, proof roadmap |
| IMPLEMENTATION_TRACEABILITY_v1.0.md | Implementation | Artifact to specification to verification mapping |
| BENCHMARK_PROTOCOL.md | Benchmark | Performance evidence requirements |
| PROOF_ROADMAP_SUMMARY_v1.0.md | Cross-layer | Concise proof roadmap for distribution |
| FINANCIAL_POSSIBILITY_MATRIX_v1.0.md | Operations | Grant funding strategy |
| PROJECT_CLOSURE_SUMMARY_v1.0.md | Operations | Architecture phase closure summary |

---

## GitHub — Executable Verification Surfaces

### 1. Reviewer Distribution Artifact

**Repository:** https://github.com/Cartilage-Stairwells/zksha-rx-reviewer-access
**Tag:** review-v0.1.6 (annotated tag)
**Commit:** 7ff9825520d5720e4944efe08965bd61f34067c5
**Clone:** git clone --branch review-v0.1.6 https://github.com/Cartilage-Stairwells/zksha-rx-reviewer-access

**Contents:**
- Implementation (src/) — AVX-512 butterfly kernel, NTT, field arithmetic
- Tests (tests/) — 8 test files, 140 tests, all pass
- Benchmark (benches/) — Three-lane benchmark with correctness gate
- Evidence (evidence/) — Benchmark artifacts, CPU info, receipts
- Formal (formal/) — Lean formalization artifacts
- Reviewer docs — README, quickstart, release notes, architecture summary

**Verification:**
- SHA256SUMS: 67/67 verified
- Validator: 7/7 pass, 1 warning (unsigned tag)
- Tag chain: v0.1.0 through v0.1.6

### 2. Protocol Anchor

**Repository:** https://github.com/Cartilage-Stairwells/tscp-anchor
**Commit:** 86d2288677d08b16120d43cefafe433637679a46 (at time of freeze index creation)
**Branch:** master

**Contents:**
- VERIFICATION_INVARIANTS.md
- IMPLEMENTATION_TARGET_BINDING.md
- EXECUTION_TRACE_RECEIPT.md
- VERIFICATION_SURFACE_DRIFT_TEST_PLAN.md
- Lean formalization (BabyBear/Boundary.lean, TraceCoreProver.lean)
- Custody framework (contracts, anchor receipts, audit manifests)
- Evidence artifacts (evidence/, supply-chain-evidence/)

---

## Theorem Dependency Graph

```
Layer 0 Types
    |
    +--> T1: semantic_equal equivalence
    |       +--> T2: canonicalization totality [A4]
    |       |       +--> T2a: malformed input rejection [A4]
    |       +--> T3: derived predicate soundness
    |       |       +--> T6: non-interference
    |       +--> T4: semantic preservation [A2, A3] <- CRITICAL PATH
    |               +--> T5: digest consistency [A1]
    |
    +--> T8 family: plane separation <- PROVEN (Lean, by decide)
```

Critical path: types -> T1 -> T4 -> T5 -> conformance vectors

---

## External Assumptions

| Assumption | Statement |
|---|---|
| A1 | SHA-256 satisfies published security properties |
| A2 | Canonicalization implementation conforms to RFC 8785 |
| A3 | Normalization implementation conforms to UAX #15 NFC |
| A4 | Inputs are well-formed semantic projections |

---

## Already Proven

| Theorem | Lean Proof | Method |
|---|---|---|
| T8 (authority unreachability) | custody_receipt_no_authority_path | decide |
| T8a (no plane crossing) | no_plane_crossing | decide |
| T8b (evidence-not-authority) | harness_is_evidence_generator | decide |
| T8c (receipt type consistency) | receipt_in_custody_plane | decide |

All proven by exhaustive enumeration. 0 axioms beyond Lean trusted kernel. 0 sorry.

---

## Benchmark Evidence

**Hardware:** AMD Zen 5 with AVX-512 ISA support
**Caveat:** Dual-256-bit execution backend — results should not be generalized to all AVX-512 microarchitectures

| Comparison | Geometric Mean Speedup |
|---|---|
| AVX2 vs Scalar | 2.37x |
| AVX-512 vs Scalar | 3.08x |

Correctness gate: PASS (all lanes agree on all sizes 2^8 through 2^20)

---

## Amendment Process

This freeze index is a pointer layer, not a specification document. To amend the architecture:

1. Propose the amendment as a new versioned document (v0.2, v1.1, etc.)
2. Update this freeze index with the new document hash and status
3. Commit the update to tscp-anchor with a descriptive message
4. Sync the new document to the Drive folder

The architecture specification itself is frozen. This index tracks the freeze state.

---

## Contact

**Maintainer:** Sean Christopher Southwick
**Outreach email:** schlagetorren@gmail.com
**Canonical email:** adamantinespine@gmail.com
**GPG:** E747C3AF22573539

---

*This document was created on 2026-08-03 to close the provenance gap between the Drive specification corpus and the GitHub verification surfaces. It is the canonical entry point for the TSCP architecture freeze.*
