# Supply Chain Exceptions

This file documents intentional, reviewed supply-chain advisory exceptions
for the tscp-anchor workspace. Each entry must include the advisory ID,
the affected crate, impact assessment, and the migration path.

Exceptions here are **not ignored** — they are tracked. Future maintainers
must re-evaluate each entry at every dependency update cycle.

---

## Active Exceptions

### RUSTSEC-2021-0127 — `serde_cbor` unmaintained

| Field         | Value                                          |
|---------------|------------------------------------------------|
| Advisory      | RUSTSEC-2021-0127                              |
| Crate         | `serde_cbor` v0.11.2                           |
| Category      | Unmaintained (not a cryptographic vulnerability) |
| Status        | **Tracked migration item — non-blocking**      |
| First noted   | 2026-07-19                                     |
| Reviewed by   | Cartilage-Stairwells / Triune-Oracle           |

**Impact assessment:**

`serde_cbor` is used in the TSCP serialization kernel for canonical CBOR
encoding of `TransitionReceipt` values. The advisory flags the crate as
unmaintained — the upstream author archived the repository in 2021 — but
records no active CVE, memory safety issue, or cryptographic weakness.

The serialization conformance suite (`serialization_conformance.rs`) includes
`test_custody_expression_blocked`, which validates that unknown-field injection
via binary CBOR mutation is rejected through `deny_unknown_fields`. This test
covers the primary attack surface. No active verification failure is associated
with this advisory.

**Why not silently suppressed:**

The advisory is surfaced by `cargo audit` and intentionally allowed in
`audit.toml` via `[[audits.ignore]]`. It is recorded here so the exception
is visible to reviewers outside the CI log context.

**Migration path:**

Replace `serde_cbor` with `ciborium` (actively maintained, IETF RFC 8949
compliant) when the TSCP serialization layer is next refactored. The
canonical encoding invariants in `tscp-kernel/src/types.rs` must be
re-validated against the replacement encoder before the swap is merged.
The conformance suite (`serialization_conformance.rs`, 12 tests) constitutes
the acceptance gate for that migration.

**Blocker for merge:** No.
**Blocker for `tscp-serialization-v1.0` milestone:** Yes — must be resolved
before the v1.0 stability declaration.

---

## Resolved Exceptions

*(None — this is the initial entry.)*

---

## Audit configuration

The `audit.toml` in the workspace root configures `cargo audit` to treat this
advisory as a warning rather than an error, allowing the Verification Seal CI
job to proceed. The `Supply Chain (cargo audit)` CI job will continue to report
`failure` on the advisory until the migration to `ciborium` is complete.

This is the intended state for the `v0.1.0-rc1` release series.
