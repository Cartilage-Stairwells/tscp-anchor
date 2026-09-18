# Conceptual Layer Under Custody

## What this directory is

This directory brings the project's **conceptual source material** — Google Drive
documents and one ChatGPT session export — under repository custody. Before this
import (2026-09-17), none of these artifacts existed in any repository, had source
attribution, or had a recorded relationship to the implemented and formalized work.

This import follows the project's own rule: **an artifact does not become canonical
merely because it is old or because it is now interesting.** Nothing here is promoted
to theorem, specification, or claim status by virtue of being imported. Import gives
the material custody, not authority.

## Structure

```
docs/design/
  README.md                                   (this index)
  originals/                                   (byte-preserved source material)
  correspondence/
    conceptual-layer-correspondence.md        (interpretation + implementation mapping)
```

- `originals/` contains verbatim exports with a custody header prepended. The header
  is clearly marked as added at import and is NOT part of the original document.
  Everything below the `--- ORIGINAL TEXT FOLLOWS (verbatim) ---` separator is the
  byte-preserved export (SHA256 recorded in each header).
- `correspondence/` records the mapping between each artifact and the implemented /
  formalized state of the project, including the **direction of influence** — which
  artifacts pre-date, post-date, or were adopted by the frozen work. It is
  interpretation (mutable layer); the originals are source material.

## Artifact index

| File | Drive date | Provenance status | Correspondence |
|---|---|---|---|
| `originals/SPRIFES_2026-09-17.txt` | 2026-09-17 | Conceptual proposal (not canonical) | Post-dates frozen implementation; convergent restatement. Closure property = OPEN THEOREM CANDIDATE |
| `originals/SPRIFES-ontology_2026-08-20.txt` | 2026-08-20 | Session export (record of executed work) | Work executed at commit 41974830 (plonoky3-zksha provenance branch) |
| `originals/SPRIFES-ontology-Koda_2026-08-20.txt` | 2026-08-20 | Session export (copy of the above, saved under assistant name) | Same as above |
| `originals/One-Kernel-to-Completion_2026-08-19.txt` | 2026-08-19 | Conceptual proposal (NOT implemented) | Proposes the Evidence Authority Kernel (EAK); pre-dates the ontology freeze by one day |
| `originals/Socratic-methodology-Kernel_2026-08-20.txt` | 2026-08-20 | Methodology source (substantially adopted) | Chain-1 reachability methodology; adopted by the B2 execution-path experiment |
| `originals/investor-pitch-deck-outline_2025-12-09.docx` (+ `.txt` extraction) | 2025-12-09 | **Historical material only** | Predates all B1–B7 / kernel results; retained for custody completeness only |

## Rules for this directory

1. **No promotion without procedure.** An artifact here becomes a specification,
   theorem, or claim only through the freeze-first / adversarial procedure used
   everywhere else in TSCP — never by declaration, age, or interest.
2. **No silent rewriting.** The originals are never edited. Corrections,
   refinements, and interpretations live in `correspondence/` or in new
   repository documents with explicit lineage.
3. **Chronology is recorded, not flattened.** Where a Drive document post-dates
   work already implemented in the repositories, the correspondence map says so.
   A later restatement of an earlier frozen result is not the origin of that
   result, and vice versa.
4. **The investor pitch outline is not evidence.** New funding and pitch material
   must be built from verified artifacts (Experiment A receipts, B2–B7 reports,
   frozen boundaries), not from this historical document.

Import performed 2026-09-17 by Koda (Base44 Superagent) for Sean Southwick,
per owner-approved plan (conceptual-layer custody first; ePrint drafting follows).
