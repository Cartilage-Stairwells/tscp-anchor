# Conceptual Layer Correspondence Map

**Status: interpretation (mutable layer).** This document records the mapping
between the conceptual artifacts in `../originals/` and the implemented /
formalized state of the project, including direction of influence. It makes no
claims on behalf of the originals and promotes nothing.

Companion to `../README.md`. Imported 2026-09-17.

---

## 1. `SPRIFES_2026-09-17.txt` — convergent restatement; closure property OPEN

**Content.** Elaborates the seven-operator ontology (EVIDENCE, PROPER, FOLLOW,
RELATIONSHIP, SHARP, SHARK, INTEGRITY) as algebraic primitives with formal type
signatures — e.g. `Evidence : (C,E,W) → Bool`, typed relationship edges
`r = (A,B,ρ,τ,π,β)`, `Sharp : P×E×R → P'`, `Shark : (C,C') → Bool` — and poses the
closure question:

    Proper(P₀) ∧ P₁ = Sharp(P₀, T)  ⟹  Proper(P₁) ∧ Shark(P₀, P₁) ?

**Direction of influence: this document POST-DATES the frozen work.**
The seven operators were validated, implemented, and frozen **before** this
document was written:

- Ontology validated (two collapse tests, B7 instantiation): 2026-08-20
- Rust implementation (`tscp-provenance` crate, `Observed`/`Admissible` types,
  `Admissible` constructible only via checked `admit()`): frozen at `9def88c3`,
  implemented at `4af552b7` (plonoky3-zksha provenance branch, 2026-08-20)
- Lean formalization (`Ontology.lean`, `SeparationLaws.lean`, `Composition.lean`):
  commit `22297d4` (tscp-anchor, branch `provenance/ontology-lean-formalization`,
  2026-08-20)

The type signatures proposed in this document are a **convergent restatement** of
work already implemented — independently authored, arriving at compatible
signatures. This document is *not* the origin of the seven-operator ontology, and
the repositories are *not* derived from it. The term "SPRIFES" appears in no
repository file prior to this import.

**Closure property: OPEN THEOREM CANDIDATE — not established.**
The Rust implementation tests a composition law: *Proper + Sharp + scope-subset
⟹ Shark*. That test is **correspondence evidence** — it shows the implementation
embodies a corresponding rule. It is **not** a proof that the SPRIFES formulation
(P as a named-tuple state, the specific closure conjunction including
`Proper(P₁)`) has the same semantics as the implemented law. Establishing the
exact mapping between the two formulations, then proving (or refuting) the closure
property under it, is a separate formalization target and must use the standard
freeze-first / adversarial procedure. Until then this property is recorded here
as **open**, and nothing downstream may cite it as a theorem.

---

## 2. `SPRIFES-ontology_2026-08-20.txt` and `SPRIFES-ontology-Koda_2026-08-20.txt` — session exports, work executed

**Content.** Export of the 2026-08-20 session that instantiated the seven
operators against the B7 receipt and ran the collapse test.

**Correspondence: executed.** The session it records produced
`ONTOLOGY_B7_INSTANTIATION.md` and `REACHABILITY_CORRECTED_DECOMPOSITION.md` at
commit `41974830` (plonoky3-zksha provenance branch), and re-categorized the
reachability audit under the corrected decomposition. The collapse test passed:
no two operators answer the same question; each catches a distinct laundering
mode (EVIDENCE — scope-widening; PROPER — dependency ≠ call edge; FOLLOW —
traversal without an edge).

**Status.** Record of executed work, not a specification. The two files are
near-identical (the "(Koda)" copy is the same session saved under the assistant
name); both are retained verbatim. The repository documents, not these exports,
are the artifacts of record.

---

## 3. `One-Kernel-to-Completion_2026-08-19.txt` — EAK proposal, NOT implemented

**Content.** ChatGPT session export (source link preserved verbatim in the
original). Proposes the **Evidence Authority Kernel (EAK)**:
Evidence → Canonicalization → Commitment → Predicate evaluation → Admission
decision → Custody transition → Authorized state, with a machine-checkable
custody firewall of **9 custody states, 25 transitions (10 allowed, 15
forbidden)**, and the invariant family "Evidence ≠ Authority, Receipt ≠
Permission, Observation ≠ Acceptance."

**Direction of influence: this document PRE-DATES the ontology freeze by one
day** (2026-08-19 vs 2026-08-20) and shares the Evidence ≠ Authority invariant
with what was frozen the next day.

**Correspondence: NOT implemented.** No repository contains an EAK, the 9-state
custody matrix, or the 25-transition table. The implemented Rust ontology's
`Observed`/`Admissible` distinction (`Admissible` constructible only via the
checked `admit()` function) is a *partial conceptual overlap* — an admission
checkpoint — not an implementation of the EAK, and should not be cited as one.

**Status.** Conceptual source material for a possible future EAK implementation.
Any such implementation is new work with its own specification, freeze, and
adversarial procedure; this document supplies motivation and design vocabulary,
not requirements.

---

## 4. `Socratic-methodology-Kernel_2026-08-20.txt` — methodology, substantially adopted

**Content.** Methodology for (a) admissible vs. historical benchmark measurements
(Experiment A at `experiment_a_afea62bc` / commit `8df0c247` admissible; the
rejected 3.94× historical), (b) theorem-count universes (the number plus its
universe is the claim), (c) Chain-1 source-level reachability evidence
(adapter definition → construction → DFT ownership → prover configuration → proof
entry point → reachability), (d) negative-evidence gradations, and (e)
frozen-state contamination governance (modifying a frozen checkout creates a new
experimental object and must not be presented as evidence about the frozen one).

**Correspondence: substantially adopted.** The B2 execution-path experiment
followed this methodology: new branch (`experiment/b2-execution-path`) from the
known-good integration commit, positive execution evidence (5,504 fused butterfly
calls in a verifying proof, compile-time dispatch, no fallback ambiguity),
semantic invariance established, no performance claims, frozen boundaries
untouched. The Socratic document's governance rules (no manufacturing evidence on
a frozen state; a proof through the path is integration evidence even without
speedup) are visible verbatim in the B2 evidence package discipline.

**Status.** Methodology source. Where this document and the B-series evidence
reports differ, the evidence reports govern.

---

## 5. `investor-pitch-deck-outline_2025-12-09.docx` — historical material only

**Content.** Investor pitch deck outline dated 2025-12-09 — **before** Experiment
A was sealed and before B1–B7, the kernel results, the observation calculus, and
the seven-operator ontology existed.

**Status: NOT evidence of any current claim.** Retained for custody completeness
so the conceptual layer is fully under custody and the stale outline cannot be
confused for an anchored artifact. New funding/pitch material must be built from
verified artifacts only (Experiment A receipts, B2–B7 reports, frozen
boundaries, the theorems in THEOREMS.md), not from this document.

---

## Summary table

| Artifact | Written | Frozen repo work | Relationship | Promotion status |
|---|---|---|---|---|
| SPRIFES (2026-09-17) | after | Aug 20 freeze/impl | Post-dates; convergent restatement | Closure property OPEN; nothing promoted |
| SPRIFES ontology exports (Aug 20) | same day | 41974830 | Record of executed session | Executed record; repo docs govern |
| One Kernel to Completion (Aug 19) | before | Aug 20 ontology | Pre-dates; motivation overlap | EAK not implemented; proposal only |
| Socratic methodology (Aug 20) | same day | B2 experiment | Adopted as methodology | Evidence reports govern |
| Investor pitch outline (Dec 2025) | long before | — | Stale | Historical material only |
