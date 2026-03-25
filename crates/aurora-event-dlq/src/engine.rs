/// event dlq: enqueue, inspect, retry, purge, log
/// Phase 1867

#[derive(Debug, Clone)]
pub struct EventDlq {
    pub enqueue_ok: bool,
    pub inspect_ok: bool,
    pub retry_ok: bool,
    pub purge_ok: bool,
    pub log_ok: bool,
}

impl Default for EventDlq {
    fn default() -> Self {
        Self::new()
    }
}

impl EventDlq {
    pub fn new() -> Self {
        Self {
            enqueue_ok: true,
            inspect_ok: true,
            retry_ok: true,
            purge_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.enqueue_ok && self.inspect_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.purge_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.enqueue_ok || !self.inspect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.enqueue_ok {
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
        let c = EventDlq::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventDlq::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventDlq::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventDlq::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventDlq::new();
        c.enqueue_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventDlq::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
