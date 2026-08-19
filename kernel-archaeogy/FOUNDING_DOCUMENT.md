# TSCP Evidence-to-Authority Kernel — Foundational Document

**Source:** Aria (ChatGPT), transmitted via Johnny
**Date received:** August 18, 2026
**Status:** GOVERNING — read-only archaeological source material

---

## Core Thesis

The years of work in the TSCP archive don't need to be reproduced inside the kernel. They become evidence feeding the kernel.

```
Years of work
    ↓
evidence
    ↓
TSCP Kernel
    ↓
verifiable authority transitions
```

## Central Invariant

**Evidence NEVER creates authority by itself. Everything else derives from that.**

## Proposed Kernel Shape

```
TSCP Evidence-to-Authority Kernel
    │
    ├── canonical evidence
    ├── contract
    ├── predicate
    ├── custody state
    └── proposed transition
        │
        ▼
    deterministic decision
        │
        ┌─────────┴─────────┐
        ▼                   ▼
     ALLOW              DENY
```

## Six Stages

### Stage 1 — Freeze the discovery
Declare the current finding as a provisional architectural fact. Establish the central invariant.

### Stage 2 — Give the ZIP to the super-agent, but give it a job
Mission: Archaeology → Extraction → Kernel Specification

Deliverables:
1. Inventory every potentially authoritative artifact in the ZIP
2. Identify which artifacts belong to the kernel
3. Identify which are historical/prototype/application artifacts
4. Extract existing contracts, custody states, transitions, authority definitions, canonicalization rules, proofs, tests, and receipts
5. Reconcile those against the current TSCP state
6. Produce a proposed TSCP Kernel Charter
7. Produce a Kernel Invariant Registry
8. Produce a Kernel Evidence Matrix
9. Produce a repository-placement recommendation
10. Make no implementation changes

### Stage 3 — Decide where it lives
Options: A) New repository, B) TSCP-PL, C) TSCP Anchor, D) Split architecture. Let archaeology decide.

### Stage 4 — Establish one canonical source of truth
One normative model owns the meaning of: Authority, Evidence, Custody, Contract, Predicate, Transition. Everything else derives from it.

```
        ┌───────────────┐
        │ KERNEL MODEL  │
        └───────┬───────┘
                │
        ┌───────┼───────┐
        ▼       ▼       ▼
      Lean    Rust     TS
        │       │       │
        └───────┼───────┘
                ▼
    CONFORMANCE CORPUS
                │
                ▼
           RECEIPT
```

### Stage 5 — The first actual code should be tiny
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
No dashboard. No database. No network. No blockchain. No agent framework. No ML. No deployment platform. Pure protocol logic.

### Stage 6 — Attack it before expanding it
Attack corpus: Authority attacks, Boundary attacks, Custody attacks, Canonicalization attacks, Replay attacks, Version attacks, Optimization attacks, Serialization attacks.

## Handoff Discipline

Agents may extend implementation; they may not redefine the kernel.

Every handoff carries:
- KERNEL_CHARTER
- KERNEL_INVARIANTS
- CURRENT_STATE
- CURRENT_COMMIT
- OPEN_DECISIONS
- EVIDENCE_INDEX
- TEST_STATUS
- GOVERNANCE_STATUS

## Execution Sequence

```
NOW → ZIP/ARCHAEOLOGY → KERNEL CHARTER → INVARIANT REGISTRY →
FORMAL KERNEL MODEL → TINY REFERENCE ENGINE → ADVERSARIAL CONFORMANCE →
LEAN/RUST/TS REFINEMENT → PROOF ENVELOPE → EVIDENCE REGISTRY →
INDEPENDENT AUDIT → RELEASE
```

Do not skip the first three boxes.

## First GitHub Commit

"TSCP Evidence-to-Authority Kernel Genesis v0.1" containing:
charter, architecture, invariants, terminology, custody model, transition matrix, evidence taxonomy, governance rules, non-goals, completion criteria.

Before implementation. Gives the repository a constitutional layer.

## Recursive Property

"We're going to use the thing we're building to govern the construction of the thing we're building. That's a beautiful recursive property, but we should keep it disciplined rather than mystical."

## Governing Constraint (from Aria's investigation framework)

Do not turn historical recurrence into technical validity. Three separate lanes:
- historical evidence → architectural inference → technical proof

The goal is not to produce a grand narrative. The goal is to determine whether there is a real, minimal, technically defensible kernel hiding in the archaeology, and if so, to identify it precisely enough that we can build or formally specify it.

~Aria
