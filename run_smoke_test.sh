#!/usr/bin/env bash
# 1. Start server in background
cargo run -p prover-server > server.log 2>&1 &
SERVER_PID=$!
echo "[INFO] Prover server started with PID $SERVER_PID"

# Give the server a moment to bind
sleep 2

# 2. Send a dummy request
echo "[INFO] Sending smoke test request..."
curl -X POST http://localhost:3000/prove/sumcheck \
     -H "Content-Type: application/json" \
     -d '{"transcript_data": [1, 2, 3], "public_parameters": [4, 5, 6]}' \
     -v

# 3. Cleanup
kill $SERVER_PID
echo -e "\n[INFO] Server stopped. Check server.log if errors occurred."
