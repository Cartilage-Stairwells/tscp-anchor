#!/data/data/com.termux/files/usr/bin/bash
set -e

if grep -nE \
"function .*verify.*\{[[:space:]]*return true|=>[[:space:]]*true" \
contracts/TSCPFriVerifier.sol
then
  echo "FAIL: unconditional verifier success"
  exit 1
fi

echo "PASS: verifier stubs fail closed"
