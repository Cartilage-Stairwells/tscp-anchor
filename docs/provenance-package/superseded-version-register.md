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
| paper.pdf (v2.1) | Google Drive (id: 1vgpu8KB_4AVxSziBsnOfqWp60-17WFMF) | 97,597 bytes | **FROZEN / AUTHORITATIVE** | Updated 2026-08-31. R-01/P0-A applied. §4.3 uses empirical test observation language. |
| paper-draft.md (v2.1) | /app/notes/tscp-strategy/paper-draft.md | 619 lines | **FROZEN / AUTHORITATIVE** | Source markdown. Single source of truth. |
| paper.pdf (v2.1) | /app/notes/tscp-strategy/paper.pdf | 97,597 bytes | **FROZEN / AUTHORITATIVE** | Local copy of Google Drive file. |

## Superseded Versions

| Artifact | Location | Size | Status | Critical Issue | Replacement Action |
|---|---|---|---|---|---|
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
