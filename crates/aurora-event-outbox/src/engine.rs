/// event outbox: store, publish, retry, cleanup, log
/// Phase 1865

#[derive(Debug, Clone)]
pub struct EventOutbox {
    pub store_ok: bool,
    pub publish_ok: bool,
    pub retry_ok: bool,
    pub cleanup_ok: bool,
    pub log_ok: bool,
}

impl Default for EventOutbox {
    fn default() -> Self {
        Self::new()
    }
}

impl EventOutbox {
    pub fn new() -> Self {
        Self {
            store_ok: true,
            publish_ok: true,
            retry_ok: true,
            cleanup_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.store_ok && self.publish_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cleanup_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.store_ok || !self.publish_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.store_ok {
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
        let c = EventOutbox::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventOutbox::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventOutbox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventOutbox::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventOutbox::new();
        c.store_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventOutbox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
