#!/usr/bin/env python3
"""Kernel-audit gate checker and receipt generator.

Inputs:
  1. Machine-readable audit output from audit/Audit.lean
     (AUDIT|<kind>|<name>|<class>|<deps> lines, one per TSCP constant
     in the elaborated environment).
  2. audit/expected-gaps.json — the accepted-gap manifest.
  3. The source corpus under TSCP/Formal (comment-stripped census).

Gates (exit non-zero on any failure):
  G1  sorryAx dependency for a constant outside sorryAx_allowed.
  G2  project-axiom dependency for a constant outside the axiom's
      declared dependents, or an undeclared project axiom, or a
      declared axiom absent from the environment.
  G3  a source-declared constant (theorem/def/axiom/structure/
      inductive/abbrev) absent from the audited environment
      (catches a root that built but was not audited, and an audit
      that ran over the wrong environment).
  G4  zero theorems audited (audit did not run over the corpus).
  G5  ambiguous name matching between census and audit.

The receipt printed to stdout has every count COMPUTED from the
audit output and census — nothing hardcoded.
"""
import json
import re
import sys
from collections import Counter
from pathlib import Path

KIND_OF = {
    "theorem": "theorem", "lemma": "theorem", "axiom": "axiom",
    "def": "def", "abbrev": "def", "instance": "def",
    "structure": "inductive", "inductive": "inductive",
}


def strip_lean(src: str) -> str:
    """Comment-stripping state machine (nested /- -/, doc blocks, --, strings)."""
    out, i, n, depth = [], 0, len(src), 0
    while i < n:
        if depth == 0 and src.startswith("/--", i):
            depth, i = depth + 1, i + 3
            continue
        if depth == 0 and src.startswith("/-", i):
            depth, i = depth + 1, i + 2
            continue
        if depth > 0:
            if src.startswith("/-", i):
                depth, i = depth + 1, i + 2
            elif src.startswith("-/", i):
                depth, i = depth - 1, i + 2
            else:
                i += 1
            continue
        if src.startswith("--", i):
            j = src.find("\n", i)
            i = n if j < 0 else j
            continue
        if src[i] == '"':
            j = i + 1
            while j < n and src[j] != '"':
                if src[j] == "\\":
                    j += 1
                j += 1
            out.append(src[i:j + 1])
            i = j + 1
            continue
        out.append(src[i])
        i += 1
    return "".join(out)


DECL = re.compile(
    r"^\s*(?:private\s+|protected\s+|noncomputable\s+|@\[[^\]]*\]\s+)*"
    r"(theorem|lemma|axiom|def|abbrev|structure|inductive|instance)\s+"
    r"([A-Za-z_][A-Za-z0-9_.']*)?", re.M)


def census(source_root: Path):
    """Source-declared constants: (kind, name, file). None name = anonymous."""
    items = []
    for p in sorted(source_root.rglob("*.lean")):
        code = strip_lean(p.read_text(encoding="utf-8"))
        for kind, name in DECL.findall(code):
            items.append((KIND_OF[kind], name, str(p)))
    return items


def base_name(name: str) -> str:
    """Strip Lean aux suffixes: foo._proof_1 / foo._auxLemma.x -> foo"""
    return name.split("._")[0]


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    audit_path, manifest_path = sys.argv[1], sys.argv[2]
    source_root = Path(sys.argv[3]) if len(sys.argv) > 3 else Path("TSCP/Formal")

    entries = []
    for line in open(audit_path, encoding="utf-8"):
        m = re.match(r"AUDIT\|([^|]+)\|([^|]+)\|([^|]+)\|(.*)", line.strip())
        if m:
            entries.append({"kind": m.group(1), "name": m.group(2),
                            "class": m.group(3),
                            "deps": [d for d in m.group(4).split(",") if d]})
    manifest = json.load(open(manifest_path, encoding="utf-8"))
    sorry_allowed = set(manifest["sorryAx_allowed"])
    axioms = manifest["project_axioms"]

    errors = []
    if not entries:
        print("GATE: no AUDIT lines; audit produced no output", file=sys.stderr)
        return 1
    thm_entries = [e for e in entries if e["kind"] == "theorem"]
    if not thm_entries:
        errors.append("zero theorem-kind constants in audit; corpus not in environment")

    # ---- suffix index for census reconciliation ----
    by_suffix = {}
    for e in entries:
        by_suffix.setdefault(e["name"].split(".")[-1], []).append(e)

    # ---- G3: every source declaration must be present in the environment ----
    src_items = census(source_root)
    anon = 0
    classified_theorems = []
    for kind, name, f in src_items:
        if not name:
            anon += 1
            continue
        cands = [e for e in by_suffix.get(name.split(".")[-1], [])
                 if e["name"].endswith("." + name) or e["name"] == name]
        if not cands:
            errors.append(f"DECLARED BUT NOT IN AUDITED ENVIRONMENT: {kind} {name} ({f})")
        elif len(cands) > 1 and kind == "theorem":
            errors.append(f"AMBIGUOUS NAME MATCH: {kind} {name} ({f}) -> {[c['name'] for c in cands]}")
        elif cands and kind == "theorem":
            classified_theorems.append((name, cands[0]["class"]))

    # ---- G1/G2: gaps only where declared ----
    for e in entries:
        if e["kind"] not in ("theorem", "def", "axiom"):
            continue
        b = base_name(e["name"])
        if e["class"] == "sorry":
            if b not in sorry_allowed:
                errors.append(f"UNEXPECTED sorryAx dependency: {e['name']} (deps: {e['deps']})")
        elif e["class"] == "project-axiom":
            for dep in [d for d in e["deps"] if d.startswith("TSCP.")]:
                if dep not in axioms:
                    errors.append(f"UNDECLARED project axiom: {dep} (used by {e['name']})")
                elif b not in axioms[dep]:
                    errors.append(f"UNEXPECTED project-axiom dependency: {e['name']} -> {dep}")

    # ---- G2b: expected gaps must actually exist ----
    for s in sorry_allowed:
        if not any(e["class"] == "sorry" and base_name(e["name"]) == s
                   for e in entries if e["kind"] in ("theorem", "def", "axiom")):
            errors.append(f"EXPECTED sorryAx-dependent constant missing from audit: {s}")
    for a in axioms:
        if not any(e["kind"] == "axiom" and e["name"] == a for e in entries):
            errors.append(f"EXPECTED project axiom missing from audit: {a}")

    # ---- receipt (all counts computed) ----
    kinds = Counter(e["kind"] for e in entries)
    thm_cls = Counter(c for _, c in classified_theorems)
    src_cls = Counter(k for k, _, _ in src_items)
    th_sorry = sorted(n for n, c in classified_theorems if c == "sorry")
    th_proj = sorted((n, c) for n, c in classified_theorems if c == "project-axiom")
    th_classical = sorted(n for n, c in classified_theorems if c == "classical")

    print("## Kernel-Level Dependency Audit (computed from audit output)")
    print()
    print(f"- Source-declared constants (comment-stripped census): "
          f"theorems {src_cls['theorem']}, axioms {src_cls['axiom']}, "
          f"defs/instances {src_cls['def']}, structures/inductives {src_cls['inductive']}"
          + (f" (+{anon} anonymous)" if anon else ""))
    print(f"- Environment constants audited: **{len(entries)}** "
          f"(theorem-kind {kinds['theorem']}, defs {kinds['def']}, axioms {kinds['axiom']}, "
          f"generated {sum(v for k, v in kinds.items() if k in ('inductive','ctor','recursor','quot','other'))})")
    print(f"- Source theorem declarations classified: **{len(classified_theorems)}** — "
          f"clean {thm_cls['clean']}, foundational-only {thm_cls['foundational']}, "
          f"classical {thm_cls['classical']}, project-axiom {thm_cls['project-axiom']}, "
          f"sorry-dependent {thm_cls['sorry']}")
    if th_sorry:
        print(f"- sorryAx-dependent source theorems ({len(th_sorry)}): {', '.join(th_sorry)}")
    if th_proj:
        print(f"- Project-axiom-dependent source theorems ({len(th_proj)}): "
              + ", ".join(f"{n} → {c}" for n, c in th_proj))
    if th_classical:
        print(f"- Classical source theorems ({len(th_classical)}): {', '.join(th_classical)}")
    seen_axioms = sorted(e["name"] for e in entries if e["kind"] == "axiom"
                         and e["name"].startswith("TSCP."))
    print(f"- Project axioms present in environment ({len(seen_axioms)}): "
          + (", ".join(seen_axioms) if seen_axioms else "none"))
    print()
    if errors:
        print("**GATE FAILED:**")
        for err in errors:
            print(f"- {err}")
        for err in errors:
            print("GATE:", err, file=sys.stderr)
        return 1
    print("**GATE PASSED:** census fully reconciled with the audited environment; "
          "every sorryAx and project-axiom dependency matches audit/expected-gaps.json; "
          "no undeclared, renamed, or missing axioms.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
