#!/usr/bin/env python3
"""Determinism regression: compare two verification manifests."""
import json
import sys

m1 = json.load(open("/tmp/manifest1.json"))
m2 = json.load(open("/tmp/manifest2.json"))

d1 = m1["stages"]["canonicalization"]["canonical_digest"]
d2 = m2["stages"]["canonicalization"]["canonical_digest"]
p1 = m1["stages"]["payload_integrity"]["computed_digest"]
p2 = m2["stages"]["payload_integrity"]["computed_digest"]
arts1 = [a.get("actual_digest", "") for a in m1["stages"]["artifact_digests"]["artifacts"]]
arts2 = [a.get("actual_digest", "") for a in m2["stages"]["artifact_digests"]["artifacts"]]

ok = (d1 == d2) and (p1 == p2) and (arts1 == arts2)

print(f"  Run 1 canonical: {d1[:16]}...")
print(f"  Run 2 canonical: {d2[:16]}...")
print(f"  Run 1 payload:   {p1[:16]}...")
print(f"  Run 2 payload:   {p2[:16]}...")
print(f"  Artifact digests match: {arts1 == arts2}")

if ok:
    print("  Determinism: PASS ✅ (all digests identical across runs)")
    sys.exit(0)
else:
    print("  Determinism: FAIL ❌ (digests differ across runs)")
    sys.exit(1)
