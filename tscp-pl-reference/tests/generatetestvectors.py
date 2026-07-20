#!/usr/bin/env python3
"""
Generate deterministic test artifacts and TSCP-PL ledgers for verifier regression tests.

Creates:
  tests/artifacts/sample.txt
  tests/artifacts/sample2.txt
  tests/ledgers/{good,reorderkeys,mutated,missingartifact,bad_hash}.jsonld

Usage:
  python3 generatetestvectors.py
"""
import os
import json
import hashlib
from datetime import datetime
from copy import deepcopy

ROOT = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..")
)
TEST_DIR = os.path.join(ROOT, "tests")
ART_DIR = os.path.join(TEST_DIR, "artifacts")
LEDGER_DIR = os.path.join(TEST_DIR, "ledgers")

os.makedirs(ART_DIR, exist_ok=True)
os.makedirs(LEDGER_DIR, exist_ok=True)


def write_file(path, data):
    with open(path, "wb") as f:
        f.write(data)


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


def _sort(obj):
    """Recursively sort all dict keys for deterministic serialization."""
    if isinstance(obj, dict):
        return {k: _sort(obj[k]) for k in sorted(obj.keys())}
    if isinstance(obj, list):
        return [_sort(item) for item in obj]
    return obj


def canonical_bytes(obj):
    """Serialize to canonical JSON bytes: sorted keys, no whitespace, UTF-8."""
    return json.dumps(_sort(obj), separators=(",", ":"), ensure_ascii=False).encode("utf-8")


def canonical_digest(obj):
    """SHA256 of canonical bytes."""
    return hashlib.sha256(canonical_bytes(obj)).hexdigest()


def payload_digest(ledger):
    """
    Compute the canonical digest of the ledger's payload — everything
    EXCEPT the verification block. This is the content identity:
    mutating conversation content changes this digest, but the recorded
    digest in verification.checks won't match.
    """
    payload = {k: v for k, v in ledger.items() if k != "verification"}
    return canonical_digest(payload)


# 1) Create deterministic artifact files
sample_path = os.path.join(ART_DIR, "sample.txt")
sample2_path = os.path.join(ART_DIR, "sample2.txt")

write_file(sample_path, b"TSCP-PL test artifact\ncontent: stable\n")
write_file(sample2_path, b"TSCP-PL second artifact\ncontent: stable\n")

sample_digest = sha256_file(sample_path)
sample2_digest = sha256_file(sample2_path)


# Helper to build a minimal ledger object (based on the reference example)
def base_ledger():
    return {
        "@context": {
            "tscp": "https://example.org/tscp/v1#",
            "prov": "http://www.w3.org/ns/prov#",
            "schema": "http://schema.org/"
        },
        "@type": "tscp:ProvenanceLedger",
        "id": "tscp-pl:test-conversation",
        "title": "TSCP-PL test vectors",
        "created": datetime.utcnow().isoformat() + "Z",
        "creator": {"name": "test-harness", "role": "tester"},
        "ledger": {
            "format": "TSCP-PL",
            "version": "1.0.0",
            "revision": 1,
            "parent_digest": None
        },
        "conversation": {
            "format": "message-array",
            "messages": [
                {
                    "seq": 1,
                    "speaker": "user",
                    "timestamp": datetime.utcnow().isoformat() + "Z",
                    "content_type": "text/plain",
                    "content": "Test vector ledger"
                }
            ]
        },
        "artifacts": [],
        "claims": [],
        "verification": {
            "status": "pending",
            "checks": []
        },
        "provenance": {
            "created_by": "test-harness",
            "created_at": datetime.utcnow().isoformat() + "Z"
        },
        "tscp:signature": None
    }


def finalize_ledger(ledger):
    """
    Compute the payload digest (everything except verification) and record
    it in the verification block. This creates a self-referential integrity
    check: mutating the content changes the payload digest, but the recorded
    digest in verification.checks won't match.
    """
    pd = payload_digest(ledger)
    ledger["verification"]["status"] = "verified"
    ledger["verification"]["checks"] = [
        {
            "name": "payload_digest",
            "status": "PASS",
            "recorded_digest": pd
        }
    ]
    return ledger


# 2) Good ledger: includes artifacts with correct digest + recorded payload digest
good = base_ledger()
good["artifacts"] = [
    {"id": "artifact:sample", "type": "blob", "path": os.path.relpath(sample_path, ROOT), "digest": f"sha256:{sample_digest}", "scope": "package-relative"},
    {"id": "artifact:sample2", "type": "blob", "path": os.path.relpath(sample2_path, ROOT), "digest": f"sha256:{sample2_digest}", "scope": "package-relative"}
]
finalize_ledger(good)
with open(os.path.join(LEDGER_DIR, "good.jsonld"), "w", encoding="utf-8") as f:
    json.dump(good, f, indent=2)

# 3) Reorder keys ledger: same content but reorder top-level keys
# Must produce identical payload digest despite different key order
reorder = deepcopy(good)
reorder_ordered = {
    "@context": reorder["@context"],
    "title": reorder["title"],
    "@type": reorder["@type"],
    "id": reorder["id"],
    "created": reorder["created"],
    "creator": reorder["creator"],
    "ledger": reorder["ledger"],
    "conversation": reorder["conversation"],
    "artifacts": reorder["artifacts"],
    "claims": reorder["claims"],
    "verification": reorder["verification"],
    "provenance": reorder["provenance"],
    "tscp:signature": reorder["tscp:signature"]
}
with open(os.path.join(LEDGER_DIR, "reorderkeys.jsonld"), "w", encoding="utf-8") as f:
    json.dump(reorder_ordered, f, indent=2)

# 4) Mutated ledger: change a message content
# The recorded payload_digest in verification.checks still has the old
# digest, so the verifier will detect the mismatch → FAIL
mutated = deepcopy(good)
mutated["conversation"]["messages"][0]["content"] = "Test vector ledger - mutated"
# Do NOT re-finalize — the old recorded digest is intentionally stale
with open(os.path.join(LEDGER_DIR, "mutated.jsonld"), "w", encoding="utf-8") as f:
    json.dump(mutated, f, indent=2)

# 5) Missing artifact ledger: reference a non-existent path
missing = deepcopy(good)
missing["artifacts"] = [
    {"id": "artifact:missing", "type": "blob", "path": "tests/artifacts/doesnotexist.txt", "digest": "sha256:deadbeef", "scope": "package-relative"}
]
# Re-finalize since we changed artifacts (the payload digest must be consistent)
finalize_ledger(missing)
with open(os.path.join(LEDGER_DIR, "missingartifact.jsonld"), "w", encoding="utf-8") as f:
    json.dump(missing, f, indent=2)

# 6) Bad hash ledger: reference sample.txt but with wrong digest
bad_hash = deepcopy(good)
bad_hash["artifacts"][0]["digest"] = "sha256:0000000000000000000000000000000000000000000000000000000000000000"
# Re-finalize since we changed artifacts
finalize_ledger(bad_hash)
with open(os.path.join(LEDGER_DIR, "bad_hash.jsonld"), "w", encoding="utf-8") as f:
    json.dump(bad_hash, f, indent=2)

print("Generated test artifacts and ledgers in:")
print("  -", ART_DIR)
print("  -", LEDGER_DIR)
print("Sample digest:", sample_digest)
print("Sample2 digest:", sample2_digest)
