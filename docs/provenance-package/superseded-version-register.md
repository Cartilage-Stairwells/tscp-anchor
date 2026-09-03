---
title: Superseded Version Register
summary: Provenance audit of all known TSCP manuscript copies, tracking v2.0 → v2.1 transition
---

# Superseded Version Register

**Created:** 2026-08-31
**Package:** PP-TSCP-20260831-001 (part of TSCP Provenance Package)
**Authority:** v2.1 (619 lines) is the sole authoritative manuscript. All prior versions are historical/provenance artifacts.

## Active Version

| Artifact | Location | Size | Status | Notes |
|---|---|---|---|---|
| tscp-paper-v2.1.2.md | docs/paper/ (tscp-anchor, git, commit d90b15aa) | 618 lines | **FROZEN / AUTHORITATIVE** | Source markdown. Single source of truth. Three verified Stage 6 P0 corrections applied (exactly 3 lines changed vs v2.1.1); re-audit PASSED (3/3 P0 resolved, no new violations, no regression). |
| tscp-paper-v2.1.2.pdf | docs/paper/ (tscp-anchor, git, commit d90b15aa) | 60,517 bytes | **FROZEN / AUTHORITATIVE** | Built from v2.1.2.md 2026-09-02 (Python markdown + xhtml2pdf pipeline). SHA-256: 01ad5b83a9de7f7867cea9aaf66ced79aeaf01e7218af89c2ebb02cca861aab5. Content-verified post-build; re-audited. |

**Frozen with a known open gate:** the 15 P1 + 2 P2 Stage 6 findings are NOT yet independently verified or dispositioned. Their target quotes persist verbatim in v2.1.2. Freeze of v2.1.2 establishes the P0 correction is verified — it does NOT close the Stage 6 audit. P1/P2 verification/disposition proceeds as a separate gate under separate authorization.

## Version Chain

v2.0 (PDF, pre-fix, "cryptographically sound" in §4.3) → v2.1 (R-01 partially softened: "empirically tested" language; exact R-01 replacement sentence NOT applied; R-02 NOT applied) → v2.1.1 (R-01 exact replacement sentence applied at both locations; both R-02 edits applied; PDF rebuilt and content-verified) → Stage 6 Tier-2 audit (3 P0 / 15 P1 / 2 P2) → independent five-point P0 verification (all 3 CONFIRMED) → **v2.1.2 (three P0 corrections applied — exactly 3 lines — with verification refinement notes incorporated; PDF rebuilt, content-verified; Stage 6 re-audit PASSED: P0 RESOLVED, NO REGRESSION)**

### v2.1 → v2.1.1 transition record (2026-09-02)

Applied edits, exact:
1. §3.3.2 Status and audit section: replaced "is a working prover and verifier with proper Fiat-Shamir, Merkle verification, and fold consistency checks" / "has been implemented and empirically tested, with proper..." with the mandated R-01 sentence: "The oracle-layer FRI implementation (fri_query.rs) passed the ARCHER tests for Fiat-Shamir transcript reconstruction, Merkle verification, and fold consistency. This testing does not establish cryptographic soundness."
2. §1.5: "The protocol's strongest contribution" → "The protocol's central contribution"
3. §8: "The strongest version of TSCP's contribution" → "The central claim of TSCP's contribution"

Verification on v2.1.1 artifacts: zero instances of "strongest"; zero instances of "cryptographically sound"; two instances of the exact R-01 replacement sentence confirmed rendered in the PDF text layer.

## Superseded Versions

| Artifact | Location | Size | Status | Critical Issue | Replacement Action |
|---|---|---|---|---|---|
| tscp-paper-v2.1.1.md / .pdf | docs/paper/ (tscp-anchor, git, commit 6e4e33d3) | 618 lines / 60,234 bytes | **SUPERSEDED by v2.1.2 — FROZEN EVIDENTIARY PREDECESSOR** (2026-09-02) | Not a defect of v2.1.1 itself: the Stage 6 audit (post-deposit) found 3 P0 language violations (2x prover-throughput conflation; 1x Axis 3 overclaim). v2.1.1 remains the frozen audit target and predecessor artifact. | Superseded by tscp-paper-v2.1.2 (commit d90b15aa). SHA-256 of v2.1.1.pdf: 24be832b2e8a09f853aeabb1df2d5c7584b8fa42a5ab8ce8ba735e58681f3407. |
| tscp-paper-v2.1.md / .pdf / .tex |
| tscp-paper-v2.1.md / .pdf / .tex | docs/paper/ (tscp-anchor, git; Drive paper.pdf equivalent) | 618 lines / 97,597 bytes | **SUPERSEDED by v2.1.1** (2026-09-02) | R-01 exact replacement sentence not applied; both R-02 "strongest" instances present. Additionally: v2.1.tex is a STALE older draft (references "12 findings, all addressed") that does not correspond to the v2.1 md/pdf content — retained as historical artifact, must not be built. | Superseded by tscp-paper-v2.1.1.md/.pdf. If the v2.1 PDF was circulated externally, send a replacement notice with v2.1.1. |
| tscp-paper-v2(1).pdf (v2.0) | This conversation (incoming_files/5a49a9298_tscp-paper-v2201.pdf) | 85,238 bytes | **SUPERSEDED** | §4.3 p.17: "cryptographically sound" — security conclusion R-01 removed | Do not circulate. Retain as provenance artifact. |
| paper.pdf (pre-v2) | This conversation workspace (/app/conversations/.../paper.pdf) | 74,341 bytes | **SUPERSEDED** | Pre-Stage 2/3 draft. Earlier than v2.0. | Do not circulate. Historical snapshot only. |

## Vectors Checked

| Vector | Checked | Result | Method |
|---|---|---|---|
| Google Drive | ✅ | Only v2.1 (paper.pdf, updated 2026-08-31) | Drive API (drive.file scope) |
| GitHub repos | ✅ | No PDFs or paper files committed | GitHub API — checked cartilage-stairwells, triune-oracle, kaliforniashell repos |
| GitHub releases | ✅ | No paper assets in any release | GitHub API — checked all releases across 3 accounts |
| IACR ePrint | ✅ | Not submitted yet — no version on ePrint | No submission made |
| Email (Gmail) | ❌ NOT CHECKED | Manual check required | Gmail read access not granted. Check sent items for any paper PDF attachments from [redacted — see Drive copy]. |
| Manuscript transfer services | ✅ | No submissions made | None attempted |

## Manual Action Items

1. **Email sweep** — Check [redacted — see Drive copy] sent items for any paper PDF sent to external recipients before 2026-08-31. If v2.0 was emailed, send a replacement notice with v2.1.
2. **[redacted — see Drive copy]** — Also check this account (used for CLA application) for any paper attachments.
3. **Brother** — If v2.0 was sent to Sean's brother, send v2.1 before he forwards it.

## Provenance Chain

```
pre-v2 (74,341 bytes, Aug 28) → SUPERSEDED
    ↓
v2.0 (85,238 bytes, Aug 31) → SUPERSEDED (contains "cryptographically sound")
    ↓
v2.1 (97,597 bytes, Aug 31) → FROZEN / AUTHORITATIVE (R-01 applied, empirical test observation)
```

v2.0 is retained as a provenance artifact. It is not a candidate manuscript. No modifications to v2.1 are needed based on v2.0's existence.
