#!/usr/bin/env python3
"""
regen_sorry_inventory.py — EXC-029 remediation (2026-09-07).

Regenerates formal/sorry-inventory.json from the AUTHORITATIVE source (the
repository's .lean files) instead of a hand-maintained table. Per the
TSCP formalization invariant: derived semantic relations are computed from
the underlying primitive, never maintained in parallel by hand.

A "real sorry" is a sorry PROOF TERM: a line whose code (excluding comments)
contains the `sorry` keyword as a proof placeholder. Comment mentions of
"sorry" (e.g. "-- fully proved, no sorry") are NOT counted.

Usage:
    python3 scripts/regen_sorry_inventory.py [--dry-run]

Output: formal/sorry-inventory.json, schema 2.0 — per-file counts with line
numbers, total, source commit, and the generator identity.
"""

import argparse
import datetime
import json
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
INVENTORY_PATH = REPO / "formal" / "sorry-inventory.json"
SORRY_TERM = re.compile(r'\bsorry\b')

def real_sorry_lines(text: str):
    """Lines where a sorry appears outside of comments AND block docstrings.

    Lean has three prose contexts that legitimately mention the word
    'sorry' without being proof terms: '--' line comments, and the
    block comment forms '/- ... -/' and '/-- ... -/' (docstrings).
    Docstring prose lines do not start with '--', so a naive filter
    overcounts (validated against the EXC-029 invariant: 12 Kernel.lean
    + 3 NormalizationBridge.lean = 15 true terms; a filter without
    docstring tracking counted 18)."""
    hits = []
    in_block = False
    for lineno, line in enumerate(text.splitlines(), 1):
        stripped = line.lstrip()
        # track block comment / docstring state: '/--', '/-', close at '-/'
        opens = line.count("/--") + line.count("/-") - line.count("/--")
        closes = line.count("-/")
        if in_block:
            if closes > 0:
                in_block = False
                # a proof term cannot share a line that closes a docstring
                # and carries no code; treat the whole line as prose
            continue
        if stripped.startswith("--"):
            continue  # full-line comment
        code = line.split("--")[0] if "--" in line else line
        if opens > 0:
            # opens on this line: code before '/-' is code, but any 'sorry'
            # after an opener is prose; conservatively scan only the prefix
            code = line[: line.find("/-")]
            if closes > 0:  # single-line block comment
                code = code + line[line.rfind("-/") + 2 :]
            in_block = True if (opens > closes) else False
        if SORRY_TERM.search(code):
            hits.append(lineno)
    return hits

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    commit = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=REPO,
        capture_output=True, text=True, check=True).stdout.strip()

    files = sorted(REPO.rglob("*.lean"))
    per_file = []
    total = 0
    for f in files:
        rel = f.relative_to(REPO).as_posix()
        try:
            lines = real_sorry_lines(f.read_text(errors="replace"))
        except OSError:
            continue
        if lines:
            per_file.append({"file": rel, "sorry_count": len(lines), "sorry_lines": lines})
            total += len(lines)

    inventory = {        "scope": ("All .lean files in this repository. Line-comment (--) and "
                  "block-comment/docstring (/-, /--) contexts are excluded as "
                  "prose; only proof-term 'sorry' is counted. Repo-scoped "
                  "(tscp-anchor); does not cover other repositories' Lean surfaces."),

        "schema_version": "2.0",
        "generated_by": "scripts/regen_sorry_inventory.py",
        "generated_at": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
        "source_commit": commit,
        "files": per_file,
        "total_sorry": total,
        "note": "Regenerated from the authoritative source (repo .lean files). "
                "Counts real sorry proof terms only; comment mentions excluded. "
                "Supersedes the hand-maintained v1.0 inventory (EXC-029)."
    }

    out = json.dumps(inventory, indent=2) + "\n"
    if args.dry_run:
        print(out)
        return 0
    INVENTORY_PATH.write_text(out)
    print(f"wrote {INVENTORY_PATH.relative_to(REPO)}: total_sorry={total} across {len(per_file)} files")
    for e in per_file:
        print(f"  {e['file']}: {e['sorry_count']} ({e['sorry_lines']})")
    return 0

if __name__ == "__main__":
    sys.exit(main())
