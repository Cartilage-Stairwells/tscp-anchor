#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "=== CLAIM SURFACE CHECK ==="

FAIL=0

grep -R -nE \
"production STARK|production ZK|verified proof|secure proof|sound proof" \
README.md RELEASE_NOTES.md specs contracts crates \
2>/dev/null |
while IFS=: read -r file line text; do

  block=$(sed -n "$((line-3)),$((line+1))p" "$file")

  if echo "$block" | grep -qE \
  "does not claim|not part of|not a production|not production|placeholder|boundary|experimental|extension layer"
  then
    continue
  fi

  echo "SUSPICIOUS: $file:$line:$text"
  FAIL=1

done

if [ "$FAIL" -eq 1 ]; then
  echo "FAIL: suspicious claim surface"
  exit 1
fi

echo "PASS: claim surface bounded"
