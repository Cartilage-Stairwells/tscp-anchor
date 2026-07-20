#!/usr/bin/env python3
"""
Supply Chain Governance Validator

Exit codes:
  0 — PASS (all checks green, evidence manifest emitted)
  1 — Schema failure (exceptions.json does not conform to exceptions.schema.json)
  2 — Governance failure (audit mapping mismatch or orphan ignores)
  3 — Evidence failure (evidence manifest could not be written)

Usage:
  python3 scripts/validate_governance.py [--schema PATH] [--data PATH] [--audit PATH] [--out PATH]

Defaults:
  --schema  schemas/exceptions.schema.json
  --data    schemas/exceptions.json
  --audit   audit.toml
  --out     supply-chain-evidence/evidence_manifest.json
"""

import argparse
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

# Minimal TOML parser (stdlib tomllib in 3.11+, fallback for older)
try:
    import tomllib
except ImportError:
    tomllib = None


def parse_toml(path):
    """Parse a TOML file, using stdlib if available, else minimal fallback."""
    if tomllib is not None:
        with open(path, "rb") as f:
            return tomllib.load(f)
    # Minimal fallback: extract ignore list from audit.toml format
    ignores = []
    in_ignore = False
    with open(path, "r") as f:
        for line in f:
            line = line.strip()
            if line.startswith("ignore") and "=" in line:
                # Single-line: ignore = ["RUSTSEC-..."]
                if "]" in line:
                    start = line.index("[")
                    end = line.index("]")
                    items = line[start+1:end].split(",")
                    for item in items:
                        item = item.strip().strip('"').strip("'")
                        if item:
                            ignores.append(item)
                    in_ignore = False
                else:
                    # Multi-line start
                    in_ignore = True
                    rest = line.split("=", 1)[1].strip()
                    if rest.startswith("["):
                        rest = rest[1:]
                        if "]" in rest:
                            end = rest.index("]")
                            items = rest[:end].split(",")
                            for item in items:
                                item = item.strip().strip('"').strip("'")
                                if item:
                                    ignores.append(item)
                            in_ignore = False
            elif in_ignore:
                if "]" in line:
                    in_ignore = False
                    items = line[:line.index("]")].split(",")
                    for item in items:
                        item = item.strip().strip('"').strip("'")
                        if item:
                            ignores.append(item)
                else:
                    item = line.strip().strip('"').strip("'")
                    if item:
                        ignores.append(item)
    return {"advisories": {"ignore": ignores}}


def validate_schema(data, schema):
    """Validate data against JSON Schema. Returns (ok, errors)."""
    try:
        import jsonschema
    except ImportError:
        # Minimal validation without jsonschema library
        return minimal_validate(data, schema)

    try:
        jsonschema.validate(data, schema, format_checker=jsonschema.FormatChecker())
        return True, []
    except jsonschema.ValidationError as e:
        return False, [str(e)]
    except jsonschema.SchemaError as e:
        return False, [f"Schema error: {e}"]


def minimal_validate(data, schema):
    """Minimal validation when jsonschema is not available."""
    errors = []

    # Check schema_version
    if data.get("schema_version") != "1.0":
        errors.append(f"schema_version must be '1.0', got '{data.get('schema_version')}'")

    exceptions = data.get("exceptions", [])
    if not isinstance(exceptions, list):
        errors.append("'exceptions' must be an array")
        return False, errors

    required_fields = [
        "exception_id", "status", "dependency", "owner",
        "evidence", "resolution", "first_recorded", "last_reviewed"
    ]
    valid_statuses = {"PROPOSED", "ACTIVE", "MITIGATED", "RESOLVED", "ARCHIVED"}
    seen_ids = set()

    for i, exc in enumerate(exceptions):
        if not isinstance(exc, dict):
            errors.append(f"Exception {i}: must be an object")
            continue

        # Check required fields
        for field in required_fields:
            if field not in exc:
                errors.append(f"Exception {i}: missing required field '{field}'")

        # Check exception_id format
        eid = exc.get("exception_id", "")
        import re
        if not re.match(r"^EX-\d{4}$", eid):
            errors.append(f"Exception {i}: exception_id '{eid}' does not match EX-NNNN")

        # Check for duplicate IDs
        if eid in seen_ids:
            errors.append(f"Exception {i}: duplicate exception_id '{eid}'")
        seen_ids.add(eid)

        # Check status
        status = exc.get("status", "")
        if status not in valid_statuses:
            errors.append(f"Exception {i}: invalid status '{status}', must be one of {valid_statuses}")

        # Check allOf conditional: ACTIVE requires audit
        if status == "ACTIVE" and "audit" not in exc:
            errors.append(f"Exception {eid}: status is ACTIVE but 'audit' field is missing (allOf contract)")

        # Check additional properties
        allowed_props = {
            "exception_id", "status", "dependency", "owner", "evidence",
            "resolution", "first_recorded", "last_reviewed",
            "associated_advisories", "planned_mitigations",
            "next_review", "notes", "audit"
        }
        for key in exc:
            if key not in allowed_props:
                errors.append(f"Exception {eid}: unknown property '{key}'")

    return len(errors) == 0, errors


def validate_audit_mapping(exceptions, audit_ignores):
    """
    Validate that:
    1. Every ACTIVE exception's audit advisory exists in audit.toml ignore list
    2. Every audit.toml ignore entry has a corresponding ACTIVE exception
    Returns (ok, errors).
    """
    errors = []

    # Collect all audit IDs from active exceptions
    active_audit_ids = set()
    active_exceptions = [e for e in exceptions if e.get("status") == "ACTIVE"]

    for exc in active_exceptions:
        audit_field = exc.get("audit", "")
        if not audit_field:
            errors.append(
                f"{exc['exception_id']}: status is ACTIVE but no audit field"
            )
            continue

        # Parse comma-separated advisory IDs
        exc_audit_ids = [a.strip() for a in audit_field.split(",")]
        for aid in exc_audit_ids:
            active_audit_ids.add(aid)
            if aid not in audit_ignores:
                errors.append(
                    f"{exc['exception_id']}: advisory '{aid}' is not in audit.toml ignore list"
                )

    # Check for orphan ignores (in audit.toml but no active exception)
    for ignored in audit_ignores:
        if ignored not in active_audit_ids:
            errors.append(
                f"Orphan ignore: '{ignored}' is in audit.toml but has no ACTIVE exception"
            )

    return len(errors) == 0, errors


def get_git_commit():
    """Get current git commit SHA."""
    try:
        result = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            capture_output=True, text=True, check=True
        )
        return result.stdout.strip()
    except Exception:
        return "unknown"


def emit_evidence_manifest(exceptions, schema_ok, audit_ok, git_commit, out_path):
    """Emit the evidence manifest artifact."""
    manifest = {
        "schema_version": "1.0",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "git_commit": git_commit,
        "conformance": "PASS" if schema_ok and audit_ok else "FAIL",
        "checks": [
            {
                "name": "registry-schema",
                "status": "PASS" if schema_ok else "FAIL",
                "artifact": "schemas/exceptions.json"
            },
            {
                "name": "audit-mapping",
                "status": "PASS" if audit_ok else "FAIL",
                "artifact": "audit.toml"
            }
        ],
        "exceptions": {
            "total": len(exceptions),
            "active": sum(1 for e in exceptions if e.get("status") == "ACTIVE"),
            "resolved": sum(1 for e in exceptions if e.get("status") == "RESOLVED"),
            "proposed": sum(1 for e in exceptions if e.get("status") == "PROPOSED"),
            "mitigated": sum(1 for e in exceptions if e.get("status") == "MITIGATED"),
            "archived": sum(1 for e in exceptions if e.get("status") == "ARCHIVED"),
        }
    }

    out_dir = os.path.dirname(out_path)
    if out_dir:
        os.makedirs(out_dir, exist_ok=True)

    with open(out_path, "w") as f:
        json.dump(manifest, f, indent=2)
        f.write("\n")

    return manifest


def main():
    parser = argparse.ArgumentParser(description="Supply Chain Governance Validator")
    parser.add_argument("--schema", default="schemas/exceptions.schema.json")
    parser.add_argument("--data", default="schemas/exceptions.json")
    parser.add_argument("--audit", default="audit.toml")
    parser.add_argument("--out", default="supply-chain-evidence/evidence_manifest.json")
    args = parser.parse_args()

    # 1. Load and validate schema
    print("[1/4] Validating exceptions.json against schema...")
    try:
        with open(args.data) as f:
            data = json.load(f)
        with open(args.schema) as f:
            schema = json.load(f)
    except Exception as e:
        print(f"  FAIL: Could not load files: {e}")
        sys.exit(1)

    schema_ok, schema_errors = validate_schema(data, schema)
    if schema_ok:
        print("  PASS: Schema validation successful")
    else:
        print(f"  FAIL: Schema validation failed:")
        for err in schema_errors:
            print(f"    - {err}")
        sys.exit(1)

    exceptions = data.get("exceptions", [])

    # 2. Validate audit mapping
    print("[2/4] Validating audit.toml mapping against active exceptions...")
    try:
        audit_config = parse_toml(args.audit)
        audit_ignores = set(audit_config.get("advisories", {}).get("ignore", []))
    except Exception as e:
        print(f"  FAIL: Could not parse audit.toml: {e}")
        sys.exit(2)

    audit_ok, audit_errors = validate_audit_mapping(exceptions, audit_ignores)
    if audit_ok:
        print(f"  PASS: Audit mapping consistent ({len(audit_ignores)} ignore entries, "
              f"{sum(1 for e in exceptions if e.get('status') == 'ACTIVE')} active exceptions)")
    else:
        print(f"  FAIL: Governance failure:")
        for err in audit_errors:
            print(f"    - {err}")
        sys.exit(2)

    # 3. Check for duplicate exception IDs (immutability rule)
    print("[3/4] Checking exception ID immutability...")
    ids = [e.get("exception_id") for e in exceptions]
    duplicates = [eid for eid in ids if ids.count(eid) > 1]
    if duplicates:
        print(f"  FAIL: Duplicate exception IDs: {set(duplicates)}")
        sys.exit(2)
    print(f"  PASS: {len(ids)} unique exception IDs")

    # 4. Emit evidence manifest
    print("[4/4] Emitting evidence manifest...")
    git_commit = get_git_commit()
    try:
        manifest = emit_evidence_manifest(
            exceptions, schema_ok, audit_ok, git_commit, args.out
        )
        print(f"  PASS: Evidence manifest written to {args.out}")
        print(f"  Conformance: {manifest['conformance']}")
        print(f"  Git commit: {git_commit}")
    except Exception as e:
        print(f"  FAIL: Could not write evidence manifest: {e}")
        sys.exit(3)

    print("\nAll governance checks PASS.")
    sys.exit(0)


if __name__ == "__main__":
    main()
