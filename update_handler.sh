#!/usr/bin/env bash
cat << 'MAIN' > prover-server/src/main.rs
use axum::{extract::Json, routing::post, Router, response::IntoResponse, http::StatusCode};
use std::net::SocketAddr;
use oracle_layer::api::{ProofRequest, ProofResponse};
use oracle_layer::prover::generate_sumcheck_proof; // Ensure this exists in your oracle-layer

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/prove/sumcheck", post(prove_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Server failed");
}

async fn prove_handler(Json(payload): Json<ProofRequest>) -> impl IntoResponse {
    // Invoke the verified Plonky3 prover logic
    match generate_sumcheck_proof(payload) {
        Ok(proof_data) => (
            StatusCode::OK,
            Json(ProofResponse {
                proof: proof_data,
                success: true,
                error_message: None,
            }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ProofResponse {
                proof: vec![],
                success: false,
                error_message: Some(format!("Prover failed: {:?}", e)),
            }),
        ).into_response(),
    }
}
MAIN

# Re-check the logic
cargo check -p prover-server
