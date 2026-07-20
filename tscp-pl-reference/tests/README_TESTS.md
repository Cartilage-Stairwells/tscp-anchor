# TSCP-PL verifier test vectors

## Purpose

Create deterministic test vectors to exercise the verifier and prove the verifier behavior:
- same JSON content (different key order) → same canonical digest → PASS
- content mutation → different digest → FAIL (payload integrity check detects stale recorded digest)
- missing artifact → FAIL (artifact digest verification detects missing file)
- bad artifact digest → FAIL (artifact digest verification detects mismatch)

## Architecture

The verifier is the reference implementation of the TSCP-PL specification. CI validates the implementation against invariant behavior rather than merely executing a script.

### Seven-stage verification pipeline

| Stage | Name | Purpose |
|---|---|---|
| 1 | Canonicalization | Deterministic JSON canonical bytes |
| 2 | Schema validation | Structure conforms to TSCP-PL schema |
| 3 | Artifact digest verification | Referenced files exist, digests match |
| 4 | Payload integrity | Recorded digest matches computed (mutation detection) |
| 5 | Evidence manifest generation | Machine-readable verification record |
| 6 | Payload export | Canonical bytes written to disk for signing |
| 7 | Manifest self-validation | Reload manifest, validate against its own schema |

### Content identity

Each ledger has a `verification.checks` entry recording the SHA256 of its payload (everything except the `verification` block). This creates a self-referential integrity check:

- `good.jsonld` — recorded digest matches computed → PASS
- `mutated.jsonld` — content changed but recorded digest is stale → FAIL
- `missingartifact.jsonld` — artifact file doesn't exist → FAIL (stage 3)
- `bad_hash.jsonld` — artifact digest doesn't match file → FAIL (stage 3)

## Quick start

From repository root:

```bash
# Ensure dependencies
python3 -m pip install --user jsonschema

# Generate artifacts and ledgers, run all regression stages
cd tscp-pl-reference
make -C tests all
```

## Regression stages

### Stage 1: Expected exit code verification

| Test vector | Expected exit | Description |
|---|---|---|
| `good.jsonld` | 0 | Baseline valid custody object |
| `reorderkeys.jsonld` | 0 | Canonicalization stability (key order doesn't matter) |
| `mutated.jsonld` | 3 | Content identity sensitivity (stale recorded digest) |
| `missingartifact.jsonld` | 3 | Missing evidence rejection |
| `bad_hash.jsonld` | 3 | Artifact integrity rejection (digest mismatch) |

### Stage 2: Canonicalization invariance

`good.jsonld` and `reorderkeys.jsonld` must produce identical canonical digests despite different key ordering. This proves the canonicalization is deterministic and key-order-independent.

### Stage 3: Determinism regression

`good.jsonld` is verified twice. The manifests are compared: canonical digest, payload digest, and artifact digests must all be identical across runs. The timestamp can differ; everything else must remain identical.

## Expected output

```
=== Stage 1: Expected exit code verification ===
--- PASS cases (expect exit 0) ---
  good.jsonld                    expected=0 actual=0  PASS
  reorderkeys.jsonld             expected=0 actual=0  PASS
--- EXPECTED FAIL cases (expect exit 3) ---
  mutated.jsonld                 expected=3 actual=3  PASS
  missingartifact.jsonld          expected=3 actual=3  PASS
  bad_hash.jsonld                expected=3 actual=3  PASS
  All test vectors produced expected exit codes ✅

=== Stage 2: Canonicalization invariance ===
  good digest:         e1065f064bf1798a...
  reorderkeys digest: e1065f064bf1798a...
  Canonicalization invariance: PASS ✅ (digests identical)

=== Stage 3: Determinism regression ===
  Run 1 canonical: e1065f064bf1798a...
  Run 2 canonical: e1065f064bf1798a...
  Run 1 payload:   b80b5484bc7948a0...
  Run 2 payload:   b80b5484bc7948a0...
  Artifact digests match: True
  Determinism: PASS ✅ (all digests identical across runs)

=== All regression stages passed ===
```

## CI artifacts

The verifier produces:
- `evidence/manifest.json` — machine-readable verification record
- `tscp-pl.payload` — canonical payload bytes (for signing)
- `evidence/verifier.log` — full verifier output
- `evidence/payload.sha256` — SHA256 of canonical payload
- `evidence/summary.json` — test run summary
- `evidence/verifier-version.txt` — verifier version info

These artifacts become inputs to the broader TSCP evidence bundle.
