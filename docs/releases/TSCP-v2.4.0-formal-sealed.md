# TSCP v2.4.0 — Formal Sealed Release

**Date:** 2026-07-25
**Commit:** bf5f9dff80b3140c97a7608ec0e6de8cf2d64427
**Attested commit:** f280f35dcbef1d58564dee61a4080bcc028dd3d3
**Tag:** TSCP-v2.4.0-formal-sealed (to be created with GPG signature)

## What This Release Seals

The TSCP formal backbone has been independently verified by GitHub Actions CI:
- 6/6 Lean modules compile on Lean 4.32.1
- 0 compilation errors
- 0 Classical logic usage
- 2 axioms (hardware/runtime boundary — explicitly classified)
- 3 sorry (NormalizationBridge reflection — explicitly quarantined)

## GitHub Attestation

**Workflow:** TSCP Verification Attestation
**Run ID:** 30159889545
**Run number:** 2
**Commit:** f280f35dcbef1d58564dee61a4080bcc028dd3d3
**Conclusion:** success (all 9 steps passed)
**Attestation type:** Build Provenance (actions/attest-build-provenance@v2)
**Artifact:** tscp-verification-receipt-f280f35d (2030 bytes, 90-day retention)

### Verify the attestation

```bash
# Download the artifact from GitHub Actions
gh run download 30159889545 --repo Cartilage-Stairwells/tscp-anchor

# Verify the build provenance attestation
gh attestation verify formal-sha256sums.txt --repo Cartilage-Stairwells/tscp-anchor
```

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

## Trust Boundary

See `docs/trust-boundary.md` for the complete analysis.

| Gap | Type | Count | On custody path? |
|-----|------|-------|-------------------|
| Hardware execution | Axiom | 1 | No |
| Runtime NTT correctness | Axiom | 1 | No |
| Reflection preimage | Sorry | 3 | No |

All 3 sorry share the same root cause (surjectivity of rename function) and are in the reflection (reverse) direction — not on the custody path. If `normalization_bridge` is later restricted to bijective renamings, all 3 can be simultaneously eliminated.

## Machine-Readable Manifests

| File | Contents |
|------|----------|
| `formal/verification-manifest.json` | 6/6 modules, 0 errors, 0 Classical, 8 custody invariants |
| `formal/axioms.json` | 2 axioms with classification, justification, replacement plans |
| `formal/sorry-inventory.json` | 3 sorry with resolution options (all require surjectivity) |
| `formal/build-environment.json` | Lean 4.32.1, Lake 5.0.0, toolchain details |
| `evidence/github-attestation.json` | Workflow run metadata + attestation reference |
| `evidence/workflow-run.json` | Step-by-step CI results |
| `evidence/artifact-digests.txt` | Artifact + attestation digests |

## Reproduction

```bash
git clone https://github.com/Cartilage-Stairwells/tscp-anchor.git
cd tscp-anchor
git checkout bf5f9df
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain leanprover/lean4:v4.32.1
source ~/.elan/env
lake build
```

## Layer Separation

| Layer | Authority | Status |
|-------|-----------|--------|
| TSCP custody | Protocol rules (rulesets, evidence manifest) | ✅ Enforced |
| GitHub attestation | External execution provenance | ✅ Active (Run #2) |
| GPG signature | Personal signer identity | ⏳ (this tag) |

## Provenance Chain

```
Source commit (f280f35d)
  ↓
GitHub Actions (Run #2, 30159889545)
  ↓
Lean 4.32.1 build (6/6 modules PASS)
  ↓
SHA256 evidence (formal-sha256sums.txt)
  ↓
GitHub Build Provenance attestation
  ↓
Evidence captured on master (bf5f9df)
  ↓
GPG-signed tag (TSCP-v2.4.0-formal-sealed) ← personal seal
```
