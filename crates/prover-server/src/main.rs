use axum::{routing::post, Json, Router, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::task;

#[derive(Deserialize)]
struct ProofRequest {
    input: Vec<u64>,
}

#[derive(Serialize)]
struct ProofResponse {
    job_id: String,
    status: String,
}

async fn prove_sumcheck(Json(_payload): Json<ProofRequest>) -> impl IntoResponse {
    let job_id = uuid();
    task::spawn_blocking(move || {
        // proof generation goes here
    });
    (StatusCode::ACCEPTED, Json(ProofResponse { job_id, status: "queued".into() }))
}

fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos().to_string()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/prove/sumcheck", post(prove_sumcheck));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    println!("Prover server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
