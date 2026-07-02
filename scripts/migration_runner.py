#!/usr/bin/env python3

import argparse
import hashlib
import json
import os
import subprocess
from datetime import datetime, timezone


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return "sha256:" + h.hexdigest()


def run_verifier(verifier, fixture):
    result = subprocess.run(
        [verifier, "verify", "--envelope", fixture],
        capture_output=True,
        text=True,
    )

    return {
        "exit_code": result.returncode,
        "stdout": result.stdout,
        "stderr": result.stderr,
    }


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--fixture",
        action="append",
        required=True,
    )

    parser.add_argument(
        "--verifier",
        required=True,
    )

    parser.add_argument(
        "--emit-witness",
        required=True,
    )

    args = parser.parse_args()

    checks = []
    fixture_hashes = {}

    overall = "PASS"

    for fixture in args.fixture:
        fixture_hashes[os.path.basename(fixture)] = sha256_file(fixture)

        result = run_verifier(args.verifier, fixture)

        status = "PASS" if result["exit_code"] == 0 else "FAIL"

        if status == "FAIL":
            overall = "FAIL"

        checks.append(
            {
                "fixture": fixture,
                "status": status,
                "exit_code": result["exit_code"],
            }
        )

    witness = {
        "spec": "TSCP-MIGRATION-001",
        "result": overall,
        "execution": {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "verifier": args.verifier,
        },
        "checks": checks,
        "hashes": {
            "fixtures": fixture_hashes,
            "verifier_binary": sha256_file(args.verifier),
            "runner": sha256_file(__file__),
        },
    }

    os.makedirs(
        os.path.dirname(args.emit_witness),
        exist_ok=True,
    )

    with open(args.emit_witness, "w") as f:
        json.dump(
            witness,
            f,
            indent=2,
        )


if __name__ == "__main__":
    main()
