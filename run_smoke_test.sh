#!/usr/bin/env bash
set -uo pipefail

cargo run -p prover-server > server.log 2>&1 &
SERVER_PID=$!
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
curl -X POST http://localhost:3030/prove/sumcheck \
     -H "Content-Type: application/json" \
     -d "{\"job_id\": \"smoke-test-1\", \"col0\": [1, 2, 3, 4], \"col1\": [5, 6, 7, 8], \"alpha\": 3}" \
     -v

kill "$SERVER_PID" 2>/dev/null
echo -e "\n[INFO] Server stopped. Check server.log if errors occurred."
