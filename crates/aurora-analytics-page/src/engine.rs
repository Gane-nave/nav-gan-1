/// analytics page: track, session, bounce, report, log
/// Phase 2185

#[derive(Debug, Clone)]
pub struct AnalyticsPage {
    pub track_ok: bool,
    pub session_ok: bool,
    pub bounce_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsPage {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsPage {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            session_ok: true,
            bounce_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.session_ok && self.bounce_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.session_ok
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
        let c = AnalyticsPage::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsPage::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsPage::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsPage::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsPage::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsPage::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
