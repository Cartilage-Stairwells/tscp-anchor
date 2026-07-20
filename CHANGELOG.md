# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Supply chain governance layer: JSON Schema (`schemas/exceptions.schema.json`),
  machine-readable registry (`schemas/exceptions.json`), deterministic validator
  (`scripts/validate_governance.py`), and CI enforcement workflow.
- Supply Chain Governance CI workflow (`.github/workflows/supply-chain-governance.yml`):
  pipeline with registry validation, dependency security (cargo audit + cargo deny),
  exception policy enforcement, and evidence bundle generation.
- `SECURITY.md` — vulnerability reporting policy (GitHub Security Advisories
  + GPG-encrypted email), supported versions, commit signing policy,
  historical unsigned commit documentation.
- `RELEASE_PROCESS.md` — release process: versioning, signing protocol,
  CI verification, GitHub Release publication, acceptance package sealing.
- `ARCHIVE_INDEX.md` — updated master index reflecting Cartilage-Stairwells
  as canonical, sealed artifacts, VEP milestones, CI workflows, cryptographic
  anchors, and supply chain exception summary.
- `audit.toml` — ignore entries for active supply chain exceptions (EX-0002,
  EX-0003) with exception ID references in comments.
- `SUPPLY_CHAIN_EXCEPTIONS.md` — governance registry with allOf schema patch
  (status=ACTIVE requires audit field), provenance fields (first_recorded,
  last_reviewed), and change history.

### Changed
- Migrated serialization codec from `serde_cbor` v0.11.2 (unmaintained,
  RUSTSEC-2021-0127) to `ciborium` v0.2.2 (actively maintained,
  IETF RFC 8949 compliant). All 12/12 serialization conformance tests pass.
  Wire-format compatibility preserved — ciborium emits the same CBOR map
  headers as serde_cbor. No test hardening required.
- Updated `CHANGELOG.md` repository links from Triune-Oracle to
  Cartilage-Stairwells (canonical migration).
- Repository migrated from `Triune-Oracle/tscp-anchor` to
  `Cartilage-Stairwells/tscp-anchor` as the canonical remote.
  Triune-Oracle remains as legacy (billing-locked, CI non-functional).

### Resolved
- RUSTSEC-2021-0127 (serde_cbor unmaintained) — RESOLVED via ciborium
  migration. Exception EX-0001 closed in supply chain governance registry.

---

## [tscp-serialization-v0.1.0-rc1] - 2026-07-19

### Added
- Serialization conformance suite (12 tests): byte stability, mutation
  rejection, receipt verification, hash stability, CI reproducibility.
- GPG signing pipeline: key `E747C3AF22573539`, commit + tag signing,
  GitHub verification active with both UIDs.
- `signer-public-key.asc` — exported GPG public key in repository root.
- VEP milestone cross-reference document (`docs/VEP_MILESTONES.md`).
- `SUPPLY_CHAIN_EXCEPTIONS.md` — initial supply chain exception tracking.

### Changed
- Git identity: `user.name=Cartilage-Stairwells`,
  `user.email=adamantinespine@gmail.com`,
  `user.signingkey=E747C3AF22573539`.
- GPG key updated with dual UIDs: `adamantinespine@gmail.com` (primary) +
  `schlagetorren@gmail.com` (secondary). Old key replaced on GitHub.

### Security
- All commits on master GPG-signed and verified on GitHub.
- Branch protection identified as remaining hardening step.

---

## [v1.0-rc1] - 2026-07-17

### Added
- AVX-512 NTT kernels optimized for the BabyBear prime field.
- Phase 1 proof system utilizing the Plonky3 ZK proving stack.
- Anchor state verification modules and authority boundary enforcement.
- Lean 4 formal verification proofs for BabyBear NTT correctness.
- IEP Layer for promotion-policy enforcement.
- VEP-0.1.1 through VEP-0.1.4 milestones on avx512-butterfly:
  minimal evidence loop, full evidence bundle, submission packaging,
  CI enforcement workflow.
- First acceptance package: `tscp-mini-ntt-parity-v1` (24/24 verification
  checks pass, GPG-signed tag).
- GitHub tracking issues #7-10 for VEP milestones.

### Changed
- Refined layer boundaries between mathematical computation, cryptographic
  proving, and authority/identity.
- Repository identity stabilized on Cartilage-Stairwells account.

---

## [0.1.0] - 2026-01-15

### Added
- Initial TSCP kernel and protocol crates.
- tscp-serialization-v0.1 specification.
- AVX-512 NTT kernels for BabyBear prime field.
- Plonky3 ZK proving stack integration.
- Anchor state verification modules.
- Lean 4 formal verification proofs.
- IEP Layer for promotion-policy enforcement.

---

[Unreleased]: https://github.com/Cartilage-Stairwells/tscp-anchor/compare/master...HEAD
[tscp-serialization-v0.1.0-rc1]: https://github.com/Cartilage-Stairwells/tscp-anchor/releases/tag/tscp-serialization-v0.1.0-rc1-signed
[v1.0-rc1]: https://github.com/Cartilage-Stairwells/tscp-anchor/releases/tag/v1.0-rc1
[0.1.0]: https://github.com/Cartilage-Stairwells/tscp-anchor/releases/tag/v0.1.0
