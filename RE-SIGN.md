# Re-Sign Procedure for v1.0-rc1

The release candidate commits were authored in an agent sandbox without access to the GPG private key. They must be re-signed from the holder's device before the v1.0 release.

**Order matters.** The `--amend -S` re-sign creates new commit SHAs (the signature changes the commit object). The placeholder tags currently on GitHub point at the old unsigned commits. You must re-sign first, force-push, then delete and recreate the tags pointing at the new signed commits.

```
re-sign → force-push → delete old tags → create signed tags → push tags → publish releases
```

## Affected Commits

| Repository | Commit | Branch |
|---|---|---|
| Cartilage-Stairwells/tscp-anchor | `1cff176` | master |
| Cartilage-Stairwells/avx512-butterfly | `288d58a` | master |

## Placeholder Tags to Replace

| Repository | Tag | Points at (unsigned) |
|---|---|---|
| Cartilage-Stairwells/tscp-anchor | `v1.0-rc1` | `4b48f2e` |
| Cartilage-Stairwells/avx512-butterfly | `v1.0-rc1` | `d5d0a00` |

These annotated-but-unsigned tags must be deleted and recreated with `-s` after the commits are re-signed.

---

## Procedure (tscp-anchor)

```bash
# ── Phase 1: Re-sign commits ─────────────────────────────────────────────────

cd tscp-anchor
git fetch cartilage master
git checkout master
git pull cartilage master

# Re-sign the top 2 commits (release manifest + serialization conformance)
git rebase --exec 'git commit --amend --no-edit -S' cartilage/master~2

# Verify all commits are signed
git log --show-signature -5

# Force-push the signed history
git push cartilage master --force

# Verify on GitHub that all commits show "Verified"
# (check: https://github.com/Cartilage-Stairwells/tscp-anchor/commits/master)

# ── Phase 2: Replace tags ────────────────────────────────────────────────────

# Delete the old unsigned tag (local + remote)
git tag -d v1.0-rc1
git push cartilage :refs/tags/v1.0-rc1

# Create a signed tag pointing at the new signed commit
git tag -s v1.0-rc1 -m "TSCP v1.0-rc1 — Release Candidate 1

Evidence corpus: 97/97 tests pass
Manifest: RELEASE_MANIFEST_v1.0-rc1.md
GPG key: E747C3AF22573539"

# Push the signed tag
git push cartilage v1.0-rc1
```

## Procedure (avx512-butterfly)

```bash
# ── Phase 1: Re-sign commits ─────────────────────────────────────────────────

cd avx512-butterfly
git fetch origin master
git checkout master
git pull origin master

# Re-sign the top 2 commits (SHA256SUMS + compile fix/IEP runner/serialization fixture)
git rebase --exec 'git commit --amend --no-edit -S' origin/master~2

# Verify all commits are signed
git log --show-signature -5

# Force-push the signed history
git push origin master --force

# Verify on GitHub that all commits show "Verified"
# (check: https://github.com/Cartilage-Stairwells/avx512-butterfly/commits/master)

# ── Phase 2: Replace tags ────────────────────────────────────────────────────

# Delete the old unsigned tag (local + remote)
git tag -d v1.0-rc1
git push origin :refs/tags/v1.0-rc1

# Create a signed tag pointing at the new signed commit
git tag -s v1.0-rc1 -m "TSCP v1.0-rc1 — Release Candidate 1

IEP v0.1 frozen, 85/85 tests pass
Evidence artifact: firebird_reference_80dc195.json
GPG key: E747C3AF22573539"

# Push the signed tag
git push origin v1.0-rc1
```

---

## After Both Repos Are Signed and Tagged

1. Publish GitHub releases on both repos:
   - Use the tag `v1.0-rc1`
   - Title: `TSCP v1.0-rc1 — Release Candidate 1`
   - Body: paste the contents of `RELEASE_MANIFEST_v1.0-rc1.md`

2. Send team notification to `redshift.prover@gmail.com` with:
   - Release candidate status
   - Link to both GitHub releases
   - Remaining v1.0 blockers (AVX-512 hardware benchmark)

---

## Verification Checklist

After everything is done, verify on GitHub:

- [ ] tscp-anchor: all commits on master show ✅ Verified
- [ ] tscp-anchor: tag `v1.0-rc1` is signed (shows 🔒 Verified)
- [ ] avx512-butterfly: all commits on master show ✅ Verified
- [ ] avx512-butterfly: tag `v1.0-rc1` is signed (shows 🔒 Verified)
- [ ] GitHub releases published on both repos
- [ ] Team notification sent

## Common Issues

**If `git rebase --exec` re-signs too many commits:**
Use `HEAD~N` with a smaller N, or re-sign only the specific commit:
```bash
git commit --amend --no-edit -S
git rebase --exec 'git commit --amend --no-edit -S' HEAD~1
```

**If the tag deletion fails remotely:**
```bash
git push cartilage --delete v1.0-rc1   # alternative syntax
```

**If GitHub doesn't show commits as Verified after force-push:**
Ensure your git config email matches a verified email on GitHub:
```bash
git config user.email
# Should be: adamantinespine@gmail.com (or schlagetorren@gmail.com)
```
