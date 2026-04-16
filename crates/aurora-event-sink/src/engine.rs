/// aurora-event-sink: event sink
/// Phase 2575

#[derive(Debug, Clone)]
pub struct EventSink {
    pub write_ok: bool,
    pub batch_ok: bool,
    pub flush_ok: bool,
    pub retry_ok: bool,
    pub monitor_ok: bool,
}

impl Default for EventSink {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSink {
    pub fn new() -> Self {
        Self {
            write_ok: true,
            batch_ok: true,
            flush_ok: true,
            retry_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.write_ok && self.batch_ok && self.flush_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retry_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.write_ok || !self.batch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.write_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = EventSink::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventSink::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventSink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventSink::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventSink::new();
        c.write_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventSink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventSink::default();
        assert!(c.all_ok());
    }
}
