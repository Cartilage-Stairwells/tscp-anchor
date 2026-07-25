# TSCP v2.4.0 — Formal Sealed Release

**Date:** (to be filled after merge)
**Commit:** (to be filled after merge — will be the merge commit)
**Tag:** TSCP-v2.4.0-formal-sealed (to be created with GPG signature)

## What This Release Seals

The TSCP formal backbone has been independently verified:
- 6/6 Lean modules compile on Lean 4.32.1
- 0 compilation errors
- 0 Classical logic usage
- 2 axioms (hardware/runtime boundary — explicitly classified)
- 3 sorry (NormalizationBridge reflection — explicitly quarantined)

## Custody Invariants (8)

| # | Invariant | Status |
|---|-----------|--------|
| 1 | ProofArtifact (evidence carrier) | ✅ Present |
| 2 | BridgeCertificate.artifact | ✅ Present |
| 3 | governance_transition_preserves_truth | ✅ Present |
| 4 | UtilityFunction (injected policy) | ✅ Present |
| 5 | utility_does_not_affect_admissibility | ✅ Present |
| 6 | PromotionResult.reject : RejectionReason → PromotionResult | ✅ Present |
| 7 | DomainEvidence (kind classification) | ✅ Present |
| 8 | no_rejection removed | ✅ Confirmed absent |

## Verification Manifest

See `formal/verification-manifest.json` for the machine-readable compilation receipt.

## Trust Boundary

See `docs/trust-boundary.md` for the complete analysis of what must be trusted outside Lean.

### Summary

| Gap | Type | Count | On custody path? |
|-----|------|-------|-------------------|
| Hardware execution | Axiom | 1 | No |
| Runtime NTT correctness | Axiom | 1 | No |
| Reflection preimage | Sorry | 3 | No |

## GitHub Attestation

After merging the attestation workflow, GitHub Actions produces a Build Provenance attestation:
- Workflow run ID: (to be filled)
- Attestation digest: (to be filled)
- Artifact: `tscp-verification-receipt-<sha>`

Verify with:
```bash
gh attestation verify formal-sha256sums.txt --repo Cartilage-Stairwells/tscp-anchor
```

## Reproduction

```bash
git clone https://github.com/Cartilage-Stairwells/tscp-anchor.git
cd tscp-anchor
git checkout <commit>
elan default leanprover/lean4:v4.32.1
lake build
# Compare SHA256 with attested evidence
find TSCP/Formal -type f -name "*.lean" -exec sha256sum {} + | sort
```

## Layer Separation

| Layer | Authority | Status |
|-------|-----------|--------|
| TSCP custody | Protocol rules (rulesets, evidence manifest) | ✅ Enforced |
| GitHub attestation | External execution provenance | ✅ (after workflow merge) |
| GPG signature | Personal signer identity | ⏳ (to be added with signed tag) |

## Contents

```
formal/
├── verification-manifest.json    (machine-readable compilation receipt)
├── axioms.json                   (axiom inventory with trust classification)
├── sorry-inventory.json          (sorry inventory with resolution options)
└── build-environment.json        (toolchain details)
docs/
├── trust-boundary.md             (what must be trusted outside Lean)
└── releases/
    └── TSCP-v2.4.0-formal-sealed.md  (this file)
evidence/
├── github-attestation.json       (filled after attestation runs)
├── workflow-run.json             (filled after attestation runs)
└── artifact-digests.txt           (filled after attestation runs)
```
