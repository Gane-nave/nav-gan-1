/// analytics event: capture, enrich, store, query, log
/// Phase 1554

#[derive(Debug, Clone)]
pub struct AnalyticsEvent {
    pub capture_ok: bool,
    pub enrich_ok: bool,
    pub store_ok: bool,
    pub query_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsEvent {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            enrich_ok: true,
            store_ok: true,
            query_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.enrich_ok && self.store_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.query_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.enrich_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
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
        let c = AnalyticsEvent::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsEvent::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsEvent::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsEvent::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsEvent::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsEvent::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
