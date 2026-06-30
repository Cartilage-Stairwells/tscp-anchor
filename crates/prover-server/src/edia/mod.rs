use std::sync::Arc;
use tokio::sync::Mutex;

use std::collections::VecDeque;

pub struct EdiaAgent {
    pub ring_buffer: VecDeque<u64>,
    pub drain_rate_tps: u32,
    // REAL NEW FIELD: Tracks active, in-flight proving operations
    pub pending_requests: u64,
}

impl EdiaAgent {
    pub fn new(capacity: usize) -> Self {
        Self {
            ring_buffer: VecDeque::with_capacity(capacity),
            drain_rate_tps: 1,
            pending_requests: 0,
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
        assert_eq!(agent.pending_requests, 0);
    }
}

    #[tokio::test]
    async fn edia_guard_decrements_pending_on_drop() {
        let agent = Arc::new(Mutex::new(EdiaAgent::new(16)));

        {
            let mut locked = agent.lock().await;
            locked.pending_requests += 1;
            assert_eq!(locked.pending_requests, 1);
        }

        let guard = EdiaGuard::new(agent.clone());
        drop(guard);

        let locked = agent.lock().await;
        assert_eq!(
            locked.pending_requests, 0,
            "EdiaGuard::drop must decrement pending_requests back to 0"
        );
    }

    #[tokio::test]
    async fn edia_guard_does_not_underflow_on_double_relevant_drop() {
        // pending_requests already at 0; guard's drop must not panic or wrap.
        let agent = Arc::new(Mutex::new(EdiaAgent::new(16)));
        let guard = EdiaGuard::new(agent.clone());
        drop(guard);

        let locked = agent.lock().await;
        assert_eq!(locked.pending_requests, 0);
    }

pub struct EdiaGuard {
    agent: Arc<Mutex<EdiaAgent>>,
}

impl EdiaGuard {
    pub fn new(agent: Arc<Mutex<EdiaAgent>>) -> Self {
        Self { agent }
    }
}

impl Drop for EdiaGuard {
    fn drop(&mut self) {
        let mut agent = self
            .agent
            .try_lock()
            .expect("EdiaGuard drop could not acquire admission lock");

        if agent.pending_requests > 0 {
            agent.pending_requests -= 1;
        }
    }
}
