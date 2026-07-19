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
| First Recorded   | Date the exception was first noted                   |
| Last Reviewed    | Date of most recent review                          |

### Optional Fields

| Field                  | Description                                      |
|------------------------|--------------------------------------------------|
| Associated Advisories  | RUSTSEC IDs or other advisory database references |
| Planned Mitigations    | Scheduled actions with target dates             |
| Next Review            | Date for next mandatory review                  |
| Notes                  | Free-form context                                |

### Status Lifecycle

```
PROPOSED → ACTIVE → MITIGATED → RESOLVED → ARCHIVED
                ↑                              |
                └──────── (reopened if regression detected)
```

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
| First Recorded   | 2026-07-19                                         |
| Last Reviewed    | 2026-07-19                                         |
| Planned Mitigations | Upgrade wasmtime to >=36.0.7 (API migration required) |
| Next Review      | 2026-08-19                                         |

---

## 6. Automated Drift Detection

**Governance requirement:** Dependency drift must be detected and reviewed.

**Repository policy (enforcement):** The `Supply Chain (cargo audit)` CI job
in `.github/workflows/ci.yml` runs `cargo audit` on every push and pull request.
The job is `continue-on-error: true` — advisories are surfaced but do not
block the verification seal. New advisories require a registry entry
(PROPOSED status) before the next release.

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

---

## Audit Configuration

The `audit.toml` in the workspace root configures `cargo audit`.
With EX-0001 resolved, no ignore entries are needed for serde_cbor.
EX-0002 and EX-0003 are tracked here but not suppressed in `audit.toml`
— they surface as CI warnings, which is the intended governance signal.

The `Supply Chain (cargo audit)` CI job will report `failure` on these
advisories, but `continue-on-error: true` prevents blocking the
verification seal. This is the intended state: advisories are visible,
tracked, and non-blocking until their planned mitigation is executed.
