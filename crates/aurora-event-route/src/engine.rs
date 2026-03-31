/// aurora-event-route: event route
/// Phase 2579

#[derive(Debug, Clone)]
pub struct EventRoute {
    pub match_ok: bool,
    pub dispatch_ok: bool,
    pub fan_ok: bool,
    pub aggregate_ok: bool,
    pub dlq_ok: bool,
}

impl Default for EventRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl EventRoute {
    pub fn new() -> Self {
        Self {
            match_ok: true,
            dispatch_ok: true,
            fan_ok: true,
            aggregate_ok: true,
            dlq_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.match_ok && self.dispatch_ok && self.fan_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.aggregate_ok && self.dlq_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.match_ok || !self.dispatch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.match_ok {
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
        let c = EventRoute::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventRoute::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventRoute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventRoute::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventRoute::new();
        c.match_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventRoute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventRoute::default();
        assert!(c.all_ok());
    }
}
