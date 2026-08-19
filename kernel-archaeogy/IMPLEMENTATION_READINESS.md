# Implementation-Readiness Assessment & Repository-Placement Recommendation

**Status:** PROPOSED
**Date:** August 18, 2026
**Authority:** TSCP Kernel Charter v0.1

---

## 1. Implementation-Readiness Assessment

### What Is Ready

| Component | Readiness | Evidence |
|:---|:---|:---|
| Canonical serialization (Component A) | ✅ READY | Specified (TSCP-CANON-001 v1.0), implemented (canonical.ts), verified (3 runtimes, 15/15 pass), conformance suite exists |
| Non-self-referential receipts (Component B) | ⚠️ PARTIALLY READY | Specified (Proof Envelope v2), demonstrated (acceptance-receipt.json), but full implementation on HOLD |
| Invariant registry | ✅ READY | 12 invariants identified, 7 enforced, 5 specified |
| Evidence matrix | ✅ READY | All archive artifacts mapped to kernel components |

### What Is NOT Ready

| Component | Gap | Required Work |
|:---|:---|:---|
| Decision function (Component C) | NO IMPLEMENTATION | Specify contract/predicate language, implement evaluate(), define custody state machine |
| Separation invariant (Component D) | NOT ENFORCED | Implement enforcement in code (evaluate must reject evidence-only authority claims) |
| Formal model | NO LEAN MODEL | Build Lean formal model of kernel components (current Lean work covers NTT, not kernel) |
| Adversarial corpus | NO ATTACKS | Design and execute attack suite (authority, boundary, custody, canonicalization, replay, version, optimization, serialization) |
| Independent audit | NO EXTERNAL AUDIT | Have external party verify implementation against specification |

### Readiness Verdict

The kernel is **ready for the first implementation experiment** (the tiny evaluate() function from Stage 5 of the founding document) but **NOT ready for production implementation or repository creation**.

The canonicalization layer is production-grade. The decision/authority layer is at the specification stage.

---

## 2. Repository-Placement Recommendation

### Option D: Split Architecture (RECOMMENDED)

```
TSCP-Kernel
    ├── formal model (Lean)
    ├── reference implementation (language TBD — likely TypeScript or Rust)
    └── conformance corpus

TSCP-PL
    └── protocol contracts using kernel

TSCP Anchor
    └── operational implementation using kernel
```

### Rationale

1. **The kernel is protocol-independent.** TSCP-CANON-001 specifies canonical serialization without depending on any protocol language, blockchain, or runtime. Forcing it into TSCP-PL or TSCP Anchor conflates the primitive with its consumers.

2. **The "one canonical source of truth" principle requires separation.** If the kernel lives inside TSCP-PL or TSCP Anchor, changes to the consumer could leak into the primitive. A separate repository makes the inheritance boundary explicit.

3. **The conformance corpus must be independent.** The acceptance-receipt.json pattern (three independent runtimes inheriting from one spec) requires the spec to live above any implementation. A dedicated kernel repository makes this structure visible.

4. **The founding document anticipates this.** Stage 3 explicitly lists Option D as a plausible outcome and says "we should let the archaeology tell us." The archaeology confirms: the kernel is used by both TSCP-PL (protocol contracts) and TSCP Anchor (operational implementation). Therefore it cannot live inside either.

### Timing

**Defer repository creation until the evaluate() experiment succeeds.**

Creating a repository before the decision function is implemented risks:
- Semantic drift (repository structure implies decisions not yet made)
- Premature commitment to a contract/predicate language
- The "initial project setup" commit the founding document warns against

**First meaningful commit**: Kernel Charter + Invariant Registry + Evidence Matrix + successful evaluate() experiment = "TSCP Evidence-to-Authority Kernel Genesis v0.1"

---

## 3. Recommended Next Steps (in order)

1. **✅ DONE** — Archaeology (this investigation)
2. **✅ DONE** — Kernel Charter (this document)
3. **✅ DONE** — Invariant Registry
4. **✅ DONE** — Evidence Matrix
5. **NEXT** — Implement evaluate() as pure protocol logic (tiny, no infrastructure)
6. **THEN** — Design adversarial attack corpus (8 attack categories from Stage 6)
7. **THEN** — Execute attacks against evaluate()
8. **THEN** — Create repository with first meaningful commit (charter + passing evaluate + defended attacks)
9. **THEN** — Build Lean formal model
10. **THEN** — Independent audit
