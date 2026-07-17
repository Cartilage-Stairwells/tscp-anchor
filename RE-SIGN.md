# Re-Sign Procedure for v1.0-rc1

The release candidate commits were authored in an agent sandbox without access to the GPG private key. They must be re-signed from the holder's device before the v1.0 release.

## Affected Commits

| Repository | Commit | Branch |
|---|---|---|
| Cartilage-Stairwells/tscp-anchor | `1cff176` | master |
| Cartilage-Stairwells/avx512-butterfly | `288d58a` | master |

## Procedure (tscp-anchor)

```bash
# 1. Pull latest master
cd tscp-anchor
git fetch cartilage master
git checkout master
git pull cartilage master

# 2. Re-sign the top commit (the serialization conformance suite)
git rebase --exec 'git commit --amend --no-edit -S' cartilage/master~1

# 3. Verify all commits are signed
git log --show-signature -3

# 4. Force-push the signed history
git push cartilage master --force

# 5. Verify on GitHub that all commits show "Verified"
```

## Procedure (avx512-butterfly)

```bash
# 1. Pull latest master
cd avx512-butterfly
git fetch origin master
git checkout master
git pull origin master

# 2. Re-sign the top commit (the compile fix + IEP runner + serialization fixture)
git rebase --exec 'git commit --amend --no-edit -S' origin/master~1

# 3. Verify all commits are signed
git log --show-signature -3

# 4. Force-push the signed history
git push origin master --force

# 5. Verify on GitHub that all commits show "Verified"
```

## After Re-Signing

1. Tag both repos:
   ```bash
   # tscp-anchor
   git tag -s v1.0-rc1 -m "TSCP v1.0-rc1 — Release Candidate 1

Evidence corpus: 97/97 tests pass
Manifest: RELEASE_MANIFEST_v1.0-rc1.md
GPG key: E747C3AF22573539"
   git push cartilage v1.0-rc1

   # avx512-butterfly
   git tag -s v1.0-rc1 -m "TSCP v1.0-rc1 — Release Candidate 1

IEP v0.1 frozen, 85/85 tests pass
Evidence artifact: firebird_reference_80dc195.json
GPG key: E747C3AF22573539"
   git push origin v1.0-rc1
   ```

2. Publish GitHub releases on both repos with the manifest content.

3. Send team notification to `redshift.prover@gmail.com`.

## Verification

After re-signing, verify on GitHub:
- All commits on master show ✅ Verified
- Tags `v1.0-rc1` exist on both repos
- Tags are signed and verified
