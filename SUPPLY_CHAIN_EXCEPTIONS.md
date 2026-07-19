# Supply Chain Exceptions Registry

**Schema version:** 1.0  
**Frozen:** 2026-07-19  
**Custodian:** Cartilage-Stairwells  
**GPG signing key:** `84692E6294128CC1C4ACCD15E747C3AF22573539`

---

## 1. Governance Metadata

This document is a **versioned governance registry** for the tscp-anchor workspace.
It tracks intentional, reviewed supply-chain advisory exceptions.

The schema structure is frozen at v1.0. Changes to the top-level section
list require a schema version increment. Exception records may be added,
updated, or archived without a schema version change.

**Machine-readable form:** `schemas/exceptions.json` (validated against
`schemas/exceptions.schema.json` by `scripts/validate_governance.py`).

**Immutability rule:** Exception IDs are permanent and SHALL NOT be
reassigned. If an exception is removed or archived, its ID is retired
permanently and MUST NOT refer to a different exception in the future.

**Governance vs. repository policy:** This document defines *what* is
required for supply-chain governance. How the repository *enforces* those
requirements (blocking CI checks, required reviewers, automated issues)
is repository policy, defined separately in `audit.toml` and CI workflow
configuration.

---

## 2. Governance Principles

1. Exceptions are **not ignored** — they are tracked, reviewed, and
   resolved or explicitly accepted with documented rationale.
2. Every exception must have an **owner** responsible for its lifecycle.
3. Every exception must record **evidence by reference**, not by embedding.
4. Exceptions are re-evaluated at every dependency update cycle.
5. Resolved exceptions are retained in the registry for audit trail.
6. The registry is consumable by reviewers, CI, and future tooling.

---

## 3. Exception Schema

### Required Fields

| Field            | Description                                          |
|------------------|------------------------------------------------------|
| Exception ID     | Permanent identifier (format: `EX-NNNN`)            |
| Status           | One of: `PROPOSED`, `ACTIVE`, `MITIGATED`, `RESOLVED`, `ARCHIVED` |
| Dependency       | Crate name and version                               |
| Owner            | Person or team responsible for the exception        |
| Evidence         | References to CI runs, commits, advisory IDs, audit artifacts |
| Resolution       | Action taken (for RESOLVED/MITIGATED) or Rationale (for ACTIVE) |
| First Recorded   | Date the exception first entered the registry        |
| Last Reviewed    | Date of most recent governance review                |

### Optional Fields

| Field                  | Description                                      |
|------------------------|--------------------------------------------------|
| Associated Advisories  | RUSTSEC IDs or other advisory database references |
| Planned Mitigations    | Scheduled actions with target dates             |
| Next Review            | Date for next mandatory review                  |
| Notes                  | Free-form context                                |
| Audit                  | RUSTSEC ID(s) ignored in audit.toml. Required when status is ACTIVE. |

### Conditional Requirement (allOf)

When `status` is `ACTIVE`, the `audit` field is REQUIRED. This closes
the gap where `status: active` with missing `audit` could pass schema
validation. Enforced in both `exceptions.schema.json` (allOf/if-then)
and `validate_governance.py`.

### Status Lifecycle

```
PROPOSED -> ACTIVE -> MITIGATED -> RESOLVED -> ARCHIVED
                ^                              |
                +-------- (reopened if regression detected)
```

### Provenance Model

```
first_recorded
      |
      v
review history (last_reviewed)
      |
      v
resolution
```

Exception IDs are immutable. `first_recorded` is set once and never
changed. `last_reviewed` updates on each governance review.

---

## 4. Resolved Exceptions

### EX-0001 — `serde_cbor` unmaintained

| Field            | Value                                              |
|------------------|----------------------------------------------------|
| Exception ID     | EX-0001                                            |
| Status           | RESOLVED                                           |
| Dependency       | `serde_cbor` v0.11.2                               |
| Owner            | Cartilage-Stairwells                                |
| Evidence         | RUSTSEC-2021-0127; merge commit `1e5e8249`; CI run on `1e5e8249` (all 4 workflows green); `cargo audit` no longer reports this advisory |
| Resolution       | Migrated to `ciborium` v0.2.2 (actively maintained, IETF RFC 8949 compliant). Codec implementation change, not canonical identity transition. Identity primitive (BLAKE3 on structured fields) unchanged. 12/12 serialization conformance tests pass. `test_custody_expression_blocked` passed without modification — ciborium emits same `0xA5` CBOR map header. |
| First Recorded   | 2026-07-19                                         |
| Last Reviewed    | 2026-07-19                                         |
| Notes            | Classification: codec migration, NOT identity transition. Acceptance gate: serialization conformance suite (12 tests). |

---

## 5. Active Exceptions

### EX-0002 — `crossbeam-epoch` 0.9.18 (RUSTSEC-2026-0204)

| Field            | Value                                              |
|------------------|----------------------------------------------------|
| Exception ID     | EX-0002                                            |
| Status           | ACTIVE                                             |
| Dependency       | `crossbeam-epoch` v0.9.18 (transitive via `wasmtime`) |
| Owner            | Cartilage-Stairwells                                |
| Evidence         | RUSTSEC-2026-0204; `cargo audit` on CI run `1e5e8249` |
| Resolution       | Rationale: transitive dependency via wasmtime. Not a direct dependency. Upgrade to `crossbeam-epoch >=0.9.20` requires wasmtime version bump. Tracked under EX-0003 batch upgrade. |
| Audit            | RUSTSEC-2026-0204                                  |
| First Recorded   | 2026-07-19                                         |
| Last Reviewed    | 2026-07-19                                         |
| Planned Mitigations | Upgrade wasmtime to >=36.x (resolves both EX-0002 and EX-0003) |
| Next Review      | 2026-08-19                                         |

### EX-0003 — `wasmtime` 29.0.1 (6 advisories)

| Field            | Value                                              |
|------------------|----------------------------------------------------|
| Exception ID     | EX-0003                                            |
| Status           | ACTIVE                                             |
| Dependency       | `wasmtime` v29.0.1                                 |
| Owner            | Cartilage-Stairwells                                |
| Evidence         | RUSTSEC-2026-0088 (low, data leakage pooling allocators); RUSTSEC-2026-0093 (medium 6.9, heap OOB read UTF-16); RUSTSEC-2026-0087 (medium 4.1, segfault f64x2.splat Cranelift); RUSTSEC-2026-0092 (medium 5.9, panic misaligned UTF-16); RUSTSEC-2026-0021 (medium 6.9, panic excessive fields wasi:http); RUSTSEC-2025-0118 (low 1.8, unsound shared linear memory); `cargo audit` on CI run `1e5e8249` |
| Resolution       | Rationale: wasmtime is used only by `tscp-wasm-smoke` (runtime smoke test), not by the core serialization or proving stack. No untrusted WASM is executed. Upgrade path: wasmtime >=36.0.7 resolves all 6 advisories. Major version bump (29 to 36+) requires API migration. |
| Audit            | RUSTSEC-2026-0088, RUSTSEC-2026-0093, RUSTSEC-2026-0087, RUSTSEC-2026-0092, RUSTSEC-2026-0021, RUSTSEC-2025-0118 |
| First Recorded   | 2026-07-19                                         |
| Last Reviewed    | 2026-07-19                                         |
| Planned Mitigations | Upgrade wasmtime to >=36.0.7 (API migration required) |
| Next Review      | 2026-08-19                                         |

---

## 6. Automated Drift Detection

**Governance requirement:** Dependency drift must be detected and reviewed.

**Repository policy (enforcement):** The `Supply Chain (cargo audit)` CI job
in `.github/workflows/ci.yml` runs `cargo audit` on every push and pull request.
Advisories listed in `audit.toml` `[advisories].ignore` are suppressed in
cargo audit output — they correspond to ACTIVE exceptions in this registry.
New unignored advisories require a registry entry (PROPOSED status) before
the next release.

**Supply Chain Governance workflow** (`.github/workflows/supply-chain-governance.yml`)
validates the registry on changes to `schemas/exceptions.json`,
`audit.toml`, or related files. It checks:
1. Registry schema conformance
2. Audit mapping consistency (active exceptions ↔ audit.toml ignores)
3. Orphan ignore detection
4. Exception policy (next_review dates, escalation)

**Trigger for review:** Any new advisory appearing in `cargo audit` output
that is not already covered by an active exception.

---

## 7. Supply Chain Monitoring

**Governance requirement:** Dependencies are re-evaluated at every
dependency update cycle.

**Review cadence:**
- Each active exception has a `Next Review` date.
- Exceptions without a `Next Review` date are reviewed at minimum quarterly.
- A dependency update (`cargo update`) triggers re-evaluation of all
  active exceptions.

**Escalation:** An exception past its `Next Review` date without a review
MUST be escalated before any release.

---

## 8. Approval Workflow

```
1. New advisory detected by cargo audit or manual review
2. Exception record created with status PROPOSED
3. Owner evaluates: impact, exploitability, mitigation path
4. If accepted: status -> ACTIVE, evidence and rationale recorded
5. If mitigated: status -> MITIGATED, resolution recorded
6. If resolved: status -> RESOLVED, evidence references recorded
7. If superseded or no longer relevant: status -> ARCHIVED
8. Change History entry added for every status transition
```

**Approval authority:** The repository owner (Cartilage-Stairwells) is the
sole approval authority for exception status transitions.

---

## 9. Change History

| Date         | Change                                                    |
|--------------|-----------------------------------------------------------|
| 2026-07-19   | Schema v1.0 established. EX-0001 created as RESOLVED.     |
| 2026-07-19   | EX-0002 created as ACTIVE (crossbeam-epoch, transitive).  |
| 2026-07-19   | EX-0003 created as ACTIVE (wasmtime, 6 advisories).       |
| 2026-07-19   | allOf patch: ACTIVE status now requires audit field.     |
| 2026-07-19   | Provenance fields added: first_recorded, last_reviewed.   |
| 2026-07-19   | Machine-readable registry: schemas/exceptions.json.       |
| 2026-07-19   | Validator: scripts/validate_governance.py.                |
| 2026-07-19   | CI enforcement: .github/workflows/supply-chain-governance.yml. |

---

## Audit Configuration

The `audit.toml` in the workspace root configures `cargo audit`.
Active exceptions (EX-0002, EX-0003) are listed in `audit.toml [advisories].ignore`
with comments referencing their exception IDs. Resolved exceptions (EX-0001)
are not listed — serde_cbor is no longer in the dependency tree.

The validator (`scripts/validate_governance.py`) enforces bidirectional
consistency: every ACTIVE exception's audit advisory must have a
corresponding `audit.toml` ignore entry, and every `audit.toml` ignore
entry must have a corresponding ACTIVE exception. Orphan ignores are
rejected.
