import subprocess
import sys
from pathlib import Path
import json
import time

# -----------------------------
# HARD GATES (NO FALLBACKS)
# -----------------------------

REQUIRED_MARKER = ".ccl-release-root"


def run(cmd):
    print(f"\n>> {cmd}")
    result = subprocess.run(cmd, shell=True)
    if result.returncode != 0:
        print(f"FAILED: {cmd}")
        sys.exit(1)


def check_clean_git():
    out = subprocess.check_output(["git", "status", "--porcelain"]).decode()
    if out.strip():
        print("ERROR: working tree not clean")
        print(out)
        sys.exit(1)


def check_marker():
    if not Path(REQUIRED_MARKER).exists():
        print("ERROR: not in CCL release root")
        sys.exit(1)


def check_repo():
    remotes = subprocess.check_output(["git", "remote", "-v"]).decode()
    if "origin" not in remotes:
        print("ERROR: no git remote")
        sys.exit(1)


def check_build():
    run("npm run build")


def check_tests():
    run("npm test")


def dry_run_publish():
    run("npm publish --dry-run")


# -----------------------------
# RELEASE PIPELINE
# -----------------------------

def main():
    print("\n=== CCL RELEASE PIPELINE START ===")

    check_marker()
    check_repo()
    check_clean_git()

    check_tests()
    check_build()
    dry_run_publish()

    branch = subprocess.check_output(
        ["git", "branch", "--show-current"]
    ).decode().strip()

    print(f"\nRelease branch: {branch}")

    tag = f"ccl-release-{int(time.time())}"
    run(f"git tag {tag}")

    run("git push origin --tags")
    run("git push origin " + branch)

    print("\n=== RELEASE READY ===")
    print(json.dumps({
        "status": "ok",
        "tag": tag,
        "branch": branch
    }, indent=2))


if __name__ == "__main__":
    main()
