#!/bin/bash
set -e

echo "=== TSCP Prover Setup ==="

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Create service user
useradd -r -s /bin/false tscp 2>/dev/null || true
mkdir -p /opt/tscp-prover

# Copy binary (run from repo root after cargo build --release)
cp crates/oracle-layer/target/release/prover-server /opt/tscp-prover/
chown tscp:tscp /opt/tscp-prover/prover-server
chmod +x /opt/tscp-prover/prover-server

# Install systemd service
cp deploy/tscp-prover.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable tscp-prover
systemctl start tscp-prover

# Firewall: allow port 3030
ufw allow 3030/tcp 2>/dev/null || iptables -A INPUT -p tcp --dport 3030 -j ACCEPT

echo "=== Done. Service status ==="
systemctl status tscp-prover
