# Supply Chain Exceptions

This file documents intentional, reviewed supply-chain advisory exceptions
for the tscp-anchor workspace. Each entry must include the advisory ID,
the affected crate, impact assessment, and the migration path.

Exceptions here are **not ignored** — they are tracked. Future maintainers
must re-evaluate each entry at every dependency update cycle.

---

## Active Exceptions

*(None)*

---

## Resolved Exceptions

### RUSTSEC-2021-0127 — `serde_cbor` unmaintained — RESOLVED

| Field              | Value                                          |
|--------------------|------------------------------------------------|
| Advisory           | RUSTSEC-2021-0127                               |
| Crate              | `serde_cbor` v0.11.2                           |
| Category           | Unmaintained (not a cryptographic vulnerability) |
| Status             | **RESOLVED — migrated to `ciborium`**          |
| First noted        | 2026-07-19                                     |
| Resolved           | 2026-07-19                                     |
| Reviewed by        | Cartilage-Stairwells                            |

**Migration record:**

`serde_cbor` was replaced with `ciborium` v0.2.2 (actively maintained,
IETF RFC 8949 compliant). The migration is a codec implementation change,
not a canonical identity transition:

- **Identity primitive:** Unchanged. `TransitionReceipt::hash()` uses
  domain-separated BLAKE3 on structured fields, not CBOR byte representation.
- **Hash values:** All existing receipt hashes remain valid.
- **Custody boundary:** Unchanged. `deny_unknown_fields` enforcement
  preserved through ciborium's serde implementation.
- **Conformance suite:** 12/12 tests pass with ciborium encoder/decoder.

**Acceptance statement:**

> The ciborium migration preserved the TSCP identity primitive because
> identity is derived from structured-field BLAKE3 hashing, not CBOR
> byte representation.

---

## Audit configuration

The `audit.toml` in the workspace root previously configured `cargo audit`
to treat RUSTSEC-2021-0127 as a warning. With the migration complete, the
exception entry in `audit.toml` may be removed. The `Supply Chain (cargo
audit)` CI job will now report `success` as no active exceptions remain.

This is the intended state for the `tscp-serialization-v1.0` release series.
