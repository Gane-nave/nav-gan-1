/// aurora-event-filter: event filter
/// Phase 2577

#[derive(Debug, Clone)]
pub struct EventFilter {
    pub include_ok: bool,
    pub exclude_ok: bool,
    pub regex_ok: bool,
    pub field_ok: bool,
    pub window_ok: bool,
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            include_ok: true,
            exclude_ok: true,
            regex_ok: true,
            field_ok: true,
            window_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.include_ok && self.exclude_ok && self.regex_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.field_ok && self.window_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.include_ok || !self.exclude_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.include_ok {
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
        let c = EventFilter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventFilter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventFilter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventFilter::new();
        c.include_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventFilter::default();
        assert!(c.all_ok());
    }
}
