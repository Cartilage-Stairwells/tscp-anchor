---
title: TSCP Manuscript Provenance Capsule
summary: Records the manuscript's state and provenance at the point of transfer/review. Part of the TSCP provenance package.
---

# TSCP Manuscript Provenance Capsule

**Artifact ID:** MC-TSCP-v2.1-20260831
**Frozen:** 2026-08-31
**Status:** CURRENT (at time of capsule creation)

---

## Manuscript Identity

| Field | Value |
|---|---|
| Title | TSCP: Cryptographic Custody Verification for Multi-Agent Computation Pipelines |
| Version | v2.1 |
| Lines | 619 |
| Source file | paper-draft.md |
| PDF | paper.pdf (97,597 bytes) |
| PDF (Google Drive) | id: 1vgpu8KB_4AVxSziBsnOfqWp60-17WFMF |

---

## Revision History

| Version | Date | Size | Status | Key Change |
|---|---|---|---|---|
| pre-v2 | 2026-08-28 | 74,341 bytes | SUPERSEDED | Initial draft, pre-Stage 2/3 |
| v2.0 | 2026-08-31 | 85,238 bytes | SUPERSEDED | Pre-R-01. Contains "cryptographically sound" in §4.3 |
| v2.1 | 2026-08-31 | 97,597 bytes | **FROZEN / AUTHORITATIVE** | R-01/P0-A applied. Empirical test observation language. §4.5 Binding Audit added. §7.4 Epistemic Scope added. |

---

## Review Stages Applied

| Stage | Description | Result |
|---|---|---|
| Stage 1 | Claim register freeze | FORMALLY_MODELED status established |
| Stage 2 | Evidence-to-prose audit | 61 claims checked, 9 overclaims fixed |
| Stage 3 | Adversarial review | 17 findings (F-001–F-017), all FIX items applied |
| Stage 4 | Novelty comparison matrix | DISTINGUISHABLE (QUALIFIED) across 3 axes |

---

## Key Epistemic Boundaries

1. **No security conclusion from absence of findings.** "No fundamental cryptographic flaws were found" → "The ARCHER audit examined... and observed no fundamental cryptographic flaws in the tested portions."

2. **Artifact-to-computation binding is OPEN.** H(A)=h and Verify(pi,C)=true do not yet imply A=Output(C). Nine seams must be closed. Documented in §4.5.

3. **Coherence is not evidence.** Generated explanations are not recovered provenance. Internal consistency is not external authority. §7.4.

4. **FORMALLY_MODELED ≠ PROVEN ≠ IMPLEMENTED.** The claim register distinguishes these explicitly.

---

## Outstanding Items (Not Blockers for Frozen Status)

- 3 formal axioms remain open in zksha-rx (machine refinement — honest engineering boundaries)
- Sumcheck verifier not implemented (documented)
- Artifact-to-computation binding constraint not implemented (documented as open requirement)
- 7 implementation gaps identified for grant-funded work (§3.6)

---

## Authorship Attribution

Per Layered Authorship Framework v1.1:

- **Primary Author:** Sean Christopher Southwick
- **Computational Assistance:** Non-human automated systems provided drafting assistance, structural editing, audit execution, and epistemic review
- **Contribution Level:** Level 2 (Assisted Development) — original concept by SCS with computational assistance in development
- **Authorship status:** Provenance record. Not a legal determination of ownership.

---

## Transfer/Review State

This capsule records the manuscript's state at the point of provenance packaging. The manuscript is:

- Frozen at v2.1 (619 lines)
- Ready for IACR ePrint submission (not yet submitted)
- Not under peer review
- Not modified after R-01 application

---

## Scope Disclaimer

This capsule documents the manuscript's provenance state. It does not constitute:
- A legal determination of intellectual-property ownership
- A peer-review endorsement
- A security audit conclusion (the ARCHER audit is an empirical test observation, not a security proof)
- A verification of the manuscript's formal claims (those are established by the claim-evidence register, not this capsule)
