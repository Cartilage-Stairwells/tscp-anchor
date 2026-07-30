#!/usr/bin/env python3
"""
run_review_lifecycle_tests.py - Aggregate test harness for review lifecycle
"""

import subprocess
import sys
from pathlib import Path

FIXTURES = Path("tests/review_lifecycle/fixtures")
VALIDATOR = "scripts/validate_review_closure.py"

TESTS = [
    ("REV-TEST-001", 0),
    ("REV-TEST-002", 1),
    ("REV-TEST-003", 1),
    ("REV-TEST-004", 1),
    ("REV-TEST-005", 1),
]


def main():
    passed = 0
    failed = 0

    for fixture, expected_code in TESTS:
        result = subprocess.run(
            [
                sys.executable,
                VALIDATOR,
                str(FIXTURES / fixture),
                "--evidence-root",
                str(FIXTURES),
            ],
            capture_output=True,
            text=True,
        )

        actual_code = result.returncode
        ok = actual_code == expected_code

        if ok:
            passed += 1
            label = "PASS" if expected_code == 0 else "FAIL"
            print(f"  [OK] {fixture}: {label} (as expected)")
        else:
            failed += 1
            label = "PASS" if actual_code == 0 else "FAIL"
            print(f"  [ERR] {fixture}: expected different result, got {label}")
            print(result.stdout)

    print()
    print(f"Summary: {passed}/{len(TESTS)} passed, {failed} failed")
    print(f"Result: {'PASS' if failed == 0 else 'FAIL'}")

    sys.exit(0 if failed == 0 else 1)


if __name__ == "__main__":
    main()
