# Security Policy

## Reporting a Vulnerability

### Primary Channel: GitHub Security Advisories

This repository supports **GitHub Security Advisories**. To report a
security vulnerability:

1. Navigate to the **Security** tab of this repository on GitHub.
2. Click **Report a vulnerability** → **New advisory**.
3. Provide a description, severity assessment (CVSS), and reproduction steps.
4. The repository owner is notified and will respond within 72 hours.

This is the preferred channel. It provides private disclosure, tracked
lifecycle management, and CVE assignment capability.

### Secondary Channel: GPG-Encrypted Email

For reports requiring direct encrypted communication:

- **Recipient:** `adamantinespine@gmail.com`
- **GPG fingerprint:** `8469 2E62 9412 8CC1 C4AC CD15 E747 C3AF 2257 3539`
- **Public key:** `signer-public-key.asc` (in repository root)

Encrypt your report using the public key above. Verify the fingerprint
matches before trusting the key.

### What to Include

- Description of the vulnerability and its impact
- Affected version or commit SHA
- Reproduction steps or proof of concept
- Suggested mitigation (if any)

### Response Timeline

| Stage | Target |
|-------|--------|
| Acknowledgment | 72 hours |
| Initial assessment | 7 days |
| Fix or mitigation | 30 days (severity-dependent) |
| Public disclosure | After fix is released, coordinated with reporter |

## Supported Versions

| Version | Status | Branch/Tag |
|---------|--------|------------|
| v1.0-rc1 | Release candidate | `v1.0-rc1` tag |
| tscp-serialization-v0.1.0-rc1 | Signed release | `tscp-serialization-v0.1.0-rc1-signed` tag |
| Unreleased (master) | Development | `master` branch |

## Commit Signing Policy

All new commits to `master` MUST be GPG-signed with key
`E747C3AF22573539`. GitHub's "Require signed commits" branch
protection rule enforces this for new pushes.

### Historical Unsigned Commits

The repository contains unsigned commits in its history, all from
the pre-signing-setup period (cargo fmt/clippy cleanup commits,
dated 2026-07-19). These commits are:

- `e176f88` — ciborium migration feature commit
- `c048743` through `5bb0bb2` (16 commits) — cargo fmt + clippy cleanup

These are documented here for transparency. They are NOT rewritten.
The project's policy is: **protect future commits, document historical
unsigned commits, do not rewrite history unless an explicit provenance
migration is decided.**

The signed commits that bracket this unsigned range are:
- `81ead10` (signed) — custody merge, before the unsigned range
- `1e5e824` (signed) — ciborium merge, after the unsigned range

## Supply Chain Security

- **Dependency auditing:** `cargo audit` runs in CI on every push and PR
- **Governance registry:** See `SUPPLY_CHAIN_EXCEPTIONS.md` and
  `schemas/exceptions.json` for the exception tracking system
- **Validator:** `scripts/validate_governance.py` enforces bidirectional
  consistency between the exception registry and `audit.toml`
- **CI enforcement:** `.github/workflows/supply-chain-governance.yml`
  validates the registry on every change

## Cryptographic Anchors

- **GPG signing key:** `8469 2E62 9412 8CC1 C4AC CD15 E747 C3AF 2257 3539`
- **Key holder:** Sean Christopher Southwick
- **Key type:** ECDSA P-384
- **Public key file:** `signer-public-key.asc`
- **GitHub verification:** Active (both UIDs registered)

## Security Boundary

This repository (`tscp-anchor`) is the **integration and verification
layer**. It does NOT contain the AVX-512 compute kernels (those are in
the `avx512-butterfly` repository). The security boundary is:

- **tscp-anchor (public):** Verification protocol, serialization contract,
  authority layer, acceptance packages, governance documents
- **avx512-butterfly (private):** Performance kernel implementation

Vulnerabilities in the verification protocol are critical. Vulnerabilities
in the kernel implementation affect performance, not correctness — the
scalar backend is the correctness authority.
