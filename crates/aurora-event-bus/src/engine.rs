/// event bus: publish, subscribe, unsubscribe, replay, log
/// Phase 1860

#[derive(Debug, Clone)]
pub struct EventBus {
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub unsubscribe_ok: bool,
    pub replay_ok: bool,
    pub log_ok: bool,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            subscribe_ok: true,
            unsubscribe_ok: true,
            replay_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.subscribe_ok && self.unsubscribe_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.replay_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.publish_ok || !self.subscribe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.publish_ok {
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
        let c = EventBus::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventBus::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventBus::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventBus::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventBus::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventBus::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
