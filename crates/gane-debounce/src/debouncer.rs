//! Debouncer — delays event processing until a quiet period has elapsed.

/// State of a debounced event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebounceState {
    /// Waiting for the quiet period to elapse.
    Waiting,
    /// Ready to fire (quiet period elapsed).
    Ready,
    /// Already fired.
    Fired,
}

/// A debouncer that delays processing until no new events arrive for a quiet period.
pub struct Debouncer {
    /// Quiet period in milliseconds.
    quiet_ms: u64,
    /// Last event timestamp.
    last_event_ms: Option<u64>,
    /// Total events received (lifetime).
    total_events: u64,
    /// Total times fired (lifetime).
    total_fired: u64,
    /// Total events suppressed (not fired because of subsequent events).
    total_suppressed: u64,
    /// Whether the debouncer has fired for the current batch.
    fired: bool,
}

impl Debouncer {
    /// Create a new debouncer with the given quiet period.
    pub fn new(quiet_ms: u64) -> Self {
        Self {
            quiet_ms,
            last_event_ms: None,
            total_events: 0,
            total_fired: 0,
            total_suppressed: 0,
            fired: false,
        }
    }

    /// Record a new event at the given timestamp.
    pub fn event(&mut self, now_ms: u64) {
        if self.last_event_ms.is_some() && !self.fired {
            self.total_suppressed += 1;
        }
        self.last_event_ms = Some(now_ms);
        self.total_events += 1;
        self.fired = false;
    }

    /// Check if the debouncer should fire at the given timestamp.
    pub fn should_fire(&self, now_ms: u64) -> bool {
        if self.fired {
            return false;
        }
        match self.last_event_ms {
            Some(last) => now_ms.saturating_sub(last) >= self.quiet_ms,
            None => false,
        }
    }

    /// Fire the debouncer (mark as fired).
    pub fn fire(&mut self) {
        if !self.fired && self.last_event_ms.is_some() {
            self.fired = true;
            self.total_fired += 1;
        }
    }

    /// Check and fire if ready. Returns true if fired.
    pub fn check_and_fire(&mut self, now_ms: u64) -> bool {
        if self.should_fire(now_ms) {
            self.fire();
            true
        } else {
            false
        }
    }

    /// Get the current state at the given timestamp.
    pub fn state(&self, now_ms: u64) -> DebounceState {
        if self.fired {
            return DebounceState::Fired;
        }
        if self.should_fire(now_ms) {
            return DebounceState::Ready;
        }
        if self.last_event_ms.is_some() {
            return DebounceState::Waiting;
        }
        DebounceState::Fired // no events = nothing to do
    }

    /// Reset the debouncer.
    pub fn reset(&mut self) {
        self.last_event_ms = None;
        self.fired = false;
    }

    /// Get the quiet period.
    pub fn quiet_ms(&self) -> u64 {
        self.quiet_ms
    }

    /// Total events received.
    pub fn total_events(&self) -> u64 {
        self.total_events
    }

    /// Total times fired.
    pub fn total_fired(&self) -> u64 {
        self.total_fired
    }

    /// Total events suppressed.
    pub fn total_suppressed(&self) -> u64 {
        self.total_suppressed
    }

    /// Suppression ratio (0.0 = nothing suppressed, 1.0 = everything suppressed).
    pub fn suppression_ratio(&self) -> f64 {
        if self.total_events == 0 {
            return 0.0;
        }
        self.total_suppressed as f64 / self.total_events as f64
    }

    /// Time until next potential fire, or 0 if ready.
    pub fn time_until_fire_ms(&self, now_ms: u64) -> u64 {
        if self.fired {
            return u64::MAX;
        }
        match self.last_event_ms {
            Some(last) => {
                let elapsed = now_ms.saturating_sub(last);
                self.quiet_ms.saturating_sub(elapsed)
            }
            None => u64::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debounce_basic() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        assert!(!d.should_fire(1200)); // 200ms < 500ms quiet
        assert!(d.should_fire(1500)); // 500ms = quiet period
    }

    #[test]
    fn test_debounce_reset_on_new_event() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        d.event(1300); // resets the timer
        assert!(!d.should_fire(1500)); // only 200ms since last event
        assert!(d.should_fire(1800)); // 500ms since t=1300
    }

    #[test]
    fn test_debounce_fire() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        assert!(d.check_and_fire(1500));
        assert!(!d.check_and_fire(1600)); // already fired
        assert_eq!(d.total_fired(), 1);
    }

    #[test]
    fn test_debounce_state() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        assert_eq!(d.state(1200), DebounceState::Waiting);
        assert_eq!(d.state(1500), DebounceState::Ready);
        d.fire();
        assert_eq!(d.state(1600), DebounceState::Fired);
    }

    #[test]
    fn test_debounce_suppression() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        d.event(1200); // suppresses first
        d.event(1400); // suppresses second
        assert_eq!(d.total_events(), 3);
        assert_eq!(d.total_suppressed(), 2);
    }

    #[test]
    fn test_debounce_reset() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        d.reset();
        assert!(!d.should_fire(2000));
        assert_eq!(d.state(2000), DebounceState::Fired);
    }

    #[test]
    fn test_suppression_ratio() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        d.event(1200);
        d.event(1400);
        d.event(1600);
        // 3 suppressed out of 4 events
        assert!((d.suppression_ratio() - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_time_until_fire() {
        let mut d = Debouncer::new(500);
        d.event(1000);
        assert_eq!(d.time_until_fire_ms(1200), 300);
        assert_eq!(d.time_until_fire_ms(1500), 0);
    }
}
