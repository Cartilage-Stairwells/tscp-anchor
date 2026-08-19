# TSCP Kernel Archaeology — Admissibility Experiment v0.1

**Date:** August 18, 2026
**Branch:** kernel-archaeology/admissibility-experiment-v0.1
**Status:** Work product from Lyra (Base44 Superagent) + Johnny + Aria (ChatGPT) collaboration

## Contents

### Specification
- `ADMISSIBILITY_CONTRACT_SPEC.md` — Admissibility Contract Specification v0.2
- `TSCP-CANON-001.md` — Canonical serialization spec
- `TSCP-CANON-001-PIN.md` — Canonical spec pin

### Archaeology
- `FOUNDING_DOCUMENT.md` — Aria's founding document
- `ARCHAEOLOGY_REPORT.md` — Full archaeology report
- `ARCHAEOLOGY_INVENTORY.md` — File inventory
- `ARCHAEOLOGY_RECURRING_PRIMITIVES.md` — Recurring primitive analysis
- `ARCHAEOLOGY_CODE_ANALYSIS.md` — Code analysis
- `ARCHAEOLOGY_DATA_ANALYSIS.md` — Data file analysis
- `ARCHAEOLOGY_MD_ANALYSIS.md` — Markdown analysis
- `ARCHAEOLOGY_PDF_ANALYSIS.md` — PDF analysis
- `RECOVERY_ELIMINATION_REPORT.md` — Recovery elimination

### Kernel Deliverables
- `KERNEL_CHARTER.md` — Kernel charter
- `KERNEL_INVARIANT_REGISTRY.md` — Invariant registry (12 invariants, 7 enforced)
- `KERNEL_EVIDENCE_MATRIX.md` — Evidence matrix
- `IMPLEMENTATION_READINESS.md` — Architecture recommendation
- `INDEPENDENT_REVIEW.md` — Self-adversarial review

### Aria Reviews
- `ARIA_REVIEW_2.md` — Aria's red-team review
- `ARIA_ROUND3_PACKAGE.md` — Round 3 package for Aria

### Rust Implementation
- `kernel/` — Rust implementation (pure protocol logic, zero dependencies)
  - `src/lib.rs` — admit() implementation
  - `src/tests.rs` — 27-test suite (6 property categories)
  - `Cargo.toml` — Package manifest

### Threat Model
- `RUST_THREAT_MODEL.md` — Frozen Rust threat model

### Validation
- `VALIDATION_RESULTS.txt` — Raw test output (27/27 pass)

## Test Results

27/27 tests pass across 6 property categories:
1. Positive admission (valid → ACCEPT)
2. Validation rejection (malformed → REJECT)
3. Binding rejection (wrong type/role → REJECT)
4. Completeness rejection (insufficient/excess → REJECT)
5. Semantic firewall (fabricated → admissible, NOT true)
6. Construction boundary (only admit() can produce AdmittedEvidence)

## Classification

- 8 PASS — properties that hold regardless of excluded mechanisms
- 5 HOLD (current: PASS) — hold in current build, could be violated if excluded mechanisms added
- 2 PASS for declared universe — tested within explicitly defined threat model
- 0 FAIL
- Specification ↔ implementation correspondence: HOLD (central unresolved question)

## Note

GPG signing not available in this sandbox. Commit is unsigned. Verify content against conversation logs.
