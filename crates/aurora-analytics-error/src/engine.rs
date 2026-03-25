/// analytics error: capture, classify, aggregate, alert, log
/// Phase 1562

#[derive(Debug, Clone)]
pub struct AnalyticsError {
    pub capture_ok: bool,
    pub classify_ok: bool,
    pub aggregate_ok: bool,
    pub alert_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsError {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsError {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            classify_ok: true,
            aggregate_ok: true,
            alert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.classify_ok && self.aggregate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.classify_ok
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
        let c = AnalyticsError::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsError::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsError::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsError::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsError::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsError::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
