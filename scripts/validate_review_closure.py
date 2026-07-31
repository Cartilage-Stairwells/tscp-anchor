#!/usr/bin/env python3
"""
validate_review_closure.py - Review lifecycle closure validator

Checks review records against the TSCP review custody model:
  1. Lifecycle mutex: exactly one state is active
  2. Gate completeness: all three gates signed for CLOSED state
  3. CT reference integrity: referenced review record exists
  4. Baseline hash verification: declared hash matches calculated evidence hash
  5. Disposition presence: CLOSED records have a disposition assigned

Usage:
    python3 scripts/validate_review_closure.py <record_dir> [--evidence-root <path>]

Exit codes:
  0 - PASS (record satisfies all closure invariants)
  1 - FAIL (one or more invariants violated)
"""

import hashlib
import re
import sys
from pathlib import Path

LIFECYCLE_STATES = [
    "INITIALIZED",
    "BASELINE_CONFIRMED",
    "OBSERVATION_CAPTURED",
    "EVALUATION_COMPLETE",
    "DISPOSITION_ASSIGNED",
    "CLOSED",
]


class ValidationResult:
    def __init__(self, record_id):
        self.record_id = record_id
        self.checks = []
        self.passed = True

    def add(self, name, passed, detail=""):
        self.checks.append((name, "PASS" if passed else "FAIL", detail))
        if not passed:
            self.passed = False

    def summary(self):
        lines = [f"Record: {self.record_id}", f"Overall: {'PASS' if self.passed else 'FAIL'}", ""]
        for name, status, detail in self.checks:
            marker = "[PASS]" if status == "PASS" else "[FAIL]"
            lines.append(f"  {marker} {name}: {status}")
            if detail:
                lines.append(f"      {detail}")
        return "\n".join(lines)


def parse_record(record_path):
    text = record_path.read_text(encoding="utf-8")
    result = {}

    m = re.search(r"\*\*Record ID:\*\*[ \t]*(.+)", text)
    result["record_id"] = m.group(1).strip() if m else None

    states_active = []
    for state in LIFECYCLE_STATES:
        pattern = rf"-\s*\[([ xX])\]\s*{re.escape(state)}"
        m = re.search(pattern, text)
        if m and m.group(1).lower() == "x":
            states_active.append(state)
    result["states_active"] = states_active

    result["gates"] = {}
    for gate_num, gate_name in [(1, "Baseline Confirmation"),
                               (2, "Evaluation Complete"),
                               (3, "Transition Authority")]:
        gate_pattern = rf"### Gate {gate_num}:\s*{re.escape(gate_name)}"
        gate_match = re.search(gate_pattern, text)
        if gate_match:
            after = text[gate_match.end():]
            next_section = re.search(r"(\n###|\n---|\n## )", after)
            section_text = after[:next_section.start()] if next_section else after
            signoff_m = re.search(r"\*\*Sign-Off ID:\*\*[ \t]*(\S+)", section_text)
            confirmed_m = re.search(r"\*\*Confirmed by:\*\*[ \t]*(\S+)", section_text)
            signoff_id = signoff_m.group(1).strip() if signoff_m else None
            confirmed_by = confirmed_m.group(1).strip() if confirmed_m else None
            result["gates"][gate_num] = {
                "signed": bool(signoff_id and confirmed_by),
                "signoff_id": signoff_id,
                "confirmed_by": confirmed_by,
            }
        else:
            result["gates"][gate_num] = {"signed": False, "signoff_id": None, "confirmed_by": None}

    m = re.search(r"\*\*CT ID:\*\*[ \t]*(\S+)", text)
    result["ct_id"] = m.group(1).strip() if m else None

    m = re.search(r"\*\*SHA-256:\*\*[ \t]*(\S+)", text)
    result["declared_hash"] = m.group(1).strip() if m else None

    m = re.search(r"\*\*Disposition:\*\*[ \t]*(.+)", text)
    result["disposition"] = m.group(1).strip() if m else None

    m = re.search(r"\*\*Closed:\*\*[ \t]*(.+)", text)
    result["closed_date"] = m.group(1).strip() if m else None

    m = re.search(r"\*\*Closed By:\*\*[ \t]*(.+)", text)
    result["closed_by"] = m.group(1).strip() if m else None

    return result


def calculate_evidence_hash(record_dir):
    evidence_dir = record_dir / "evidence"
    if not evidence_dir.exists():
        return None
    hasher = hashlib.sha256()
    file_count = 0
    for filepath in sorted(evidence_dir.rglob("*")):
        if filepath.is_file():
            hasher.update(filepath.read_bytes())
            file_count += 1
    return hasher.hexdigest() if file_count > 0 else None


def check_lifecycle_mutex(result, record):
    active = record["states_active"]
    if len(active) == 0:
        result.add("Lifecycle mutex", False, "No lifecycle state is active")
    elif len(active) == 1:
        result.add("Lifecycle mutex", True, f"Active state: {active[0]}")
    else:
        result.add("Lifecycle mutex", False, f"Multiple states active: {', '.join(active)}")


def check_gate_completeness(result, record):
    if "CLOSED" not in record["states_active"]:
        result.add("Gate completeness", True, "Not CLOSED - gate check skipped")
        return
    all_signed = all(record["gates"].get(i, {}).get("signed", False) for i in (1, 2, 3))
    if all_signed:
        gates = [f"G{i}:{record['gates'][i]['signoff_id']}" for i in (1, 2, 3)]
        result.add("Gate completeness", True, f"All gates signed: {', '.join(gates)}")
    else:
        unsigned = [i for i in (1, 2, 3) if not record["gates"].get(i, {}).get("signed", False)]
        result.add("Gate completeness", False, f"Unsigned gates: {', '.join(str(g) for g in unsigned)}")


def check_ct_reference(result, record, evidence_root):
    ct_id = record["ct_id"]
    if not ct_id:
        result.add("CT reference", False, "No CT ID declared")
        return
    m = re.search(r"(\d+)", ct_id)
    if not m:
        result.add("CT reference", False, f"CT ID has no numeric suffix: {ct_id}")
        return
    num = m.group(1)
    record_id = record.get("record_id", "")
    if ct_id.startswith("CT-REV-TEST-") or ct_id.startswith("CT-REV-ISSUE-"):
        if evidence_root:
            search_name = f"REV-TEST-{num}" if "TEST" in ct_id else f"REV-ISSUE-{num}"
            record_path = evidence_root / search_name / "review-record.md"
            if record_path.exists():
                result.add("CT reference", True, f"Review record exists: {search_name}")
                return
        if record_id and record_id.endswith(num):
            result.add("CT reference", True, f"Self-referencing dry-run record: {ct_id}")
            return
        result.add("CT reference", False, f"Referenced review record not found for CT ID: {ct_id}")
    elif ct_id.startswith("CT-ISSUE-"):
        if evidence_root:
            search_name = f"REV-ISSUE-{num}"
            record_path = evidence_root / search_name / "review-record.md"
            if record_path.exists():
                result.add("CT reference", True, f"Review record exists: {search_name}")
                return
        result.add("CT reference", False, f"Referenced review record not found: {ct_id}")
    else:
        result.add("CT reference", True, f"CT ID format recognized: {ct_id}")


def check_baseline_hash(result, record, record_dir):
    declared = record["declared_hash"]
    if not declared:
        result.add("Baseline hash", False, "No SHA-256 declared")
        return
    calculated = calculate_evidence_hash(record_dir)
    if calculated is None:
        if len(declared) >= 64 and re.match(r"^[a-f0-9]+$", declared):
            result.add("Baseline hash", True, "Hash declared (no evidence files to verify against)")
        else:
            result.add("Baseline hash", False, f"Declared hash is not a valid SHA-256: {declared}")
        return
    if declared == calculated:
        result.add("Baseline hash", True, f"Hash verified: {declared[:16]}...")
    else:
        result.add("Baseline hash", False, f"Declared: {declared[:16]}... Calculated: {calculated[:16]}...")


def check_disposition(result, record):
    if "CLOSED" not in record["states_active"]:
        result.add("Disposition presence", True, "Not CLOSED - disposition check skipped")
        return
    if record["disposition"]:
        result.add("Disposition presence", True, f"Disposition: {record['disposition']}")
    else:
        result.add("Disposition presence", False, "CLOSED record has no disposition assigned")


def check_closure_metadata(result, record):
    if "CLOSED" not in record["states_active"]:
        result.add("Closure metadata", True, "Not CLOSED - metadata check skipped")
        return
    has_date = bool(record["closed_date"])
    has_by = bool(record["closed_by"])
    if has_date and has_by:
        result.add("Closure metadata", True, f"Closed: {record['closed_date']} by {record['closed_by']}")
    else:
        missing = []
        if not has_date:
            missing.append("closure date")
        if not has_by:
            missing.append("closed by")
        result.add("Closure metadata", False, f"Missing: {', '.join(missing)}")


def validate_record(record_dir, evidence_root=None):
    record_dir = Path(record_dir)
    record_path = record_dir / "review-record.md"
    if not record_path.exists():
        print(f"ERROR: review-record.md not found in {record_dir}")
        return None
    record = parse_record(record_path)
    result = ValidationResult(record.get("record_id", "UNKNOWN"))
    check_lifecycle_mutex(result, record)
    check_gate_completeness(result, record)
    check_ct_reference(result, record, evidence_root)
    check_baseline_hash(result, record, record_dir)
    check_disposition(result, record)
    check_closure_metadata(result, record)
    return result


def main():
    if len(sys.argv) < 2:
        print("Usage: validate_review_closure.py <record_dir> [--evidence-root <path>]")
        sys.exit(1)
    record_dir = Path(sys.argv[1])
    evidence_root = None
    args = sys.argv[2:]
    for i, arg in enumerate(args):
        if arg == "--evidence-root" and i + 1 < len(args):
            evidence_root = Path(args[i + 1])
            break
    result = validate_record(record_dir, evidence_root)
    if result is None:
        sys.exit(1)
    print(result.summary())
    print()
    print(f"RESULT: {'PASS' if result.passed else 'FAIL'}")
    sys.exit(0 if result.passed else 1)


if __name__ == "__main__":
    main()
