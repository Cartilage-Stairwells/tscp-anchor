# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Integration of Plonky3 zero-knowledge proving stack elements in development.
- Initial WASM smoke-testing execution harness.
- Verification scaffolding for the AVX-512 butterfly NTT routines.

### Changed
- Refined layer boundaries between mathematical computation, cryptographic proving, and authority/identity.

---

## [0.1.0] - 2026-01-15

### Added
- **avx512-butterfly**: High-performance AVX-512 NTT kernels optimized for the BabyBear prime field.
- **tscp-pl-phase1**: Phase 1 proof system utilizing the Plonky3 ZK proving stack.
- **tscp-anchor**: Anchor state verification modules and authority boundary enforcement.
- **tscp-canon**: Lean 4 formal verification proofs for BabyBear NTT correctness.
- **IEP Layer**: Instrumented Evaluation Protocol for promotion-policy enforcement.
- **tscp-serialization-v0.1**: Initial serialization specification.
  - *Status:* `FROZEN_SPECIFICATION` — pending `VERIFICATION_PACKAGE_PASS`.

[Unreleased]: https://github.com/Cartilage-Stairwells/tscp-anchor/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Cartilage-Stairwells/tscp-anchor/releases/tag/v0.1.0
