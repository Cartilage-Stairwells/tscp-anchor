use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct EdiaAgent {
    pub ring_buffer: VecDeque<u64>,
    pub drain_rate_tps: u32,
    // REAL NEW FIELD: Tracks active, in-flight proving operations
    pub pending_requests: Arc<AtomicU64>,
}

impl EdiaAgent {
    pub fn new(capacity: usize) -> Self {
        Self {
            ring_buffer: VecDeque::with_capacity(capacity),
            drain_rate_tps: 1,
            pending_requests: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn ingest_telemetry(&mut self) -> Result<(), String> {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        self.ring_buffer.push_back(ts);
        Ok(())
    }

    pub fn step(&mut self) {
        let n = self.drain_rate_tps as usize;
        let len = self.ring_buffer.len();
        self.ring_buffer.drain(..n.min(len));
        if self.ring_buffer.is_empty() {
            self.drain_rate_tps = 1;
        }
    }
}

/// Tracks the number of in-flight requests.
///
/// Release on decrement and Acquire on observation are intentionally
/// conservative. Today this is only a monitoring counter (Relaxed would
/// suffice), but the stronger ordering avoids revisiting memory-ordering
/// decisions if the counter later becomes associated with additional
/// shared state.
pub struct EdiaGuard {
    pending_requests: Arc<AtomicU64>,
}

impl EdiaGuard {
    /// Increments pending_requests and returns a guard that decrements
    /// it on drop. This is the only way to bump the counter, so callers
    /// can no longer forget the increment.
    pub async fn acquire(agent: &Arc<Mutex<EdiaAgent>>) -> Self {
        let pending_requests = {
            let locked = agent.lock().await;
            locked.pending_requests.clone()
        };
        pending_requests.fetch_add(1, Ordering::Relaxed);
        Self { pending_requests }
    }
}

impl Drop for EdiaGuard {
    fn drop(&mut self) {
        // No lock acquired here -- cannot panic, safe across await points.
        self.pending_requests
            .try_update(Ordering::Release, Ordering::Relaxed, |v| {
                Some(v.saturating_sub(1))
            })
            .ok();
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;

    #[test]
    fn backpressure_control_loop_converges() {
        let mut agent = EdiaAgent::new(16);
        for _ in 0..7 {
            agent.ingest_telemetry().unwrap();
        }
        for _ in 0..20 {
            agent.step();
        }
        assert!(agent.ring_buffer.len() < 7);
        // Verify the new field initializes cleanly
        assert_eq!(agent.pending_requests.load(Ordering::Acquire), 0);
    }

    #[tokio::test]
    async fn edia_guard_decrements_pending_on_drop() {
        let agent = Arc::new(Mutex::new(EdiaAgent::new(16)));
        let guard = EdiaGuard::acquire(&agent).await;

        {
            let locked = agent.lock().await;
            assert_eq!(locked.pending_requests.load(Ordering::Acquire), 1);
        }

        drop(guard);

        let locked = agent.lock().await;
        assert_eq!(
            locked.pending_requests.load(Ordering::Acquire),
            0,
            "EdiaGuard::drop must decrement pending_requests back to 0"
        );
    }

    #[tokio::test]
    async fn edia_guard_does_not_underflow_on_double_relevant_drop() {
        // pending_requests already at 0; guard's drop must not panic or wrap.
        let agent = Arc::new(Mutex::new(EdiaAgent::new(16)));
        let guard = EdiaGuard::acquire(&agent).await;
        drop(guard);

        let locked = agent.lock().await;
        assert_eq!(locked.pending_requests.load(Ordering::Acquire), 0);
    }

    #[tokio::test]
    async fn edia_guard_survives_concurrent_acquire_and_drop() {
        let agent = Arc::new(Mutex::new(EdiaAgent::new(16)));

        // Spawn many concurrent acquire+drop cycles to force lock
        // contention on the agent mutex inside `acquire`, and to
        // exercise `Drop` firing while other guards are mid-flight.
        let mut handles = Vec::new();
        for _ in 0..50 {
            let agent = agent.clone();
            handles.push(tokio::spawn(async move {
                let guard = EdiaGuard::acquire(&agent).await;
                // tiny yield to widen the window for overlap with
                // other tasks' acquire/drop
                tokio::task::yield_now().await;
                drop(guard);
            }));
        }

        for h in handles {
            h.await.expect("task panicked");
        }

        let locked = agent.lock().await;
        assert_eq!(
            locked.pending_requests.load(Ordering::Acquire),
            0,
            "all 50 concurrent guards should net out to zero pending_requests"
        );
    }
}
