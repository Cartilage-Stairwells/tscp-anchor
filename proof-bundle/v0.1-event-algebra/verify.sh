#!/bin/sh
set -e

git verify-tag v0.1-event-algebra
sha256sum -c proof-bundle/v0.1-event-algebra/files.sha256
cargo test --workspace

echo "TSCP VERIFICATION PASS"
