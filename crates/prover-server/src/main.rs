use axum::{extract::Json, routing::post, Router, response::IntoResponse, http::StatusCode};
use std::net::SocketAddr;
use p3_baby_bear::BabyBear;

use p3_baby_bear::Poseidon2BabyBear;
use p3_challenger::{CanObserve, CanSample, DuplexChallenger};
use p3_field::PrimeCharacteristicRing;
use oracle_layer::folded::FoldedOracleBuilder;
use oracle_layer::oracle::MleOracle;
use serde::{Deserialize, Serialize};

type F = BabyBear;


type Perm = Poseidon2BabyBear<16>;
type Challenger = DuplexChallenger<F, Perm, 16, 8>;

#[derive(Deserialize)]
struct ProofRequest {
    job_id: String,
    col0: Vec<u32>,
    col1: Vec<u32>,
    alpha: u32,
}

#[derive(Serialize)]
struct ProofResponse {
    job_id: String,
    proof: Vec<u8>,
    status: String,
}
#[tokio::main]
async fn main() {
    let dft = p3_dft::Radix2DitParallel::<F>::default();
    let input_mmcs = batch_merkle::new_batch_merkle();
    let fri_mmcs = batch_merkle::new_batch_merkle();
    let fri_params = p3_fri::FriParameters {
        log_blowup: 1,
        log_final_poly_len: 0,
        max_log_arity: 1,
        num_queries: 32,
        commit_proof_of_work_bits: 0,
        query_proof_of_work_bits: 0,
        mmcs: fri_mmcs,
    };
    let pcs = commitment::new_tscp_pcs(dft, input_mmcs, batch_merkle::new_batch_merkle(), fri_params);
    let _ = &pcs; // PCS constructed for future opening/commitment phase; not yet used by prove_handler.
    let app = Router::new().route("/prove/sumcheck", post(prove_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
    println!("TSCP Prover Server listening on {}", addr);
    axum::serve(listener, app).await.expect("Server failed");
}

fn prover_round(oracle: &impl MleOracle<F>, prefix: &[F]) -> [F; 2] {
    let n_vars = oracle.n_vars();
    let remaining = n_vars - prefix.len() - 1;
    let half = 1usize << remaining;
    
    let sum_bit = |bit: F| -> F {
        (0..half).map(|i| {
            let mut pt = prefix.to_vec();
            pt.push(bit);
            for b in 0..remaining {
                pt.push(if (i >> b) & 1 == 1 { F::ONE } else { F::ZERO });
            }
            oracle.eval(&pt)
        }).fold(F::ZERO, |a, b| a + b)
    };
    [sum_bit(F::ZERO), sum_bit(F::ONE)]
}

async fn prove_handler(Json(req): Json<ProofRequest>) -> impl IntoResponse {
    let n_vars = req.col0.len().ilog2() as usize;
    let col0: Vec<F> = req.col0.iter().map(|&v| F::from_u32(v)).collect();
    let col1: Vec<F> = req.col1.iter().map(|&v| F::from_u32(v)).collect();
    let alpha = F::from_u32(req.alpha);

    let mut challenger = {
        use p3_baby_bear::default_babybear_poseidon2_16;
        Challenger::new(default_babybear_poseidon2_16())
    };
    for &v in &col0 { challenger.observe(v); }
    for &v in &col1 { challenger.observe(v); }

    let folded = FoldedOracleBuilder::new(vec![col0, col1], n_vars)
        .absorb_challenge(alpha).build();

    let mut prefix: Vec<F> = Vec::new();
    let mut proof_data = Vec::new();

    for _ in 0..n_vars {
        let [g0, g1] = prover_round(&folded, &prefix);
        
        challenger.observe(g0);
        challenger.observe(g1);
        let r: F = challenger.sample();
        
        proof_data.extend(bincode::serialize(&(g0, g1)).unwrap());
        prefix.push(r);
    }

    (StatusCode::OK, Json(ProofResponse {
        job_id: req.job_id,
        proof: proof_data,
        status: "success".to_string(),
    }))
}




