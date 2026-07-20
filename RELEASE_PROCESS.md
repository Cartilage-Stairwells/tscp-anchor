# Release Process

## Overview

This document defines the release process for the TSCP integration
layer (`tscp-anchor`). It covers versioning, signing, CI verification,
and artifact publication.

## Versioning

TSCP follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

| Version component | Meaning |
|-------------------|---------|
| Major | Serialization contract change (receipt hashes invalidated) |
| Minor | New verification features, new acceptance packages |
| Patch | Bug fixes, dependency updates, governance changes |

Release candidates use the `-rcN` suffix (e.g., `v1.0-rc1`).

## Pre-Release Checklist

Before tagging a release:

1. **CI must be green** — all workflows on `master`:
   - TSCP Verification Predicate CI
   - TSCP Anchor Pipeline
   - Verify Receipts
   - WASM Smoke Test (JIT)
   - Supply Chain Governance

2. **cargo audit clean** — no unignored advisories:
   ```bash
   cargo audit
   ```
   Active exceptions in `schemas/exceptions.json` must be reviewed.
   Any advisory past its `next_review` date must be escalated.

3. **Serialization conformance** — 12/12 tests pass:
   ```bash
   cargo test -p tscp-kernel
   ```

4. **Supply chain governance** — validator passes:
   ```bash
   python3 scripts/validate_governance.py
   ```

5. **Working tree clean** — no uncommitted changes:
   ```bash
   git status --porcelain
   ```

## Signing Protocol

### GPG Configuration

```bash
git config user.signingkey E747C3AF22573539
git config commit.gpgsign true
git config tag.gpgsign true
```

### Tagging a Release

1. **Verify the commit is signed:**
   ```bash
   git log --show-signature -1 HEAD
   # Must show: Good signature from "Sean Christopher Southwick"
   ```

2. **Delete any existing unsigned placeholder tags** (if present):
   ```bash
   git tag -d <tag-name>  # delete locally
   git push origin :refs/tags/<tag-name>  # delete remote
   ```

3. **Create a signed annotated tag:**
   ```bash
   git tag -s -a v1.0.0 -m "TSCP v1.0.0 — release

   Summary of changes:
   - ...
   "
   ```

4. **Push the tag:**
   ```bash
   git push origin v1.0.0
   ```

5. **Verify on GitHub:**
   ```bash
   curl -s -H "Authorization: token $TOKEN" \
     "https://api.github.com/repos/Cartilage-Stairwells/tscp-anchor/git/refs/tags/v1.0.0"
   ```
   The tag must show `verified=True` on GitHub.

### Tag Naming Convention

| Pattern | Purpose |
|---------|---------|
| `v{major}.{minor}.{patch}` | Semantic version release |
| `v{major}.{minor}.{patch}-rc{N}` | Release candidate |
| `tscp-{feature}-v{version}` | Feature-specific seal |
| `tscp-mini-ntt-parity-v1` | Acceptance package seal |

## CI Verification

After pushing a tag, GitHub Actions triggers all workflow runs on the
tagged commit. Monitor:

```
https://github.com/Cartilage-Stairwells/tscp-anchor/actions
```

All workflows must complete with `success`. If any workflow fails:

1. Do NOT publish a GitHub Release.
2. Diagnose the failure.
3. Fix on a feature branch, push, verify CI passes.
4. Re-tag (delete the failed tag first, then create a new signed tag).

## GitHub Release Publication

Once CI is green on the tagged commit:

1. Navigate to Releases → New release
2. Select the signed tag
3. Title: `TSCP v1.0.0`
4. Body includes:
   - Summary of changes
   - Cryptographic anchors (GPG fingerprint, commit SHA, tag SHA)
   - CI verification summary
   - Verification instructions for consumers
5. Attach artifacts (acceptance package tarballs, SHA256SUMS, evidence manifests)
6. Publish

## Acceptance Package Sealing

For acceptance packages (separate from version releases):

1. Run the verification package locally (24/24 PASS)
2. Generate SHA256SUMS
3. Create a signed tag: `git tag -s -a tscp-{package-name} -m "..."`
4. Push the tag
5. Record in `ARCHIVE_INDEX.md`

## Supply Chain Governance Review

Before each release, review the exception registry:

```bash
python3 scripts/validate_governance.py
```

Check:
- No ACTIVE exception is past its `next_review` date
- No new advisories exist outside the registry
- `audit.toml` ignore entries match the registry bidirectionally

## Post-Release

1. Update `CHANGELOG.md` with the release entry
2. Update `ARCHIVE_INDEX.md` with the new tag
3. Notify stakeholders
4. Update this document if the process changed

## Provenance Chain

```
Source commit (GPG-signed)
    ↓
CI verification (all workflows green)
    ↓
Signed tag (annotated, GPG-signed)
    ↓
GitHub Release (with cryptographic anchors)
    ↓
Stakeholder notification
```

Each link is independently verifiable: reproducible, tamper-evident,
and non-repudiable.
