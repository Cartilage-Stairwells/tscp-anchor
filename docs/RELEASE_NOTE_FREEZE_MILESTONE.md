# Freeze Milestone — Architecture Specification Complete

**Date:** 2026-08-03
**Status:** Closed. Next work is implementation and proof completion.
**Author:** Sean Christopher Southwick

---

## What This Milestone Marks

The TSCP custody plane architecture specification is frozen. This release note defines the relationships between the three artifact surfaces and prevents them from being treated as independent sources of truth.

---

## Authoritative References

| Surface | Role | Location | Identifier |
|---|---|---|---|
| Reviewer snapshot | Frozen executable evidence | GitHub | review-v0.1.6 (commit 7ff9825) |
| Architecture freeze index | Specification navigation root | GitHub / Drive | docs/ARCHITECTURE_FREEZE_INDEX.md |
| Drive document hashes | Authoritative integrity references | Google Drive | 14 SHA256 hashes in freeze index |

---

## Relationship Rules

1. **review-v0.1.6 is immutable.** It is the frozen reviewer snapshot. Do not modify, rebase, or re-tag. Reviewers clone this tag.

2. **ARCHITECTURE_FREEZE_INDEX.md is the navigation root.** Any question about "where is the specification?" starts here. It links the Drive corpus to the GitHub verification surfaces and records all document hashes.

3. **Drive package hashes are authoritative.** If a document's SHA256 does not match the hash recorded in the freeze index, the document has been modified outside the amendment process. The freeze index hash is the reference, not the file on disk.

4. **The three surfaces are not independent.** The Drive documents define the specification. The reviewer repo provides executable evidence. The tscp-anchor repo provides the protocol anchor and this navigation root. They are linked by the freeze index.

5. **Future changes require amendment.** Any modification to the architecture specification must follow the amendment process defined in the freeze index: propose new versioned document, update index with hash, commit to tscp-anchor, sync to Drive.

---

## Architecture Lifecycle

```
Specification    DONE    (11 documents, 6 Lean theorems proven)
Formalization    NEXT    (T1, T4, T5, T6 — Lean proofs)
Implementation   NEXT    (Plonky3/SP1 adapter, conformance vectors)
Conformance      NEXT    (L1-L4 compliance ladder)
Benchmarking     DONE    (3.08x AVX-512, 2.37x AVX2 — measured on AMD Zen 5)
```

---

## What Reviewers Need

1. **Clone the reviewer snapshot:**
   ```
   git clone --branch review-v0.1.6 https://github.com/Cartilage-Stairwells/zksha-rx-reviewer-access
   ```

2. **Read the freeze index:**
   https://github.com/Cartilage-Stairwells/tscp-anchor/blob/docs/architecture-freeze-index/docs/ARCHITECTURE_FREEZE_INDEX.md

3. **Access the specification package:**
   https://drive.google.com/drive/folders/17ogKPlh6qrsMoedW6zH_sWvD295AmMU_

4. **Verify document integrity:**
   Compare any document's SHA256 against the hash recorded in the freeze index.

---

## Freeze Properties

| Property | How Satisfied |
|---|---|
| Discoverability | Freeze index is the single entry point |
| Integrity | 14 SHA256 hashes + 67/67 repo checksums verified |
| Traceability | Every artifact traces to a specification theorem |
| Reproducibility | Clone, build, test, benchmark — all from review-v0.1.6 |

---

## Amendment Process

To change the architecture after this freeze:

1. Draft the amendment as a new versioned document (e.g., REVIEWER_SEMANTICS_v1.1.md)
2. Compute the new document's SHA256
3. Update ARCHITECTURE_FREEZE_INDEX.md with the new hash and status
4. Commit the update to tscp-anchor (signed, per repository rules)
5. Sync the new document to the Drive folder

The specification is frozen. The freeze index tracks its state. This milestone closes the freeze mechanics.

---

*Sean Christopher Southwick — 2026-08-03 — GPG: E747C3AF22573539*
