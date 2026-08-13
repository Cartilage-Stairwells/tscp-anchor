//! Evidence-mode Criterion benchmark for the TSCP oracle-layer.
//!
//! This bench calls the real prove/verify pipeline through the oracle bridge.
//! It produces two measurement types:
//!   - integrity: prove() + verify() (full pipeline — NOT for speedup claims)
//!   - performance: prove() ONLY, verify() outside timed region (USE for speedup)
//!
//! Layer: end_to_end
//! Claim boundary: "oracle-layer end-to-end prover improved Y%"
//! NOT a kernel-level claim (AVX-512 butterfly benchmarks live separately).
//!
//! Evidence-mode constraints:
//!   - Build with explicit target-cpu (NOT native)
//!   - Correctness gate must pass before any timing is accepted
//!   - Results are written to canonical JSON for the evidence bundle

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use std::fs;
use std::time::Instant;

use tscp_verifier::oracle_bridge;
use tscp_verifier::timing::phases;

use p3_baby_bear::BabyBear;

// ─── Configuration ───────────────────────────────────────────────────────────

const TRACE_SIZES: &[usize] = &[256, 1024, 4096, 16384];
const NUM_QUERIES: usize = 20;
const OUTPUT_PATH: &str = "benchmark_results.json";

// ─── Results collection ──────────────────────────────────────────────────────

#[derive(serde::Serialize)]
struct BenchResults {
    bench_name: String,
    layer: String,
    num_queries: usize,
    runs: Vec<RunEntry>,
}

#[derive(serde::Serialize)]
struct RunEntry {
    trace_size: usize,
    measurement_type: String,
    median_ms: f64,
    min_ms: f64,
    max_ms: f64,
    stddev_ms: f64,
    sample_count: usize,
    // Phase-level breakdown from PhaseTimer (mean across samples)
    phase_proving_total_ms: f64,
    phase_transcript_generation_ms: f64,
    phase_fri_ms: f64,
    phase_verification_ms: f64,
    // Correctness
    verification_passed: bool,
    // Sizes
    proof_size_bytes: usize,
    transcript_size_bytes: usize,
    fiat_shamir_rounds: u32,
}

// ─── Test data generation ────────────────────────────────────────────────────

/// Generate test evaluations and domain for a given trace size.
/// Uses a deterministic pattern so results are reproducible.
fn generate_trace(trace_size: usize) -> (Vec<BabyBear>, Vec<BabyBear>) {
    let evals: Vec<BabyBear> = (1..=trace_size).map(|i| BabyBear::new(i as u32)).collect();
    let domain: Vec<BabyBear> = (1..=trace_size)
        .map(|i| BabyBear::new((i * 5 + 1) as u32))
        .collect();
    (evals, domain)
}

// ─── Phase timing collector ──────────────────────────────────────────────────

/// Collects phase timings across multiple prove/verify runs.
struct PhaseCollector {
    proving_total: Vec<f64>,
    transcript_generation: Vec<f64>,
    fri: Vec<f64>,
    verification: Vec<f64>,
    proof_sizes: Vec<usize>,
    transcript_sizes: Vec<usize>,
    fiat_shamir_rounds: Vec<u32>,
    verification_passed: bool,
}

impl PhaseCollector {
    fn new() -> Self {
        Self {
            proving_total: Vec::new(),
            transcript_generation: Vec::new(),
            fri: Vec::new(),
            verification: Vec::new(),
            proof_sizes: Vec::new(),
            transcript_sizes: Vec::new(),
            fiat_shamir_rounds: Vec::new(),
            verification_passed: false,
        }
    }

    fn record_prove(
        &mut self,
        timer: &tscp_verifier::timing::PhaseTimer,
        proof_bytes: usize,
        transcript_bytes: usize,
        fs_rounds: u32,
    ) {
        self.proving_total
            .push(timer.get_or_zero(phases::PROVING_TOTAL));
        self.transcript_generation
            .push(timer.get_or_zero(phases::TRANSCRIPT_GENERATION));
        self.fri.push(timer.get_or_zero(phases::FRI));
        self.proof_sizes.push(proof_bytes);
        self.transcript_sizes.push(transcript_bytes);
        self.fiat_shamir_rounds.push(fs_rounds);
    }

    fn record_verify(&mut self, timer: &tscp_verifier::timing::PhaseTimer, passed: bool) {
        self.verification
            .push(timer.get_or_zero(phases::VERIFICATION));
        self.verification_passed = passed;
    }

    fn mean(data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        data.iter().sum::<f64>() / data.len() as f64
    }

    fn last_or_zero(data: &[usize]) -> usize {
        *data.last().unwrap_or(&0)
    }

    fn last_or_zero_u32(data: &[u32]) -> u32 {
        *data.last().unwrap_or(&0)
    }
}

// ─── Benchmarks ──────────────────────────────────────────────────────────────

fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);
    group.warm_up_time(std::time::Duration::from_secs(3));

    let mut results = BenchResults {
        bench_name: "evidence_baseline".to_string(),
        layer: "end_to_end".to_string(),
        num_queries: NUM_QUERIES,
        runs: Vec::new(),
    };

    for &trace_size in TRACE_SIZES {
        let (evals_template, domain_template) = generate_trace(trace_size);

        // ── INTEGRITY benchmark: prove + verify (full pipeline) ─────────
        let mut collector = PhaseCollector::new();

        group.bench_with_input(
            BenchmarkId::new("integrity", trace_size),
            &trace_size,
            |b, &_ts| {
                b.iter(|| {
                    let evals = evals_template.clone();
                    let domain = domain_template.clone();

                    // Prove
                    let prove_result = oracle_bridge::prove_instrumented_internal(
                        evals,
                        domain.clone(),
                        NUM_QUERIES,
                    )
                    .expect("prove should succeed");

                    collector.record_prove(
                        &prove_result.timer,
                        prove_result.proof_bytes.len(),
                        prove_result.transcript_bytes.len(),
                        prove_result.fiat_shamir_rounds,
                    );

                    // Verify
                    let verify_result = oracle_bridge::verify_instrumented_with_proof(
                        &domain,
                        &prove_result.proof,
                        NUM_QUERIES,
                    )
                    .expect("verify should succeed");

                    collector.record_verify(&verify_result.timer, verify_result.verification_ok);

                    // Correctness gate: panic if verification fails
                    assert!(
                        verify_result.verification_ok,
                        "CORRECTNESS GATE FAILED at trace_size={}: verify() returned false.",
                        trace_size
                    );
                });
            },
        );

        // Collect Criterion statistics from the last Criterion run
        // Criterion doesn't expose per-bench stats directly, but we have the PhaseTimer data.
        // We'll compute stats from the PhaseTimer samples.
        let (median, min, max, stddev) = stats(&collector.proving_total);

        results.runs.push(RunEntry {
            trace_size,
            measurement_type: "integrity".to_string(),
            median_ms: median,
            min_ms: min,
            max_ms: max,
            stddev_ms: stddev,
            sample_count: collector.proving_total.len(),
            phase_proving_total_ms: PhaseCollector::mean(&collector.proving_total),
            phase_transcript_generation_ms: PhaseCollector::mean(&collector.transcript_generation),
            phase_fri_ms: PhaseCollector::mean(&collector.fri),
            phase_verification_ms: PhaseCollector::mean(&collector.verification),
            verification_passed: collector.verification_passed,
            proof_size_bytes: PhaseCollector::last_or_zero(&collector.proof_sizes),
            transcript_size_bytes: PhaseCollector::last_or_zero(&collector.transcript_sizes),
            fiat_shamir_rounds: PhaseCollector::last_or_zero_u32(&collector.fiat_shamir_rounds),
        });

        // ── PERFORMANCE benchmark: prove ONLY, verify outside ────────────
        let mut perf_collector = PhaseCollector::new();

        group.bench_with_input(
            BenchmarkId::new("performance", trace_size),
            &trace_size,
            |b, &ts| {
                b.iter_custom(|iters| {
                    let mut total = std::time::Duration::ZERO;
                    for _ in 0..iters {
                        let evals = evals_template.clone();
                        let domain = domain_template.clone();

                        // ── TIMED REGION: prove() only ────────────────────
                        let t_start = Instant::now();
                        let prove_result = oracle_bridge::prove_instrumented_internal(
                            evals,
                            domain.clone(),
                            NUM_QUERIES,
                        )
                        .expect("prove should succeed");
                        total += t_start.elapsed();
                        // ── END TIMED REGION ──────────────────────────────

                        // Verify OUTSIDE timed region — must pass
                        let verify_result = oracle_bridge::verify_instrumented_with_proof(
                            &domain,
                            &prove_result.proof,
                            NUM_QUERIES,
                        )
                        .expect("verify should succeed");

                        assert!(
                            verify_result.verification_ok,
                            "CORRECTNESS GATE FAILED (performance) at trace_size={}",
                            ts
                        );

                        perf_collector.record_prove(
                            &prove_result.timer,
                            prove_result.proof_bytes.len(),
                            prove_result.transcript_bytes.len(),
                            prove_result.fiat_shamir_rounds,
                        );
                        perf_collector
                            .record_verify(&verify_result.timer, verify_result.verification_ok);
                    }
                    total
                });
            },
        );

        let (median, min, max, stddev) = stats(&perf_collector.proving_total);

        results.runs.push(RunEntry {
            trace_size,
            measurement_type: "performance".to_string(),
            median_ms: median,
            min_ms: min,
            max_ms: max,
            stddev_ms: stddev,
            sample_count: perf_collector.proving_total.len(),
            phase_proving_total_ms: PhaseCollector::mean(&perf_collector.proving_total),
            phase_transcript_generation_ms: PhaseCollector::mean(
                &perf_collector.transcript_generation,
            ),
            phase_fri_ms: PhaseCollector::mean(&perf_collector.fri),
            phase_verification_ms: PhaseCollector::mean(&perf_collector.verification),
            verification_passed: perf_collector.verification_passed,
            proof_size_bytes: PhaseCollector::last_or_zero(&perf_collector.proof_sizes),
            transcript_size_bytes: PhaseCollector::last_or_zero(&perf_collector.transcript_sizes),
            fiat_shamir_rounds: PhaseCollector::last_or_zero_u32(
                &perf_collector.fiat_shamir_rounds,
            ),
        });
    }

    group.finish();

    // Write results JSON
    let json = serde_json::to_string_pretty(&results).expect("serialize results");
    fs::write(OUTPUT_PATH, &json).expect("write results");
    eprintln!("\nBenchmark results written to {}", OUTPUT_PATH);
}

fn stats(samples: &[f64]) -> (f64, f64, f64, f64) {
    if samples.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mut s = samples.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = s[0];
    let max = *s.last().unwrap();
    let median = if s.len() % 2 == 0 {
        (s[s.len() / 2 - 1] + s[s.len() / 2]) / 2.0
    } else {
        s[s.len() / 2]
    };
    let mean = s.iter().sum::<f64>() / s.len() as f64;
    let stddev = (s.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / s.len() as f64).sqrt();
    (median, min, max, stddev)
}

criterion_group!(benches, bench_end_to_end);
criterion_main!(benches);
