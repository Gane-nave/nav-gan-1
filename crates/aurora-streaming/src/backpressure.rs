//! Back-pressure handling for streaming pipelines.

/// Back-pressure strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpressureStrategy {
    /// Drop newest messages when buffer is full.
    DropNewest,
    /// Drop oldest messages when buffer is full.
    DropOldest,
    /// Block the producer until space is available.
    Block,
    /// Apply sampling — only keep every Nth message.
    Sample(u32),
}

/// Back-pressure state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureLevel {
    /// No pressure — normal operation.
    None,
    /// Light pressure — starting to buffer.
    Light,
    /// Moderate pressure — should slow down.
    Moderate,
    /// Heavy pressure — near capacity.
    Heavy,
    /// Critical — at capacity, shedding load.
    Critical,
}

/// Back-pressure controller for a streaming pipeline.
pub struct BackpressureController {
    /// Maximum buffer capacity.
    capacity: usize,
    /// Current buffer fill level.
    current_fill: usize,
    /// Strategy when buffer is full.
    strategy: BackpressureStrategy,
    /// Total accepted messages.
    accepted: u64,
    /// Total dropped messages.
    dropped: u64,
    /// Total blocked attempts.
    blocked: u64,
    /// Sample counter for Sample strategy.
    sample_counter: u32,
    /// Thresholds for pressure levels (as fraction of capacity).
    light_threshold: f64,
    moderate_threshold: f64,
    heavy_threshold: f64,
    critical_threshold: f64,
}

impl BackpressureController {
    /// Create a new controller with given capacity and strategy.
    pub fn new(capacity: usize, strategy: BackpressureStrategy) -> Self {
        Self {
            capacity,
            current_fill: 0,
            strategy,
            accepted: 0,
            dropped: 0,
            blocked: 0,
            sample_counter: 0,
            light_threshold: 0.25,
            moderate_threshold: 0.50,
            heavy_threshold: 0.75,
            critical_threshold: 0.90,
        }
    }

    /// Try to admit a message. Returns true if accepted.
    pub fn try_admit(&mut self) -> bool {
        // Check sampling strategy first
        if let BackpressureStrategy::Sample(n) = self.strategy {
            if n == 0 {
                self.dropped += 1;
                return false;
            }
            self.sample_counter += 1;
            #[allow(unknown_lints, clippy::manual_is_multiple_of)]
            if self.sample_counter % n != 0 {
                self.dropped += 1;
                return false;
            }
        }

        if self.current_fill >= self.capacity {
            match self.strategy {
                BackpressureStrategy::DropNewest => {
                    self.dropped += 1;
                    false
                }
                BackpressureStrategy::DropOldest => {
                    // Conceptually drop the oldest — consumer side handles this.
                    // We admit the new one.
                    self.accepted += 1;
                    true
                }
                BackpressureStrategy::Block => {
                    self.blocked += 1;
                    false
                }
                BackpressureStrategy::Sample(_) => {
                    self.dropped += 1;
                    false
                }
            }
        } else {
            self.current_fill += 1;
            self.accepted += 1;
            true
        }
    }

    /// Signal that a message was consumed (frees buffer space).
    pub fn consume(&mut self) {
        self.current_fill = self.current_fill.saturating_sub(1);
    }

    /// Consume multiple messages at once.
    pub fn consume_n(&mut self, n: usize) {
        self.current_fill = self.current_fill.saturating_sub(n);
    }

    /// Current pressure level.
    pub fn pressure_level(&self) -> PressureLevel {
        let ratio = self.fill_ratio();
        if ratio >= self.critical_threshold {
            PressureLevel::Critical
        } else if ratio >= self.heavy_threshold {
            PressureLevel::Heavy
        } else if ratio >= self.moderate_threshold {
            PressureLevel::Moderate
        } else if ratio >= self.light_threshold {
            PressureLevel::Light
        } else {
            PressureLevel::None
        }
    }

    /// Current fill ratio (0.0 to 1.0).
    pub fn fill_ratio(&self) -> f64 {
        if self.capacity == 0 {
            return 1.0;
        }
        self.current_fill as f64 / self.capacity as f64
    }

    /// Current fill level.
    pub fn current_fill(&self) -> usize {
        self.current_fill
    }

    /// Remaining capacity.
    pub fn remaining(&self) -> usize {
        self.capacity.saturating_sub(self.current_fill)
    }

    /// Total accepted messages.
    pub fn accepted_count(&self) -> u64 {
        self.accepted
    }

    /// Total dropped messages.
    pub fn dropped_count(&self) -> u64 {
        self.dropped
    }

    /// Total blocked attempts.
    pub fn blocked_count(&self) -> u64 {
        self.blocked
    }

    /// Acceptance rate.
    pub fn acceptance_rate(&self) -> f64 {
        let total = self.accepted + self.dropped + self.blocked;
        if total == 0 {
            return 1.0;
        }
        self.accepted as f64 / total as f64
    }

    /// Reset counters.
    pub fn reset_counters(&mut self) {
        self.accepted = 0;
        self.dropped = 0;
        self.blocked = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admit_within_capacity() {
        let mut ctrl = BackpressureController::new(10, BackpressureStrategy::DropNewest);
        for _ in 0..10 {
            assert!(ctrl.try_admit());
        }
        assert_eq!(ctrl.current_fill(), 10);
    }

    #[test]
    fn test_drop_newest_at_capacity() {
        let mut ctrl = BackpressureController::new(2, BackpressureStrategy::DropNewest);
        assert!(ctrl.try_admit());
        assert!(ctrl.try_admit());
        assert!(!ctrl.try_admit()); // dropped
        assert_eq!(ctrl.dropped_count(), 1);
    }

    #[test]
    fn test_drop_oldest_at_capacity() {
        let mut ctrl = BackpressureController::new(2, BackpressureStrategy::DropOldest);
        assert!(ctrl.try_admit());
        assert!(ctrl.try_admit());
        assert!(ctrl.try_admit()); // oldest conceptually dropped, new one accepted
        assert_eq!(ctrl.accepted_count(), 3);
    }

    #[test]
    fn test_block_at_capacity() {
        let mut ctrl = BackpressureController::new(1, BackpressureStrategy::Block);
        assert!(ctrl.try_admit());
        assert!(!ctrl.try_admit()); // blocked
        assert_eq!(ctrl.blocked_count(), 1);
    }

    #[test]
    fn test_sample_strategy() {
        let mut ctrl = BackpressureController::new(100, BackpressureStrategy::Sample(3));
        let mut accepted = 0;
        for _ in 0..9 {
            if ctrl.try_admit() {
                accepted += 1;
            }
        }
        assert_eq!(accepted, 3); // every 3rd message
    }

    #[test]
    fn test_consume_frees_space() {
        let mut ctrl = BackpressureController::new(2, BackpressureStrategy::DropNewest);
        assert!(ctrl.try_admit());
        assert!(ctrl.try_admit());
        assert!(!ctrl.try_admit()); // full

        ctrl.consume();
        assert!(ctrl.try_admit()); // space freed
    }

    #[test]
    fn test_pressure_levels() {
        let mut ctrl = BackpressureController::new(100, BackpressureStrategy::DropNewest);
        assert_eq!(ctrl.pressure_level(), PressureLevel::None);

        for _ in 0..30 {
            ctrl.try_admit();
        }
        assert_eq!(ctrl.pressure_level(), PressureLevel::Light);

        for _ in 0..30 {
            ctrl.try_admit();
        }
        assert_eq!(ctrl.pressure_level(), PressureLevel::Moderate);

        for _ in 0..20 {
            ctrl.try_admit();
        }
        assert_eq!(ctrl.pressure_level(), PressureLevel::Heavy);

        for _ in 0..15 {
            ctrl.try_admit();
        }
        assert_eq!(ctrl.pressure_level(), PressureLevel::Critical);
    }

    #[test]
    fn test_fill_ratio() {
        let mut ctrl = BackpressureController::new(100, BackpressureStrategy::DropNewest);
        assert!((ctrl.fill_ratio() - 0.0).abs() < f64::EPSILON);
        for _ in 0..50 {
            ctrl.try_admit();
        }
        assert!((ctrl.fill_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_acceptance_rate() {
        let mut ctrl = BackpressureController::new(2, BackpressureStrategy::DropNewest);
        ctrl.try_admit();
        ctrl.try_admit();
        ctrl.try_admit(); // dropped
        ctrl.try_admit(); // dropped
        assert!((ctrl.acceptance_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_consume_n() {
        let mut ctrl = BackpressureController::new(10, BackpressureStrategy::DropNewest);
        for _ in 0..8 {
            ctrl.try_admit();
        }
        ctrl.consume_n(5);
        assert_eq!(ctrl.current_fill(), 3);
        assert_eq!(ctrl.remaining(), 7);
    }

    #[test]
    fn test_zero_capacity() {
        let mut ctrl = BackpressureController::new(0, BackpressureStrategy::DropNewest);
        assert!((ctrl.fill_ratio() - 1.0).abs() < f64::EPSILON);
        assert!(!ctrl.try_admit());
    }

    #[test]
    fn test_sample_zero_no_panic() {
        // Regression: Sample(0) must not panic from division by zero.
        let mut ctrl = BackpressureController::new(100, BackpressureStrategy::Sample(0));
        assert!(!ctrl.try_admit());
        assert!(!ctrl.try_admit());
        assert_eq!(ctrl.dropped_count(), 2);
        assert_eq!(ctrl.accepted_count(), 0);
    }
}
