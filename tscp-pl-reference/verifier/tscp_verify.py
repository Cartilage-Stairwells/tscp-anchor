#!/usr/bin/env python3
"""
TSCP-PL Reference Verifier v1.0

Reference implementation of the TSCP Provenance Ledger verification protocol.

Seven-stage verification pipeline:
  1. Canonicalization — deterministic JSON canonical bytes
  2. Schema validation — structure conforms to TSCP-PL schema
  3. Artifact digest verification — referenced files exist, digests match
  4. Payload integrity — recorded digest matches computed digest (mutation detection)
  5. Evidence manifest generation — machine-readable verification record
  6. Payload export — canonical bytes written to disk for signing
  7. Manifest self-validation — reload manifest, validate against its own schema

Exit codes:
  0 = PASS (all 7 stages passed)
  1 = argument/usage error
  2 = file not found / IO error
  3 = verification failure (digest mismatch, missing artifact, schema invalid, etc.)
"""

import argparse
import hashlib
import json
import os
import sys
from datetime import datetime

# ─── Constants ──────────────────────────────────────────────────────────────

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.abspath(os.path.join(SCRIPT_DIR, ".."))
EVIDENCE_DIR = os.path.join(ROOT_DIR, "evidence")
MANIFEST_FILE = os.path.join(EVIDENCE_DIR, "manifest.json")
PAYLOAD_FILE = os.path.join(ROOT_DIR, "tscp-pl.payload")

EXIT_OK = 0
EXIT_USAGE = 1
EXIT_IO = 2
EXIT_VERIFY = 3


# ─── Canonicalization helpers ───────────────────────────────────────────────

def _sort(obj):
    """Recursively sort all dict keys for deterministic serialization."""
    if isinstance(obj, dict):
        return {k: _sort(obj[k]) for k in sorted(obj.keys())}
    if isinstance(obj, list):
        return [_sort(item) for item in obj]
    return obj


def stable_serialize(obj):
    """Serialize to canonical JSON bytes: sorted keys, no whitespace, UTF-8."""
    sorted_obj = _sort(obj)
    return json.dumps(sorted_obj, separators=(",", ":"), ensure_ascii=False).encode("utf-8")


def canonical_bytes(obj):
    """Return canonical bytes of a JSON object."""
    return stable_serialize(obj)


def sha256_bytes(data):
    """SHA256 of raw bytes, returned as hex string."""
    return hashlib.sha256(data).hexdigest()


def sha256_file(path):
    """SHA256 of a file, returned as hex string."""
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


def load_json(path):
    """Load JSON from file, return parsed object."""
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def write_json(path, obj):
    """Write JSON to file with indent=2."""
    with open(path, "w", encoding="utf-8") as f:
        json.dump(obj, f, indent=2, ensure_ascii=False)


def write_payload(data, path):
    """Write raw bytes to a file."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "wb") as f:
        f.write(data)


def payload_digest(ledger):
    """
    Compute the canonical digest of the ledger's payload — everything
    EXCEPT the verification block. This is the content identity.
    """
    payload = {k: v for k, v in ledger.items() if k != "verification"}
    return sha256_bytes(canonical_bytes(payload))


# ─── Schema (minimal TSCP-PL v1.0) ─────────────────────────────────────────

REQUIRED_TOP_LEVEL = [
    "@context", "@type", "id", "ledger",
    "conversation", "artifacts", "verification", "provenance"
]

REQUIRED_LEDGER = ["format", "version"]
REQUIRED_ARTIFACT = ["id", "type", "path", "digest", "scope"]
REQUIRED_VERIFICATION = ["status", "checks"]


# ─── Stage 1: Canonicalization ──────────────────────────────────────────────

def stage_canonicalize(tscp_pl):
    """Stage 1: Produce deterministic canonical bytes from the ledger."""
    cbytes = canonical_bytes(tscp_pl)
    digest = sha256_bytes(cbytes)
    return {
        "status": "PASS",
        "canonical_digest": digest,
        "canonical_size_bytes": len(cbytes),
    }


# ─── Stage 2: Schema Validation ─────────────────────────────────────────────

def stage_schema_validate(tscp_pl):
    """Stage 2: Validate the ledger structure against TSCP-PL v1.0 schema."""
    errors = []

    for key in REQUIRED_TOP_LEVEL:
        if key not in tscp_pl:
            errors.append(f"missing_top_level_key:{key}")

    ledger = tscp_pl.get("ledger", {})
    for key in REQUIRED_LEDGER:
        if key not in ledger:
            errors.append(f"missing_ledger_key:{key}")

    artifacts = tscp_pl.get("artifacts", [])
    for i, art in enumerate(artifacts):
        for key in REQUIRED_ARTIFACT:
            if key not in art:
                errors.append(f"artifact[{i}]_missing_key:{key}")

    verification = tscp_pl.get("verification", {})
    for key in REQUIRED_VERIFICATION:
        if key not in verification:
            errors.append(f"missing_verification_key:{key}")

    if errors:
        return {"status": "FAIL", "errors": errors}

    return {"status": "PASS", "errors": []}


# ─── Stage 3: Artifact Digest Verification ──────────────────────────────────

def verify_artifacts(tscp_pl, root_dir):
    """Stage 3: Verify all referenced artifacts exist and digests match."""
    artifacts_info = []
    all_ok = True

    for art in tscp_pl.get("artifacts", []):
        art_id = art.get("id", "unknown")
        art_path = art.get("path", "")
        expected_digest = art.get("digest", "")
        scope = art.get("scope", "package-relative")

        # Resolve path relative to repo root
        if scope == "package-relative":
            resolved = os.path.join(root_dir, art_path)
        else:
            resolved = art_path

        info = {
            "id": art_id,
            "path": art_path,
            "expected_digest": expected_digest,
            "scope": scope,
        }

        if not os.path.exists(resolved):
            info["status"] = "MISSING"
            info["resolved_path"] = resolved
            all_ok = False
            artifacts_info.append(info)
            continue

        actual_digest_raw = sha256_file(resolved)
        expected_clean = expected_digest.replace("sha256:", "")
        actual_digest = f"sha256:{actual_digest_raw}"

        if actual_digest == expected_digest or actual_digest_raw == expected_clean:
            info["status"] = "PASS"
            info["actual_digest"] = actual_digest
        else:
            info["status"] = "DIGEST_MISMATCH"
            info["actual_digest"] = actual_digest
            all_ok = False

        artifacts_info.append(info)

    return {
        "status": "PASS" if all_ok else "FAIL",
        "artifacts": artifacts_info,
    }


# ─── Stage 4: Payload Integrity (Mutation Detection) ────────────────────────

def stage_payload_integrity(tscp_pl):
    """
    Stage 4: Verify the recorded payload digest matches the computed digest.

    The verification.checks array should contain a 'payload_digest' entry
    with a 'recorded_digest' field. If the content has been mutated since
    the digest was recorded, the computed digest won't match → FAIL.

    This is the content identity sensitivity check: any mutation to the
    conversation, artifacts, ledger, or other payload fields is detected.
    """
    computed_digest = payload_digest(tscp_pl)

    # Find the recorded digest in verification.checks
    checks = tscp_pl.get("verification", {}).get("checks", [])
    recorded_digest = None

    for check in checks:
        if check.get("name") == "payload_digest":
            recorded_digest = check.get("recorded_digest")
            break

    if recorded_digest is None:
        # No recorded digest — can't verify integrity
        return {
            "status": "FAIL",
            "reason": "no_recorded_payload_digest",
            "computed_digest": computed_digest,
        }

    if computed_digest == recorded_digest:
        return {
            "status": "PASS",
            "computed_digest": computed_digest,
            "recorded_digest": recorded_digest,
        }
    else:
        return {
            "status": "FAIL",
            "reason": "payload_digest_mismatch",
            "computed_digest": computed_digest,
            "recorded_digest": recorded_digest,
        }


# ─── Stage 5: Evidence Manifest Generation ─────────────────────────────────

def generate_manifest(tscp_pl, canonical_result, schema_result,
                      artifact_result, integrity_result, ledger_path):
    """Stage 5: Generate the machine-readable evidence manifest."""
    manifest = {
        "schema_version": "1.0",
        "generated_at": datetime.utcnow().isoformat() + "Z",
        "ledger_path": os.path.relpath(ledger_path, ROOT_DIR),
        "ledger_id": tscp_pl.get("id", "unknown"),
        "stages": {
            "canonicalization": {
                "status": canonical_result["status"],
                "canonical_digest": canonical_result["canonical_digest"],
                "canonical_size_bytes": canonical_result["canonical_size_bytes"],
            },
            "schema_validation": {
                "status": schema_result["status"],
                "errors": schema_result["errors"],
            },
            "artifact_digests": {
                "status": artifact_result["status"],
                "artifacts": artifact_result["artifacts"],
            },
            "payload_integrity": {
                "status": integrity_result["status"],
                "computed_digest": integrity_result.get("computed_digest", ""),
                "recorded_digest": integrity_result.get("recorded_digest", ""),
                "reason": integrity_result.get("reason"),
            },
        },
        "overall_status": "PASS",
    }

    all_stages = [
        canonical_result["status"],
        schema_result["status"],
        artifact_result["status"],
        integrity_result["status"],
    ]
    if any(s == "FAIL" for s in all_stages):
        manifest["overall_status"] = "FAIL"

    return manifest


# ─── Stage 6: Payload Export ────────────────────────────────────────────────

def stage_export_payload(canonical_data, manifest, payload_path):
    """Stage 6: Write canonical bytes and manifest to disk."""
    os.makedirs(EVIDENCE_DIR, exist_ok=True)
    write_payload(canonical_data, payload_path)
    write_json(MANIFEST_FILE, manifest)

    return {
        "status": "PASS",
        "payload_path": payload_path,
        "manifest_path": MANIFEST_FILE,
        "payload_size_bytes": len(canonical_data),
    }


# ─── Stage 7: Manifest Self-Validation ──────────────────────────────────────

def stage_manifest_self_validate():
    """
    Stage 7: Reload the manifest from disk and validate it against its own schema.

    This prevents a corrupted evidence writer from producing malformed evidence
    while still returning success.
    """
    if not os.path.exists(MANIFEST_FILE):
        return {"status": "FAIL", "reason": "manifest_file_not_found"}

    try:
        manifest = load_json(MANIFEST_FILE)
    except json.JSONDecodeError as e:
        return {"status": "FAIL", "reason": "manifest_invalid_json", "error": str(e)}

    errors = []

    required_manifest_keys = [
        "schema_version", "generated_at", "ledger_path",
        "ledger_id", "stages", "overall_status"
    ]
    for key in required_manifest_keys:
        if key not in manifest:
            errors.append(f"manifest_missing_key:{key}")

    stages = manifest.get("stages", {})
    required_stages = [
        "canonicalization", "schema_validation",
        "artifact_digests", "payload_integrity"
    ]
    for stage_name in required_stages:
        if stage_name not in stages:
            errors.append(f"manifest_missing_stage:{stage_name}")
        else:
            if "status" not in stages[stage_name]:
                errors.append(f"manifest_stage_{stage_name}_missing_status")

    # Verify payload file exists and digest matches
    if os.path.exists(PAYLOAD_FILE):
        with open(PAYLOAD_FILE, "rb") as f:
            payload_data = f.read()
        actual_digest = sha256_bytes(payload_data)
        expected_digest = stages.get("canonicalization", {}).get("canonical_digest", "")
        if expected_digest and actual_digest != expected_digest:
            errors.append("payload_digest_mismatch_on_reload")
    else:
        errors.append("payload_file_not_found_on_reload")

    if errors:
        return {"status": "FAIL", "reason": "manifest_self_validation_failed", "errors": errors}

    return {"status": "PASS", "manifest_keys_verified": len(required_manifest_keys)}


# ─── Main Verification Pipeline ─────────────────────────────────────────────

def verify(ledger_path, quiet=False):
    """
    Run the full 7-stage verification pipeline on a TSCP-PL ledger.

    Returns exit code: 0 = PASS, 3 = FAIL.
    """
    try:
        tscp_pl = load_json(ledger_path)
    except FileNotFoundError:
        if not quiet:
            print(f"ERROR: Ledger file not found: {ledger_path}", file=sys.stderr)
        return EXIT_IO
    except json.JSONDecodeError as e:
        if not quiet:
            print(f"ERROR: Invalid JSON in ledger: {e}", file=sys.stderr)
        return EXIT_VERIFY

    # Stage 1: Canonicalization
    canonical_result = stage_canonicalize(tscp_pl)
    canonical_data = canonical_bytes(tscp_pl)
    if not quiet:
        print(f"[1/7] Canonicalization: {canonical_result['status']} "
              f"(digest={canonical_result['canonical_digest'][:16]}...)")

    # Stage 2: Schema Validation
    schema_result = stage_schema_validate(tscp_pl)
    if not quiet:
        print(f"[2/7] Schema validation: {schema_result['status']}")
        if schema_result["errors"]:
            for err in schema_result["errors"]:
                print(f"       ERROR: {err}")

    # Stage 3: Artifact Digest Verification
    artifact_result = verify_artifacts(tscp_pl, ROOT_DIR)
    if not quiet:
        print(f"[3/7] Artifact digests: {artifact_result['status']}")
        for art in artifact_result["artifacts"]:
            mark = "✅" if art["status"] == "PASS" else "❌"
            print(f"       {mark} {art['id']}: {art['status']}")

    # Stage 4: Payload Integrity (Mutation Detection)
    integrity_result = stage_payload_integrity(tscp_pl)
    if not quiet:
        print(f"[4/7] Payload integrity: {integrity_result['status']}")
        if integrity_result["status"] == "FAIL":
            print(f"       reason: {integrity_result.get('reason', 'unknown')}")
            if "computed_digest" in integrity_result and "recorded_digest" in integrity_result:
                print(f"       computed: {integrity_result['computed_digest'][:16]}...")
                print(f"       recorded: {integrity_result['recorded_digest'][:16]}...")

    # Stage 5: Evidence Manifest Generation
    manifest = generate_manifest(
        tscp_pl, canonical_result, schema_result,
        artifact_result, integrity_result, ledger_path
    )
    if not quiet:
        print(f"[5/7] Evidence manifest: {manifest['overall_status']}")

    # Stage 6: Payload Export
    export_result = stage_export_payload(canonical_data, manifest, PAYLOAD_FILE)
    if not quiet:
        print(f"[6/7] Payload export: {export_result['status']} "
              f"({export_result['payload_size_bytes']} bytes)")

    # Stage 7: Manifest Self-Validation
    self_validation = stage_manifest_self_validate()
    if not quiet:
        print(f"[7/7] Manifest self-validation: {self_validation['status']}")
        if self_validation["status"] == "FAIL":
            for err in self_validation.get("errors", []):
                print(f"       ERROR: {err}")

    # Determine final result
    all_stages = [
        canonical_result["status"],
        schema_result["status"],
        artifact_result["status"],
        integrity_result["status"],
        "PASS" if manifest and manifest["overall_status"] == "PASS" else "FAIL",
        export_result["status"],
        self_validation["status"],
    ]

    overall = "PASS" if all(s == "PASS" for s in all_stages) else "FAIL"

    if not quiet:
        print()
        if overall == "PASS":
            print("=== TSCP-PL VERIFICATION: PASS ===")
            print(f"  Canonical digest: {canonical_result['canonical_digest']}")
            print(f"  Payload digest:   {integrity_result.get('computed_digest', 'N/A')}")
            print(f"  Manifest:         {MANIFEST_FILE}")
            print(f"  Payload:          {PAYLOAD_FILE}")
        else:
            print("=== TSCP-PL VERIFICATION: FAIL ===")
            failed = [s for s in all_stages if s != "PASS"]
            print(f"  Failed stages: {len(failed)}")

    return EXIT_OK if overall == "PASS" else EXIT_VERIFY


# ─── CLI Entry Point ────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="TSCP-PL Reference Verifier v1.0",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Verification stages:
  1. Canonicalization
  2. Schema validation
  3. Artifact digest verification
  4. Payload integrity (mutation detection)
  5. Evidence manifest generation
  6. Payload export
  7. Manifest self-validation

Exit codes:
  0 = PASS
  1 = usage error
  2 = IO error
  3 = verification failure
        """
    )
    subparsers = parser.add_subparsers(dest="command", help="Subcommand")

    verify_parser = subparsers.add_parser("verify", help="Verify a TSCP-PL ledger")
    verify_parser.add_argument("ledger", help="Path to the .jsonld ledger file")
    verify_parser.add_argument("--quiet", action="store_true", help="Suppress output")

    digest_parser = subparsers.add_parser("digest", help="Compute canonical digest of a ledger")
    digest_parser.add_argument("ledger", help="Path to the .jsonld ledger file")

    args = parser.parse_args()

    if args.command == "verify":
        sys.exit(verify(args.ledger, quiet=args.quiet))
    elif args.command == "digest":
        try:
            tscp_pl = load_json(args.ledger)
        except (FileNotFoundError, json.JSONDecodeError) as e:
            print(f"ERROR: {e}", file=sys.stderr)
            sys.exit(EXIT_IO)
        print(sha256_bytes(canonical_bytes(tscp_pl)))
        sys.exit(EXIT_OK)
    else:
        parser.print_help()
        sys.exit(EXIT_USAGE)


if __name__ == "__main__":
    main()
