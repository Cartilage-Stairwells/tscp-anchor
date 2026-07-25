# TSCP Formal Verification Evidence

This directory captures machine-verifiable evidence for the TSCP formal backbone.

## Structure

```
evidence/
├── README.md                    (this file)
├── github-attestation.json      (filled after attestation workflow runs)
├── workflow-run.json             (filled after attestation workflow runs)
└── artifact-digests.txt          (filled after attestation workflow runs)
```

## Provenance Chain

```
Source commit (master)
  ↓
GitHub Actions workflow (tscp-attestation.yml)
  ↓
Lean 4.32.1 build (6/6 modules)
  ↓
SHA256 evidence (formal-sha256sums.txt)
  ↓
GitHub Build Provenance attestation
  ↓
This directory (captured evidence)
```

## Verification

A third party can verify by:
1. Clone the repository at the sealed commit
2. Check the GitHub attestation: `gh attestation verify <subject-digest> --repo Cartilage-Stairwells/tscp-anchor`
3. Run `lake build` to reproduce the compilation
4. Compare their SHA256 sums with the attested evidence
5. Inspect `formal/` for axiom/sorry inventory
6. Read `docs/trust-boundary.md` for the trust boundary analysis
