/// event filter2: match, transform, route, drop, log
/// Phase 1871

#[derive(Debug, Clone)]
pub struct EventFilter2 {
    pub match_ok: bool,
    pub transform_ok: bool,
    pub route_ok: bool,
    pub drop_ok: bool,
    pub log_ok: bool,
}

impl Default for EventFilter2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EventFilter2 {
    pub fn new() -> Self {
        Self {
            match_ok: true,
            transform_ok: true,
            route_ok: true,
            drop_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.match_ok && self.transform_ok && self.route_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.drop_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.match_ok || !self.transform_ok
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
        let c = EventFilter2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventFilter2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventFilter2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventFilter2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventFilter2::new();
        c.match_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventFilter2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
