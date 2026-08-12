use std::collections::HashMap;
use std::time::Instant;

/// Records wall-clock time for named benchmark phases.
/// Phase boundaries must be measured where the work occurs,
/// not inferred from total pipeline time.
pub struct PhaseTimer {
    phases: HashMap<String, f64>,  // phase name → elapsed ms
    active: Option<(String, Instant)>,
}

impl PhaseTimer {
    pub fn new() -> Self {
        Self {
            phases: HashMap::new(),
            active: None,
        }
    }

    /// Start timing a named phase. Panics if a phase is already active.
    pub fn start(&mut self, phase: &str) {
        if let Some((ref active_phase, _)) = self.active {
            panic!("Cannot start phase '{}': phase '{}' is already active", phase, active_phase);
        }
        self.active = Some((phase.to_string(), Instant::now()));
    }

    /// Stop the currently active phase and record its elapsed time.
    pub fn stop(&mut self) {
        if let Some((phase, start)) = self.active.take() {
            let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
            // Support multiple recordings of same phase by summing or overwriting?
            // The instructions don't specify, but overwriting or summing is standard. Overwriting is simple.
            self.phases.insert(phase, elapsed_ms);
        } else {
            panic!("Cannot stop phase: no phase is currently active");
        }
    }

    /// Get elapsed ms for a named phase. Returns None if phase was not recorded.
    pub fn get(&self, phase: &str) -> Option<f64> {
        self.phases.get(phase).cloned()
    }

    /// Get elapsed ms or 0.0 if not recorded.
    pub fn get_or_zero(&self, phase: &str) -> f64 {
        self.get(phase).unwrap_or(0.0)
    }
}

impl Default for PhaseTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard phase names used throughout the emitter.
pub mod phases {
    pub const TRANSCRIPT_GENERATION: &str = "transcript_generation";
    pub const SUMCHECK: &str = "sumcheck";
    pub const FRI: &str = "fri";
    pub const PROVING_TOTAL: &str = "proving_total";
    pub const VERIFICATION: &str = "verification";
}
