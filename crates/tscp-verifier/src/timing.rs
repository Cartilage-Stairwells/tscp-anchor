use std::collections::HashMap;
use std::time::Instant;

/// Records wall-clock time for named benchmark phases.
/// Phase boundaries must be measured where the work occurs,
/// not inferred from total pipeline time.
///
/// Supports nested phases (stack-based) and timer merging (set/merge).
pub struct PhaseTimer {
    phases: HashMap<String, f64>, // phase name → elapsed ms
    active: Vec<(String, Instant)>,
}

impl PhaseTimer {
    pub fn new() -> Self {
        Self {
            phases: HashMap::new(),
            active: Vec::new(),
        }
    }

    /// Start timing a named phase. Supports nested phases.
    pub fn start(&mut self, phase: &str) {
        self.active.push((phase.to_string(), Instant::now()));
    }

    /// Stop the most recently started phase and record its elapsed time.
    pub fn stop(&mut self) {
        if let Some((phase, start)) = self.active.pop() {
            let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
            self.phases.insert(phase, elapsed_ms);
        } else {
            panic!("Cannot stop phase: no phase is currently active");
        }
    }

    /// Set a phase's elapsed time directly (for merging timers).
    /// This does NOT use wall-clock measurement — it sets an arbitrary value.
    /// Use when combining results from separate timer instances.
    pub fn set(&mut self, phase: &str, ms: f64) {
        self.phases.insert(phase.to_string(), ms);
    }

    /// Merge another timer's phases into this one.
    /// Values from `other` overwrite existing values for the same phase.
    pub fn merge(&mut self, other: &PhaseTimer) {
        for (phase, ms) in &other.phases {
            self.phases.insert(phase.clone(), *ms);
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
