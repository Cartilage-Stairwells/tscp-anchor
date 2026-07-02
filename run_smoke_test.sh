#!/usr/bin/env bash
set -euo pipefail

command -v jq >/dev/null 2>&1 || {
    echo "[ERROR] jq is required for smoke test validation."
    exit 1
}

cargo run -p prover-server > server.log 2>&1 &
SERVER_PID=$!
trap 'code=$?; if [ $code -ne 0 ]; then echo "[INFO] Non-zero exit ($code) -- server.log:"; cat server.log; fi; kill "$SERVER_PID" 2>/dev/null || true' EXIT
echo "[INFO] Prover server starting (PID $SERVER_PID), waiting for port 3030..."

for i in $(seq 1 180); do
    if curl -s -o /dev/null http://localhost:3030/prove/sumcheck; then
        break
    fi
    if ! kill -0 "$SERVER_PID" 2>/dev/null; then
        echo "[ERROR] Server process exited before binding. Log:"
        cat server.log
        exit 1
    fi
    sleep 1
done

echo "[INFO] Sending smoke test request..."
RESPONSE=$(curl -s -X POST http://localhost:3030/prove/sumcheck \
     -H "Content-Type: application/json" \
     -d "{\"job_id\": \"smoke-test-1\", \"col0\": [1, 2, 3, 4], \"col1\": [5, 6, 7, 8], \"alpha\": 3}")

echo "$RESPONSE"

echo "$RESPONSE" | jq . >/dev/null || {
    echo "[FAIL] invalid JSON response"
    exit 1
}

CLAIM=$(echo "$RESPONSE" | jq -r '.envelope.claim')
VERSION=$(echo "$RESPONSE" | jq -r '.envelope.version')
SEMVER=$(echo "$RESPONSE" | jq -c '.envelope.plonky3_semver')
STATUS_FIELD=$(echo "$RESPONSE" | jq -r '.status')

FAIL=0
[ "$CLAIM" = "88" ] || { echo "[FAIL] expected claim=88, got claim=$CLAIM"; FAIL=1; }
[ "$VERSION" = "V0_6_1" ] || { echo "[FAIL] expected version=V0_6_1, got version=$VERSION"; FAIL=1; }
[ "$SEMVER" = "[0,6,1]" ] || { echo "[FAIL] expected plonky3_semver=[0,6,1], got $SEMVER"; FAIL=1; }
[ "$STATUS_FIELD" = "success" ] || { echo "[FAIL] expected status=success, got status=$STATUS_FIELD"; FAIL=1; }

if [ "$FAIL" -eq 1 ]; then
    exit 1
fi

echo "[PASS] golden claim, version, semver, and status all match"
echo -e "\n[INFO] Server stopped. Check server.log if errors occurred."
