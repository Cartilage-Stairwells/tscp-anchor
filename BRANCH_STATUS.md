# BRANCH_STATUS.md

Provenance and branch disclosure for this repository. It exists so that a reader can tell, without
cloning, which heads are load-bearing, which are archival, which are stranded elsewhere, and how this
repository relates to the older one that shares its name.

It does not assert verification of anything, and it does not modify `CLAIMS_STATEMENT.md`.

Last updated: 2026-10-29

---

## 1. Repository of record

`Cartilage-Stairwells/tscp-anchor` is the repository of record for the TSCP formal corpus. Where this
repository and the predecessor disagree, this one governs.

**Predecessor:** `Triune-Oracle/tscp-anchor`, created 2026-04-27, last pushed 2026-10-20.

The migration was a push, not a GitHub fork, so no fork relationship is recorded by the platform. The
lineage is nevertheless unambiguous: the predecessor's `master` tip **`c054dd14`** exists inside this
repository, which makes this repository a **strict descendant** of it. All 34 tags present on the
predecessor are present here; this repository now carries 60.

A reader who arrives at the predecessor first sees 34 tags, 18 branches and no Release A commit
(`70e614f3`), no `formal-corpus-v2026-09-11` tag, and no `CLAIMS_STATEMENT.md`. A reader who arrives
here sees none of the branches described in section 2. Neither repository states the relationship, which
is why this file exists.

## 2. What did not travel from the predecessor

Seventeen branches remain only on the predecessor. They are not reachable from `master` here and they
are not part of any claim made about the verified core.

**Four carry commits not present in this repository** — 61 commits in total:

| Predecessor branch | Ahead of master | Note |
|---|---|---|
| `custody-merge-ready` | +40 | Rust `crates/oracle-layer` STARK/FRI lineage plus a v1.0-rc1 release manifest, `RE-SIGN.md` and `FINAL_VERIFICATION_LOG_v1.0-rc1.md`. Sampled commit subjects repeat ("fix: cargo fmt + clippy clean — all 24 files"), which suggests a rebase replay. |
| `feat/tscp-mini-ntt-parity-v1` | +12 | A self-contained NTT parity evidence object: `vectors/n8.json`, `vectors/n16.json`, `vector_generator.py`, `verify.py`, `acceptance-evidence.json`, `SHA256SUMS`, `SIGN_TAG.md`, `verification.log`. Dated 2026-07-18 to 07-19. |
| `fix-ci-pipelines` | +8, diverged by 1 | Despite the name, adds an entire `custody-plane/` subsystem: `FCO_Invariants.lean`, `ReceiptComposition.lean`, `FCO_TRANSITION_ALGEBRA.md`, `SEMANTIC_REFERENCE_PROTOCOL.md`, `receipt_validator.rs`, `composition_verifier.py`, `external_verifier.py`, `boundary_firewall.rs`, two JSON schemas, `custody-gate.yml` and `BUILD_ATTESTATION.yaml`. |
| `maintenance/reproduction-status` | +1 | One README line and removal of a compiler warning. |

**Twelve share no common ancestor with `master` at all.** They collapse to five distinct tips, because
every `archive/*` branch has a plain-named twin and `post-freeze` points at the same commit as
`next-dev` (`23b6e174`). Each is a June–July 2026 snapshot of 160–183 files containing three Lean files,
frozen before the histories diverged. They are archival, not pending work.

**One is fully behind:** `stage2-backend-bindings` (0 ahead, 63 behind) is superseded content.

Two of these artifacts are of substance and exist nowhere in this repository: the mini-NTT parity
evidence object and the `custody-plane/` formal subsystem. Nothing in this repository depends on them
and no claim here includes them, but if the predecessor were deleted, those two would be lost with it.
Any future decision about the predecessor should treat it as an archive to be preserved rather than a
duplicate to be retired.

## 3. Branch status in this repository

Measured against `master` at the date above. Twenty-seven heads exist besides `master`; none of their
tips is reachable from `master`. "Ahead" and "behind" are commit counts; a "—" means the branch shares
no common ancestor.

| Branch | Ahead | Behind | Status |
|---|---|---|---|
| `canonical-clean-v1` | — | — | **Orphan history.** 302 commits, 266 files, 9 Lean files, zero merges back. An abandoned clean-room rebuild from mid-2026. Not a source of claims. |
| `provenance/ontology-lean-formalization` | 1 | 84 | **Experimental, do not merge.** Adds three Lean files absent from `master`: `Composition.lean`, `Ontology.lean`, `SeparationLaws.lean`. `SeparationLaws.lean` carries **two `sorry` occurrences** (a third mention is prose in a doc comment), so the three files are outside the verified core. |
| `kernel-archaeology/admissibility-experiment-v0.1` | 8 | 86 | **Experimental, do not merge.** The largest unmerged body in the repository. Its conclusions are not reflected in any claim here. |
| `feature/montgomery-scalar-bridge` | 2 | 127 | **Not equivalent.** `Montgomery.lean` is byte-identical to the `master` copy, but the branch carries a regressive `lakefile`, so equal file size and equal theorem count do not make it a substitute. |
| `docs/canon-evidence-index` | 1 | 8 | Content superseded on `master`. The head is not an ancestor because the change landed by squash. |
| `docs/evidence-custody-index` | 1 | 6 | As above. |
| `docs/provenance-boundaries` | 1 | 7 | As above. |
| `docs/refresh-evidence-index-date` | 1 | 5 | As above. |
| `docs/architecture-freeze-index` | 2 | 105 | Unmerged, not classified for retention. |
| `docs/conceptual-layer-custody` | 1 | 56 | Unmerged, not classified. |
| `docs/continuity-handoff` | 1 | 86 | Unmerged, not classified. |
| `docs/corpus-grok-2026-10-07` | 1 | 54 | Unmerged, not classified. |
| `docs/proof-library-census-errata` | 1 | 54 | Unmerged, not classified. |
| `docs/voxcanon-exception-record-schema` | 1 | 54 | Unmerged, not classified. |
| `docs/custody-boundary-artifacts` | 8 | 105 | Unmerged, not classified. |
| `evidence/plonky3-comparison` | 1 | 150 | Unmerged, not classified. |
| `feature/attestation-evidence-capture` | 1 | 131 | Unmerged, not classified. |
| `feature/formal-verification-manifests` | 3 | 134 | Unmerged, not classified. |
| `feature/release-evidence-bundle` | 1 | 129 | Unmerged, not classified. |
| `feature/release-notes-update` | 1 | 130 | Unmerged, not classified. |
| `feature/reproducibility-audit` | 1 | 128 | Unmerged, not classified. |
| `feature/tscp-attestation-workflow` | 1 | 134 | Unmerged, not classified. |
| `fix/attestation-workflow-elan` | 1 | 132 | Unmerged, not classified. |
| `fix-audit-repro-path` | 1 | 9 | Unmerged, not classified. |
| `formal/bezout-release-note` | 1 | 118 | Unmerged, not classified. |
| `release-a-claims-update` | 1 | 10 | Unmerged, not classified. |
| `release-a-packaging` | 2 | 11 | Unmerged, not classified. |

Branches attached to open pull requests are excluded from any cleanup and are preserved regardless of
the table above.

## 4. Retention policy

1. A branch head is deleted **only** when every commit on it is reachable from `master`. Reachability is
   a graph property and is checked, not assumed.
2. Before that, a judgment is made about whether the head carries unique references, a release-tag
   relationship, or operational significance worth preserving. Equal file sizes and equal theorem counts
   do not establish equivalence.
3. History is **never rewritten.** Evidence records cite commit SHAs, and a rewrite would invalidate
   them. Repositories carry more bytes than their tips because bulk working directories were committed
   and later ignored; that is recorded, not repaired.
4. Disconnected histories are retained rather than merged.
5. Cleanup performed on 2026-10-29: twenty-two fully merged heads were deleted. Four reviewer first-contact
   commits were retained under the tag `reviewer-first-contact-2026-10-28` rather than on a branch head.
   Open pull requests were untouched.

## 5. Preserved misspelling

`kernel-archaeogy` in Correspondence Freeze v0.3 (commit `5645c1f`) is misspelled and is intentionally
preserved. Sealed records reference the path as it was written, so correcting it in place would break
those references.

## 6. Scope of this file

Read-only disclosure. It records what is measurable about branch topology and repository lineage. It does
not change the claims statement, does not merge or delete any branch, and does not assert that any
experimental or stranded work has been verified.
