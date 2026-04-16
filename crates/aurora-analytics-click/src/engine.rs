/// analytics click: track, aggregate, funnel, report, log
/// Phase 2184

#[derive(Debug, Clone)]
pub struct AnalyticsClick {
    pub track_ok: bool,
    pub aggregate_ok: bool,
    pub funnel_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsClick {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsClick {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            aggregate_ok: true,
            funnel_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.aggregate_ok && self.funnel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.aggregate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok {
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
        let c = AnalyticsClick::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsClick::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsClick::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsClick::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsClick::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsClick::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
