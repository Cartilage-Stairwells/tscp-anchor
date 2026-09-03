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
| tscp-paper-v2.1.3.md | docs/paper/ (tscp-anchor, git, deposit commit ca3754f3) | 618 lines | **FROZEN / AUTHORITATIVE** | Source markdown. Single source of truth. 14 authorized P1 corrections applied on v2.1.2 (14 sentences across 12 lines); re-audit PASSED (14/14 resolved, 0 new violations, 0 regressions); final independent pre-freeze check PASSED. |
| tscp-paper-v2.1.3.pdf | docs/paper/ (tscp-anchor, git, deposit commit ca3754f3) | 60,811 bytes | **FROZEN / AUTHORITATIVE** | Built from v2.1.3.md 2026-09-03 (Python markdown + xhtml2pdf pipeline). SHA-256: 75f2ee38682c784d8b51e97272366021fe313945d6540e44a8e0605ff363b736. Content-verified post-build (14/14 replacements in text layer, all removed phrasings absent). |

**Accurate audit-state statement (binding — do not recharacterize):** Stage 6 verified 20/20 findings. 3 P0 + 14 authorized P1 corrections are resolved in v2.1.3. P1-08, P2-01, and P2-02 remain **verified-but-deferred** (not waived). §2.4 redundancy and §6.3 old-sentence-removal remain open as P3 editorial notes. Stage 6 does NOT have "zero open findings" and must not be retroactively described that way.

**Provenance chain (v2.1.3):** v2.1.2 (frozen predecessor, commit 1a054453, pdf SHA 01ad5b83…) → 20/20 finding verification (avx512-butterfly 18b36af) → disposition record (avx512-butterfly c82d5d9 — 14 ACCEPT / 3 DEFER, individually enumerated) → correction record (avx512-butterfly cd28c66) → v2.1.3 candidate deposit (tscp-anchor ca3754f3) → independent re-audit PASSED (avx512-butterfly d9c0469) → final independent pre-freeze check PASSED (five-point byte check) → **v2.1.3 FROZEN/AUTHORITATIVE (this commit)**.

## Superseded Versions

| Artifact | Location | Size | Status | Critical Issue | Replacement Action |
|---|---|---|---|---|---|
| tscp-paper-v2.1.2.md / .pdf | docs/paper/ (tscp-anchor, git, commit 1a054453) | 618 lines / 60,517 bytes | **SUPERSEDED by v2.1.3 — FROZEN PREDECESSOR, P0-CORRECTED** (2026-09-03) | Not a defect of v2.1.2 itself: the 15 P1 + 2 P2 findings from the same Stage 6 audit were dispositioned after v2.1.2's freeze; 14 authorized P1 corrections were applied in v2.1.3. v2.1.2 remains the P0-corrected frozen predecessor and the P1/P2 audit target. | Superseded by tscp-paper-v2.1.3 (commit ca3754f3). SHA-256 of v2.1.2.pdf: 01ad5b83a9de7f7867cea9aaf66ced79aeaf01e7218af89c2ebb02cca861aab5. |
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
