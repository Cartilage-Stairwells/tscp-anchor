use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use serde::Serialize;
use uuid::Uuid;

use prometheus::{
    Encoder, HistogramVec, IntCounterVec, IntGauge, Opts, Registry, TextEncoder,
};

#[derive(Clone, Debug)]
pub struct TraceContext {
    pub request_id: String,
}

impl TraceContext {
    pub fn new() -> Self {
        Self { request_id: Uuid::new_v4().to_string() }
    }
}

pub struct PhaseTimer {
    name: &'static str,
    start: Instant,
    ctx: TraceContext,
    recorder: Arc<Telemetry>,
}

impl PhaseTimer {
    pub fn new(name: &'static str, ctx: TraceContext, recorder: Arc<Telemetry>) -> Self {
        Self { name, start: Instant::now(), ctx, recorder }
    }

    pub fn stop(self) {
        let elapsed = self.start.elapsed();
        self.recorder.record_phase(&self.ctx, self.name, elapsed);
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ProverEvent {
    pub request_id: String,
    pub phase: String,
    pub duration_ms: u128,
    pub trace_size: Option<usize>,
    pub constraint_count: Option<usize>,
}

pub struct Telemetry {
    registry: Registry,
    pub prover_requests: IntCounterVec,
    pub phase_latency: HistogramVec,
    pub trace_size: IntGauge,
    pub constraint_count: IntGauge,
    debug_log: Mutex<Vec<ProverEvent>>,
}

impl Telemetry {
    pub fn new() -> Arc<Self> {
        let registry = Registry::new();

        let prover_requests = IntCounterVec::new(
            Opts::new("prover_requests_total", "Total prover requests"),
            &["status"],
        ).unwrap();

        let phase_latency = HistogramVec::new(
            prometheus::HistogramOpts::new("phase_latency_ms", "Latency per prover phase (ms)"),
            &["phase"],
        ).unwrap();

        let trace_size = IntGauge::new("trace_rows", "Current trace size").unwrap();
        let constraint_count = IntGauge::new("constraint_count", "Number of constraints").unwrap();

        registry.register(Box::new(prover_requests.clone())).unwrap();
        registry.register(Box::new(phase_latency.clone())).unwrap();
        registry.register(Box::new(trace_size.clone())).unwrap();
        registry.register(Box::new(constraint_count.clone())).unwrap();

        Arc::new(Self {
            registry,
            prover_requests,
            phase_latency,
            trace_size,
            constraint_count,
            debug_log: Mutex::new(Vec::with_capacity(1024)),
        })
    }

    pub fn start_phase(self: &Arc<Self>, name: &'static str, ctx: TraceContext) -> PhaseTimer {
        PhaseTimer::new(name, ctx, self.clone())
    }

    fn record_phase(&self, ctx: &TraceContext, phase: &'static str, duration: Duration) {
        let ms = duration.as_millis();
        self.phase_latency.with_label_values(&[phase]).observe(ms as f64);

        let mut log = self.debug_log.lock();
        log.push(ProverEvent {
            request_id: ctx.request_id.clone(),
            phase: phase.to_string(),
            duration_ms: ms,
            trace_size: None,
            constraint_count: None,
        });
        if log.len() > 2048 {
            log.drain(0..512);
        }
    }

    pub fn gather_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let mut buffer = Vec::new();
        encoder.encode(&self.registry.gather(), &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    pub fn record_request<F, T>(self: &Arc<Self>, ctx: TraceContext, f: F) -> T
    where
        F: FnOnce(TraceContext, Arc<Telemetry>) -> T,
    {
        self.prover_requests.with_label_values(&["ok"]).inc();
        f(ctx, self.clone())
    }

    pub fn set_gauges(&self, rows: usize, constraints: usize) {
        self.trace_size.set(rows as i64);
        self.constraint_count.set(constraints as i64);
    }
}
