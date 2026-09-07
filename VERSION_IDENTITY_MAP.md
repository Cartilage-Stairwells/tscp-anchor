# Version Identity Map (B-3 / EXC-013)

Added 2026-09-07 (R1 remediation). This repository has historically carried
multiple version identifiers belonging to DIFFERENT version spaces. They are
not errors, but they were previously unmapped, so a reviewer could not tell
which identity governed what. This map states the scheme and the mapping.
Chosen scheme going forward: **SemVer for the npm workspace package**, plus
**object-scoped version IDs** (`<object>-v<x.y.z>`) for frozen/governed
artifacts. No identifier is reused across spaces.

| Identity | Where | Version space | Meaning |
|---|---|---|---|
| `TSCP-IMPL-0001` v**1.0.1** "ratified" | data.json | TSCP implementation object | The ratified version of the TSCP implementation object; data.json is the hashed, manifest-recorded artifact (keccak256 digest in README per EXC-016 relabel) |
| `TSCP-GOV-0001` v**1.0.0** "ratified" | gov.json | TSCP governance object | The ratified governance object version; sibling hashed artifact |
| **1.0.0** | package.json | npm workspace | This repository's npm package version |
| **0.1.0** | CHANGELOG, tag `v0.1.0` | workspace release | First crate/workspace release identity (2026-01-15) |
| **tscp-serialization-v0.1.0-rc1** | CHANGELOG, tag `tscp-serialization-v0.1.0-rc1-signed` | subsystem release | Serialization subsystem release candidate, independently signed |
| **TSCP-v2.4.0-formal-sealed** | release/manifest.json (sealed @ e41096f9, 2026-07-25) | formal seal | The formal-sealed manuscript/release identity; seal recorded in the immutable release manifest |

Every version string in this repository belongs to one of the six spaces
above; historical CHANGELOG entries preserve superseded identities verbatim
and are labeled by their dates. New releases MUST state which space they
version.
