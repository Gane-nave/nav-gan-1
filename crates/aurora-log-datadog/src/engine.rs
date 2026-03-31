/// aurora-log-datadog: log datadog
/// Phase 2615

#[derive(Debug, Clone)]
pub struct LogDatadog {
    pub ingest_ok: bool,
    pub search_ok: bool,
    pub alert_ok: bool,
    pub dashboard_ok: bool,
    pub export_ok: bool,
}

impl Default for LogDatadog {
    fn default() -> Self {
        Self::new()
    }
}

impl LogDatadog {
    pub fn new() -> Self {
        Self {
            ingest_ok: true,
            search_ok: true,
            alert_ok: true,
            dashboard_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.ingest_ok && self.search_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dashboard_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.ingest_ok || !self.search_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ingest_ok {
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
        let c = LogDatadog::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LogDatadog::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LogDatadog::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LogDatadog::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LogDatadog::new();
        c.ingest_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LogDatadog::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = LogDatadog::default();
        assert!(c.all_ok());
    }
}
