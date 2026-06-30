pub mod deep_ali;
pub mod edia;
pub mod owsl_bridge;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, get},
    Router,
};
use owsl_bridge::owsl_permits_verification;
use p3_baby_bear::BabyBear;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Semaphore;

use oracle_layer::folded::FoldedOracleBuilder;
use oracle_layer::oracle::MleOracle;
use p3_baby_bear::Poseidon2BabyBear;
use p3_challenger::{CanObserve, CanSample, DuplexChallenger};
use p3_field::PrimeCharacteristicRing;
use p3_field::PrimeField64;
use prover_server::proof_envelope::ProofEnvelope;
use serde::{Deserialize, Serialize};

type F = BabyBear;

type Perm = Poseidon2BabyBear<16>;
type Challenger = DuplexChallenger<F, Perm, 16, 8>;

#[derive(Clone)]
struct AppState {
    proving_permits: Arc<Semaphore>,
    pub edia_agent: std::sync::Arc<tokio::sync::Mutex<edia::EdiaAgent>>,
}

#[derive(Deserialize)]
struct ProofRequest {
    job_id: String,
    col0: Vec<u32>,
    col1: Vec<u32>,
    alpha: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct SumcheckProof {
    /// The prover's claim: the total sum of the folded oracle over the
    /// full Boolean hypercube. This is what sumcheck proves, round by
    /// round, without the verifier ever summing the oracle directly.
    claimed_sum: F,
    /// One (g(0), g(1)) pair per variable, in order.
    rounds: Vec<(F, F)>,
}

#[derive(Serialize)]
struct SealedProofResponse {
    job_id: String,
    envelope: ProofEnvelope,
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
    let pcs = commitment::new_tscp_pcs(
        dft,
        input_mmcs,
        batch_merkle::new_batch_merkle(),
        fri_params,
    );
    let _ = &pcs; // PCS constructed for future opening/commitment phase; not yet used by prove_handler.
    let state = AppState {
        proving_permits: Arc::new(Semaphore::new(4)), // max 4 concurrent proofs; tune as needed
        edia_agent: std::sync::Arc::new(tokio::sync::Mutex::new(edia::EdiaAgent::new(16))),
    };
    let app = Router::new()
        .route("/prove/sumcheck", post(prove_handler))
    .route("/metrics", get(metrics_handler))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");
    println!("TSCP Prover Server listening on {}", addr);
    axum::serve(listener, app).await.expect("Server failed");
}

fn prover_round(oracle: &impl MleOracle<F>, prefix: &[F]) -> [F; 2] {
    let n_vars = oracle.n_vars();
    let remaining = n_vars - prefix.len() - 1;
    let half = 1usize << remaining;

    let sum_bit = |bit: F| -> F {
        (0..half)
            .map(|i| {
                let mut pt = prefix.to_vec();
                pt.push(bit);
                for b in 0..remaining {
                    pt.push(if (i >> b) & 1 == 1 { F::ONE } else { F::ZERO });
                }
                oracle.eval(&pt)
            })
            .fold(F::ZERO, |a, b| a + b)
    };
    [sum_bit(F::ZERO), sum_bit(F::ONE)]
}

/// Verifies a sumcheck proof's round-to-round consistency: that each
/// round's g(0) + g(1) equals the running claim from the previous
/// round (or the prover's initial claimed_sum, for round 0), and that
/// the challenge in each round is re-derived from a transcript seeded
/// identically to the prover's.
///
/// Full soundness: verifies round-to-round consistency AND checks the
/// final running claim against the actual MLE oracle eval at the full
/// challenge vector. Closed in commit 6212db90 -- a malicious prover
/// cannot lie on the last round.
#[allow(dead_code)]
fn sumcheck_verify(
    proof: &SumcheckProof,
    challenger: &mut Challenger,
    oracle: &impl oracle_layer::oracle::MleOracle<F>,
) -> bool {
    let mut running_claim = proof.claimed_sum;
    let mut challenges: Vec<F> = Vec::with_capacity(proof.rounds.len());

    for &(g0, g1) in &proof.rounds {
        if g0 + g1 != running_claim {
            return false;
        }

        challenger.observe(g0);
        challenger.observe(g1);
        let r: F = challenger.sample();

        running_claim = g0 + r * (g1 - g0);
        challenges.push(r);
    }

    // Final binding check: the last running claim must equal the oracle
    // evaluated at the full challenge vector. This closes the soundness
    // gap — a malicious prover can no longer lie on the last round.
    let oracle_eval = oracle.eval(&challenges);
    running_claim == oracle_eval
}

async fn prove_handler(
    State(state): State<AppState>,
    Json(req): Json<ProofRequest>,
) -> impl IntoResponse {
    let _permit = match state.proving_permits.try_acquire() {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({ "error": "prover at capacity, retry later" })),
            )
                .into_response();
        }
    };

    let start = std::time::Instant::now();

    // Phase 1 admission accounting: semaphore is the authority.
    let _edia_guard = {
        let mut agent = state.edia_agent.lock().await;
        agent.pending_requests += 1;
        edia::EdiaGuard::new(state.edia_agent.clone())
    };

    if !owsl_permits_verification() {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "OWSL entropy gate denied verification" })),
        )
            .into_response();
    }

    let n_vars = req.col0.len().ilog2() as usize;
    let col0: Vec<F> = req.col0.iter().map(|&v| F::from_u32(v)).collect();
    let col1: Vec<F> = req.col1.iter().map(|&v| F::from_u32(v)).collect();
    let alpha = F::from_u32(req.alpha);

    let mut challenger = {
        use p3_baby_bear::default_babybear_poseidon2_16;
        Challenger::new(default_babybear_poseidon2_16())
    };
    for &v in &col0 {
        challenger.observe(v);
    }
    for &v in &col1 {
        challenger.observe(v);
    }

    let folded = FoldedOracleBuilder::new(vec![col0, col1], n_vars)
        .absorb_challenge(alpha)
        .build();

    let claimed_sum: F = (0..(1usize << n_vars))
        .map(|idx| {
            let point: Vec<F> = (0..n_vars)
                .map(|b| if (idx >> b) & 1 == 1 { F::ONE } else { F::ZERO })
                .collect();
            folded.eval(&point)
        })
        .fold(F::ZERO, |a, b| a + b);

    let mut prefix: Vec<F> = Vec::new();
    let mut rounds = Vec::with_capacity(n_vars);

    for _ in 0..n_vars {
        let [g0, g1] = prover_round(&folded, &prefix);

        challenger.observe(g0);
        challenger.observe(g1);
        let r: F = challenger.sample();

        rounds.push((g0, g1));
        prefix.push(r);
    }

    let proof = SumcheckProof {
        claimed_sum,
        rounds,
    };

    let payload = match serde_json::to_vec(&proof) {
        Ok(bytes) => bytes,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let claim_u64: u64 = claimed_sum.as_canonical_u64();
    let envelope = ProofEnvelope::seal_061(claim_u64, payload);

    (
    state.metrics.total_proof_time_ns.fetch_add(start.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
        StatusCode::OK,
        Json(SealedProofResponse {
            job_id: req.job_id,
            envelope,
            status: "success".to_string(),
        }),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_baby_bear::default_babybear_poseidon2_16;

    fn fresh_challenger() -> Challenger {
        Challenger::new(default_babybear_poseidon2_16())
    }

    /// Runs the real prover logic in-process (without going through
    /// axum/HTTP) over the given columns, returning the SumcheckProof.
    fn run_prover(col0: Vec<u32>, col1: Vec<u32>, alpha: u32) -> SumcheckProof {
        let n_vars = col0.len().ilog2() as usize;
        let col0: Vec<F> = col0.iter().map(|&v| F::from_u32(v)).collect();
        let col1: Vec<F> = col1.iter().map(|&v| F::from_u32(v)).collect();
        let alpha = F::from_u32(alpha);

        let mut challenger = fresh_challenger();
        for &v in &col0 {
            challenger.observe(v);
        }
        for &v in &col1 {
            challenger.observe(v);
        }

        let folded = FoldedOracleBuilder::new(vec![col0, col1], n_vars)
            .absorb_challenge(alpha)
            .build();

        let claimed_sum: F = (0..(1usize << n_vars))
            .map(|idx| {
                let point: Vec<F> = (0..n_vars)
                    .map(|b| if (idx >> b) & 1 == 1 { F::ONE } else { F::ZERO })
                    .collect();
                folded.eval(&point)
            })
            .fold(F::ZERO, |a, b| a + b);

        let mut prefix: Vec<F> = Vec::new();
        let mut rounds = Vec::with_capacity(n_vars);
        for _ in 0..n_vars {
            let [g0, g1] = prover_round(&folded, &prefix);
            challenger.observe(g0);
            challenger.observe(g1);
            let r: F = challenger.sample();
            rounds.push((g0, g1));
            prefix.push(r);
        }

        SumcheckProof {
            claimed_sum,
            rounds,
        }
    }

    fn make_oracle(
        col0: Vec<u32>,
        col1: Vec<u32>,
        alpha: u32,
    ) -> oracle_layer::folded::FoldedOracle<F> {
        let n_vars = col0.len().ilog2() as usize;
        let col0: Vec<F> = col0.iter().map(|&v| F::from_u32(v)).collect();
        let col1: Vec<F> = col1.iter().map(|&v| F::from_u32(v)).collect();
        oracle_layer::folded::FoldedOracleBuilder::new(vec![col0, col1], n_vars)
            .absorb_challenge(F::from_u32(alpha))
            .build()
    }

    #[test]
    fn honest_proof_verifies() {
        let proof = run_prover(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);
        let oracle = make_oracle(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);
        let mut verifier_challenger = fresh_challenger();
        for &v in &[
            F::from_u32(1),
            F::from_u32(2),
            F::from_u32(3),
            F::from_u32(4),
        ] {
            verifier_challenger.observe(v);
        }
        for &v in &[
            F::from_u32(5),
            F::from_u32(6),
            F::from_u32(7),
            F::from_u32(8),
        ] {
            verifier_challenger.observe(v);
        }
        assert!(
            sumcheck_verify(&proof, &mut verifier_challenger, &oracle),
            "an honestly generated sumcheck proof must verify against a freshly-seeded transcript"
        );
    }

    #[test]
    fn tampered_round_message_fails_verification() {
        let mut proof = run_prover(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);
        proof.rounds[0].0 = proof.rounds[0].0 + F::ONE;

        let mut verifier_challenger = fresh_challenger();
        for &v in &[
            F::from_u32(1),
            F::from_u32(2),
            F::from_u32(3),
            F::from_u32(4),
        ] {
            verifier_challenger.observe(v);
        }
        for &v in &[
            F::from_u32(5),
            F::from_u32(6),
            F::from_u32(7),
            F::from_u32(8),
        ] {
            verifier_challenger.observe(v);
        }
        let oracle = make_oracle(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);
        assert!(
            !sumcheck_verify(&proof, &mut verifier_challenger, &oracle),
            "a tampered round message must break the g(0)+g(1) == running_claim invariant and fail"
        );
    }

    #[test]
    fn wrong_claimed_sum_fails_verification() {
        let mut proof = run_prover(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);
        proof.claimed_sum = proof.claimed_sum + F::ONE;

        let mut verifier_challenger = fresh_challenger();
        for &v in &[
            F::from_u32(1),
            F::from_u32(2),
            F::from_u32(3),
            F::from_u32(4),
        ] {
            verifier_challenger.observe(v);
        }
        for &v in &[
            F::from_u32(5),
            F::from_u32(6),
            F::from_u32(7),
            F::from_u32(8),
        ] {
            verifier_challenger.observe(v);
        }
        let oracle = make_oracle(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);
        assert!(
            !sumcheck_verify(&proof, &mut verifier_challenger, &oracle),
            "a claimed_sum that doesn't match round 0's g(0)+g(1) must fail immediately"
        );
    }

    #[test]
    fn verifier_with_wrong_transcript_seed_fails() {
        // If the verifier doesn't observe the same public inputs the
        // prover did, it will derive different challenges and the
        // proof will (almost certainly) fail to verify, since g(r) for
        // the wrong r generally won't match the next round's claim.
        let proof = run_prover(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);

        let mut wrong_challenger = fresh_challenger();
        for &v in &[
            F::from_u32(99),
            F::from_u32(98),
            F::from_u32(97),
            F::from_u32(96),
        ] {
            wrong_challenger.observe(v);
        }
        for &v in &[
            F::from_u32(5),
            F::from_u32(6),
            F::from_u32(7),
            F::from_u32(8),
        ] {
            wrong_challenger.observe(v);
        }
        let oracle = make_oracle(vec![1, 2, 3, 4], vec![5, 6, 7, 8], 3);
        assert!(
            !sumcheck_verify(&proof, &mut wrong_challenger, &oracle),
            "verifying with a transcript seeded from different public inputs must fail"
        );
    }
}

async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    use std::sync::atomic::Ordering;
    let m = &state.metrics;
    let admitted = m.admitted.load(Ordering::Relaxed);
    let completed = m.completed.load(Ordering::Relaxed);
    let rejected = m.rejected.load(Ordering::Relaxed);
    let inflight = m.inflight.load(Ordering::Relaxed);
    let peak = m.peak.load(Ordering::Relaxed);
    let total_ns = m.total_proof_time_ns.load(Ordering::Relaxed);
    let avg_ms = if completed > 0 { total_ns / completed / 1_000_000 } else { 0 };
    Json(serde_json::json!({
        "admitted": admitted,
        "completed": completed,
        "rejected": rejected,
        "inflight": inflight,
        "peak_concurrent": peak,
        "avg_proof_ms": avg_ms,
        "invariant_holds": admitted == completed + inflight + rejected
    }))
}
