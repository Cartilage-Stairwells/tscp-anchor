# Repository Custody Transition Addendum
# Triune-Oracle / Cartilage-Stairwells

**Version:** 1.0  
**Date:** 2026-07-14  
**Status:** ACTIVE — migration in progress

---

## Identity Model

| Organization | Role | Purpose |
|---|---|---|
| **Triune-Oracle** | Canonical production identity | Active protocol, verification, tooling, deployments |
| **Cartilage-Stairwells** | Historical provenance / archive | Experiments, compliance anchors, research origins |

This separation preserves history without forcing historical experiments to present as production assets. Neither account supersedes the other — they serve different functions in the same custody chain.

---

## Phase 0 — Preservation Snapshot (REQUIRED before any destructive action)

Before any history rewrite or force-push, the following must exist:

- [ ] Full repository manifest captured (`pre_cleanup_manifest.json`)
- [ ] All remote tags recorded (see manifest)
- [ ] Current HEAD commits recorded per repo
- [ ] Local rollback tag created: `pre-history-cleanup-YYYYMMDD-HHMM`
- [ ] Remote rollback tag pushed and verified: `pre-history-cleanup-YYYY-MM-DD`
- [ ] Working tree confirmed clean (`git status --porcelain` = empty)

**Completed 2026-07-14:**
- `pre_cleanup_manifest.json` captured — 4 repos, 34+30+4 tags recorded
- `tscp-serialization-v0.1-dossier-2026-07-14` tag pushed and verified on both tscp-anchor instances
- `pre-history-cleanup-20260714-1955` local tag created on T-O/tscp-anchor
- Working tree clean on fresh clone confirmed

---

## Cleanup Sequence (per repository)

The required ordering is non-negotiable. Each step must complete before the next begins.

```
1. create annotated tag
2. push tag to remote
3. verify tag exists remotely (git fetch --tags + git rev-parse refs/tags/...)
4. git filter-repo (remove target/, node_modules/, credential files)
5. git gc --aggressive --prune=now
6. verify zero residual references
7. generate cleanup report (retain as auditable record)
8. git push --force-with-lease origin <branch>
9. verify remote HEAD matches expected post-cleanup SHA
```

**Never skip step 3.** The remote tag is the rollback anchor. A force-push without a confirmed remote anchor leaves no safe recovery path.

---

## Credential Policy

No credentials, private keys, API tokens, wallet secrets, or production access material may exist in:
- Repository history (any commit, any branch)
- Active branches or working tree
- Tags or annotated tag messages
- Build artifacts committed to version control
- CI workflow files (use repository secrets)

**Current status:**
- `CulturalCodex` — `.env` with revoked Polygon key + Infura key. Keys dead. History scrub pending.
- `pinata-api-keys` — credentials rotated. History scrub + deletion pending.
- All other repos — audited clean as of 2026-07-14.

---

## Naming Convention

Canonical production repositories live under `Triune-Oracle/`. Naming follows technical meaning — not all repositories need the `triune-` prefix:

| Prefix | Use |
|---|---|
| `tscp-*` | Protocol layer (serialization, canon, anchor, verification) |
| `triune-*` | Infrastructure and orchestration layer |
| `avx512-*` / technical names | Compute kernels where the name already carries technical meaning |

Avoid forcing uniform prefixes onto repositories where the technical name is already the canonical identifier.

---

## Migration Record Format

Every migrated repository must produce a migration record:

```
Migration Record
================
Repository:          <name>
Previous location:   <owner/repo>
New location:        <owner/repo>
Migration date:      YYYY-MM-DD
Commit base (pre):   <sha>
Commit base (post):  <sha>
Tags preserved:      <list>
Cleanup performed:   <yes/no — describe if yes>
Cleanup report:      <filename>
CI status post-push: <pass/fail/pending>
Verification:        <pass/fail/pending>
```

---

## Canonicalization Completion Gate

**CANONICALIZATION IS COMPLETE iff ALL of the following are true:**

| Gate | Condition | Status |
|---|---|---|
| 1. Inventory | `ARCHIVE_INDEX.md` exists, all repos classified | ✅ COMPLETE |
| 2. Secrets | All credentials audited; none in history | 🔄 IN PROGRESS (CulturalCodex, pinata-api-keys pending scrub) |
| 3. Canonical repos identified | Primary repos per cluster identified | ✅ COMPLETE |
| 4. Migration records | Each migrated repo has a migration record | 🔄 IN PROGRESS |
| 5. Rollback path | Pre-cleanup tags exist remotely for all modified repos | 🔄 IN PROGRESS (tscp-anchor tagged; tscp-canon pending) |
| 6. CI passes | All TSCP cluster repos have passing CI | 🔄 IN PROGRESS (CI deployed; awaiting first passing run) |
| 7. Remote state verified | Post-cleanup HEAD and tags confirmed on remote | ⏳ PENDING cleanup execution |

Current overall status: **IN PROGRESS**

---

## Repository Disposition Table

| Repo | From | To | Action | Gate |
|---|---|---|---|---|
| tscp-anchor | C-S (canonical) | T-O | History cleanup + sync | History rewrite pending |
| tscp-anchor | T-O | — | History cleanup | History rewrite pending |
| tscp-canon | T-O | — | History cleanup | History rewrite pending |
| avx512-butterfly | C-S | T-O | Migrate | Post-cleanup |
| tscp-pl-phase1 | C-S | — | ARCHIVE (compliance anchor) | No action — read-only |
| CulturalCodex | T-O | — | History scrub .env | Pending |
| pinata-api-keys | T-O | — | History scrub + DELETE | Pending |
| triune-swarm-engine | T-O | — | Migrate to canonical cluster | Post-cleanup |
| toolintell | T-O | triune-tools | Rename/migrate | Post-cleanup |
| Adamantine-Spine | T-O | — | ARCHIVE | Low priority |
| ~30 inactive repos | T-O | — | ARCHIVE or DELETE | Low priority |

---

## Rollback Procedure

If a force-push must be undone:

```bash
# On any machine with the pre-cleanup tag
git clone git@github.com:<owner>/<repo>.git <repo>-rollback
cd <repo>-rollback
git checkout pre-history-cleanup-<DATE>
git push --force-with-lease origin master
```

This restores the remote to the exact pre-rewrite state. Only works if the remote tag was confirmed before the rewrite (Phase 0 requirement).

---

*Maintained by Triune-Oracle*  
*Last updated: 2026-07-14*
