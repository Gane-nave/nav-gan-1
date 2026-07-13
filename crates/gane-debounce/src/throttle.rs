//! Throttle — limits event processing to at most one per time window.

/// A throttle that allows at most one event per time window.
pub struct Throttle {
    /// Minimum interval between events (ms).
    interval_ms: u64,
    /// Last time an event was allowed through.
    last_allowed_ms: Option<u64>,
    /// Total events received (lifetime).
    total_events: u64,
    /// Total events allowed through (lifetime).
    total_allowed: u64,
    /// Total events throttled (lifetime).
    total_throttled: u64,
}

impl Throttle {
    /// Create a new throttle with the given interval.
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval_ms,
            last_allowed_ms: None,
            total_events: 0,
            total_allowed: 0,
            total_throttled: 0,
        }
    }

    /// Check if an event should be allowed at the given timestamp.
    pub fn should_allow(&self, now_ms: u64) -> bool {
        match self.last_allowed_ms {
            Some(last) => now_ms.saturating_sub(last) >= self.interval_ms,
            None => true,
        }
    }

    /// Try to allow an event. Returns true if allowed.
    pub fn try_allow(&mut self, now_ms: u64) -> bool {
        self.total_events += 1;
        if self.should_allow(now_ms) {
            self.last_allowed_ms = Some(now_ms);
            self.total_allowed += 1;
            true
        } else {
            self.total_throttled += 1;
            false
        }
    }

    /// Reset the throttle.
    pub fn reset(&mut self) {
        self.last_allowed_ms = None;
    }

    /// Get the interval.
    pub fn interval_ms(&self) -> u64 {
        self.interval_ms
    }

    /// Total events received.
    pub fn total_events(&self) -> u64 {
        self.total_events
    }

    /// Total events allowed.
    pub fn total_allowed(&self) -> u64 {
        self.total_allowed
    }

    /// Total events throttled.
    pub fn total_throttled(&self) -> u64 {
        self.total_throttled
    }

    /// Throttle ratio (0.0 = nothing throttled, 1.0 = everything throttled).
    pub fn throttle_ratio(&self) -> f64 {
        if self.total_events == 0 {
            return 0.0;
        }
        self.total_throttled as f64 / self.total_events as f64
    }

    /// Time until next event can be allowed.
    pub fn time_until_next_ms(&self, now_ms: u64) -> u64 {
        match self.last_allowed_ms {
            Some(last) => {
                let elapsed = now_ms.saturating_sub(last);
                self.interval_ms.saturating_sub(elapsed)
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throttle_first_always_allowed() {
        let mut t = Throttle::new(1000);
        assert!(t.try_allow(100));
        assert_eq!(t.total_allowed(), 1);
    }

    #[test]
    fn test_throttle_blocks_rapid() {
        let mut t = Throttle::new(1000);
        assert!(t.try_allow(100));
        assert!(!t.try_allow(500)); // 400ms < 1000ms
        assert_eq!(t.total_throttled(), 1);
    }

    #[test]
    fn test_throttle_allows_after_interval() {
        let mut t = Throttle::new(1000);
        assert!(t.try_allow(100));
        assert!(!t.try_allow(500));
        assert!(t.try_allow(1100)); // 1000ms since last allowed
    }

    #[test]
    fn test_throttle_reset() {
        let mut t = Throttle::new(1000);
        t.try_allow(100);
        t.reset();
        assert!(t.try_allow(200)); // immediately after reset
    }

    #[test]
    fn test_throttle_ratio() {
        let mut t = Throttle::new(1000);
        t.try_allow(100); // allowed
        t.try_allow(200); // throttled
        t.try_allow(300); // throttled
        t.try_allow(1100); // allowed
        assert!((t.throttle_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_throttle_time_until_next() {
        let mut t = Throttle::new(1000);
        assert_eq!(t.time_until_next_ms(100), 0); // no previous
        t.try_allow(100);
        assert_eq!(t.time_until_next_ms(500), 600); // 600ms remaining
        assert_eq!(t.time_until_next_ms(1100), 0); // ready
    }

    #[test]
    fn test_throttle_zero_interval() {
        let mut t = Throttle::new(0);
        assert!(t.try_allow(100));
        assert!(t.try_allow(100)); // same timestamp, interval=0
        assert!(t.try_allow(200));
    }

    #[test]
    fn test_throttle_stats() {
        let mut t = Throttle::new(500);
        t.try_allow(100);
        t.try_allow(200);
        t.try_allow(300);
        t.try_allow(700);
        assert_eq!(t.total_events(), 4);
        assert_eq!(t.total_allowed(), 2); // t=100, t=700
        assert_eq!(t.total_throttled(), 2); // t=200, t=300
    }
}
