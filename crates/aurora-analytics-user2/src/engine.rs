/// analytics user2: identify, segment, cohort, report, log
/// Phase 2186

#[derive(Debug, Clone)]
pub struct AnalyticsUser2 {
    pub identify_ok: bool,
    pub segment_ok: bool,
    pub cohort_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsUser2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsUser2 {
    pub fn new() -> Self {
        Self {
            identify_ok: true,
            segment_ok: true,
            cohort_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.identify_ok && self.segment_ok && self.cohort_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.identify_ok || !self.segment_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.identify_ok {
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
        let c = AnalyticsUser2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsUser2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsUser2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsUser2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsUser2::new();
        c.identify_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsUser2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
