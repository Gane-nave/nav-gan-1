/// aurora-event-bus2: event bus2
/// Phase 2574

#[derive(Debug, Clone)]
pub struct EventBus2 {
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub filter_ok: bool,
    pub route_ok: bool,
    pub replay_ok: bool,
}

impl Default for EventBus2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus2 {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            subscribe_ok: true,
            filter_ok: true,
            route_ok: true,
            replay_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.subscribe_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.route_ok && self.replay_ok
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
        let c = EventBus2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventBus2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventBus2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventBus2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventBus2::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventBus2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventBus2::default();
        assert!(c.all_ok());
    }
}
