---
title: TSCP Provenance Package — MANIFEST
summary: Controlling document for the TSCP provenance package. Establishes package identity, artifact inventory, and evidence boundaries.
---

# TSCP Provenance Package — MANIFEST

**Package ID:** PP-TSCP-20260831-001
**Created:** 2026-08-31T21:39:00-07:00
**Status:** FROZEN
**Authorship Policy:** Layered Authorship Framework v1.1

---

## Purpose

This package bundles three provenance artifacts that serve distinct evidentiary functions. Together they establish a chain:

**artifact → attribution policy → version identity → transfer/review state**

This is a **provenance package**, not a "proof of authorship" package. It documents what is established; it does not assert more than its evidence supports.

---

## Scope Disclaimer

This package does not constitute:
- A legal determination of intellectual-property ownership
- A peer-review endorsement of the TSCP manuscript
- A security audit conclusion (ARCHER findings are empirical test observations, not security proofs)
- A verification of formal claims (those are established by the claim-evidence register)

Authorship attribution per v1.1 is a provenance record, not automatically proof of authorship or ownership.

---

## Artifact Inventory

### 1. Layered Authorship Framework v1.1

| Field | Value |
|---|---|
| Artifact ID | LAF-001 |
| File | layered-authorship-framework-v1.1.md |
| Version | v1.1 |
| Lines | 153 |
| Status | CURRENT |
| Authorship | Primary Author: Sean Christopher Southwick. Computational Assistance: structural organization, formatting, epistemic review. Contribution Level: Level 2 (Assisted Development). |
| Evidence status | Documents human and computational contributions. Distinguishes authorship attribution from legal ownership. Establishes evidence requirements for each provenance layer. Does not establish legal compliance. |
| Relationship to package | Defines the authorship/provenance policy under which all other artifacts in this package are attributed. Controlling policy document. |

### 2. Superseded-Version Register

| Field | Value |
|---|---|
| Artifact ID | SVR-TSCP-001 |
| File | superseded-version-register.md |
| Version | v1.0 |
| Lines | 49 |
| Status | CURRENT |
| Authorship | Primary Author: Sean Christopher Southwick. Computational Assistance: verification execution, register compilation. Contribution Level: Level 2 (Assisted Development). |
| Evidence status | Establishes version history. Confirms v2.1 as sole authoritative manuscript. Labels v2.0 and pre-v2 as SUPERSEDED. Identifies one unchecked vector (email). Documents provenance chain: pre-v2 → v2.0 (superseded) → v2.1 (frozen). |
| Relationship to package | Prevents obsolete manuscript versions from being mistaken for current ones. Critical given v2.0 contains "cryptographically sound" language that v2.1 deliberately removed. |

### 3. TSCP Manuscript Provenance Capsule

| Field | Value |
|---|---|
| Artifact ID | MC-TSCP-v2.1-20260831 |
| File | manuscript-capsule.md |
| Version | v1.0 |
| Lines | 97 |
| Status | CURRENT |
| Authorship | Primary Author: Sean Christopher Southwick. Computational Assistance: drafting assistance, structural editing, audit execution, epistemic review. Contribution Level: Level 2 (Assisted Development). |
| Evidence status | Records manuscript state at point of transfer/review. Documents 4 review stages applied, 4 epistemic boundaries, outstanding items, and transfer/review state. Manuscript is frozen at v2.1 (619 lines). Not under peer review. Not yet submitted to IACR ePrint. |
| Relationship to package | Records the manuscript's provenance state at the point of packaging. Links manuscript identity to the version register and attribution policy. |

---

## Evidence Chain

```
Layered Authorship Framework v1.1 (attribution policy)
    ↓ defines attribution standards
TSCP Manuscript v2.1 (619 lines, frozen)
    ↓ version identity established by
Superseded-Version Register (v2.0 → SUPERSEDED, v2.1 → AUTHORITATIVE)
    ↓ transfer/review state recorded by
Manuscript Provenance Capsule (4 stages applied, epistemic boundaries documented)
```

---

## Integrity

SHA-256 hashes for all artifacts are recorded in `SHA256SUMS`. Any modification to package contents invalidates the hash file. To verify:

```bash
sha256sum -c SHA256SUMS
```

If hashes match, the package is in its frozen state. If they do not match, the package has been modified after freezing.

---

## Package Authorship

**Primary Author:** Sean Christopher Southwick
**Computational Assistance:** Non-human automated systems provided structural organization, verification execution, and epistemic review
**Contribution Level:** Level 2 (Assisted Development) per LAF v1.1
**Authorship status:** Provenance record. Not a legal determination of ownership.
