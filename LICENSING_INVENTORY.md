# Licensing inventory (verified 2026-11-15)

Scope: all 29 owner repositories under `Cartilage-Stairwells`. Method: GitHub
`/license` detection endpoint per repository, read-only. This document records
**observed state only**. It selects no license and changes no repository: license
choice is an owner decision.

## Repositories with a declared license (10)

| Repository | Visibility | Detected |
| --- | --- | --- |
| avx512-butterfly | private | Apache-2.0 |
| avx512-butterfly-evidence | public | Apache-2.0 |
| canonical-lexicon | public | Apache-2.0 |
| plonky3-zksha | private | Apache-2.0 |
| tscp-anchor | public | Apache-2.0 |
| tscp-anchor-review | public | Apache-2.0 |
| tscp-canon | public | Apache-2.0 |
| tscp-v04-p0-baseline | public | Apache-2.0 |
| zksha-rx-reviewer-access | public | Apache-2.0 |
| babybear-core-verified | public | MIT |

## Repositories with no LICENSE file (19)

| Repository | Visibility |
| --- | --- |
| tscp.store | **public** |
| agent-handoff | private |
| experiment-5-6b-evidence | private |
| experiment-7-evidence | private |
| experiment-8a-evidence | private |
| fp-vae-benchmark | private |
| fp-vae-research-handoff | private |
| frost-openpgp-research | private |
| immortalization-engine | private |
| kaggriculture-fable | private |
| meridian-ops | private |
| triumvirate-archaeology | private |
| triumvirate-axion | private |
| tscp-evidence | private |
| tscp-pl-phase1 | private |
| tscp-pl-physics-thread | private |
| tscp-workstream-records | private |
| tscp-zkptp | private |
| zksha-rx | private |

## Standing flags

- **zksha-rx** is recorded elsewhere as under provenance review and must **not**
  be treated as Apache-2.0. It currently carries no LICENSE file, which is
  consistent with that status; the absence is not evidence of permission.
- **tscp.store** is the only *public* repository without a license. Absent a
  license, default copyright applies and downstream reuse rights are unstated.
  This is the highest-consequence item in this inventory and is an owner decision.
- Private repositories without a license are not a public exposure, but they
  share the same ambiguity for any future publication or handoff.

## Not established here

Whether any license text exists in commit history of the repositories above
`/license` reported as unlicensed. That requires per-repository history
inspection and is recorded as unverified rather than absent.
