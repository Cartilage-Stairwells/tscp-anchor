#!/usr/bin/env bash
set -euo pipefail

echo "[1/3] Updating oracle-layer API schema..."
mkdir -p oracle-layer/src
cat << 'API' > oracle-layer/src/api.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofRequest {
    pub transcript_data: Vec<u8>,
    pub public_parameters: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofResponse {
    pub proof: Vec<u8>,
    pub success: bool,
    pub error_message: Option<String>,
}
API

echo "[2/3] Scaffolding prover-server handler..."
mkdir -p prover-server/src
cat << 'MAIN' > prover-server/src/main.rs
use axum::{extract::Json, routing::post, Router, response::IntoResponse, http::StatusCode};
use std::net::SocketAddr;
use oracle_layer::api::{ProofRequest, ProofResponse};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/prove/sumcheck", post(prove_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Server failed");
}

async fn prove_handler(Json(payload): Json<ProofRequest>) -> impl IntoResponse {
    // Logic: Invoke the oracle-layer prover
    // Note: Ensure your oracle-layer/src/lib.rs exports the api module
    (StatusCode::OK, Json(ProofResponse {
        proof: vec![0u8; 32], // Placeholder for actual proof
        success: true,
        error_message: None,
    }))
}
MAIN

echo "[3/3] Checking integrity..."
cargo check -p prover-server
echo "Automation complete. Prover server is wired."
