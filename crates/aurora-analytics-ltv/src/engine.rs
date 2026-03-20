/// analytics ltv: calculate, predict, segment, report, log
/// Phase 2201

#[derive(Debug, Clone)]
pub struct AnalyticsLtv {
    pub calculate_ok: bool,
    pub predict_ok: bool,
    pub segment_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsLtv {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsLtv {
    pub fn new() -> Self {
        Self {
            calculate_ok: true,
            predict_ok: true,
            segment_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.calculate_ok && self.predict_ok && self.segment_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.calculate_ok || !self.predict_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.calculate_ok {
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
        let c = AnalyticsLtv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsLtv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsLtv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsLtv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsLtv::new();
        c.calculate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsLtv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
