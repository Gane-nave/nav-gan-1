/// analytics batch2: schedule, process, store, report, log
/// Phase 2194

#[derive(Debug, Clone)]
pub struct AnalyticsBatch2 {
    pub schedule_ok: bool,
    pub process_ok: bool,
    pub store_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsBatch2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsBatch2 {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            process_ok: true,
            store_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.process_ok && self.store_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.process_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schedule_ok {
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
        let c = AnalyticsBatch2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsBatch2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsBatch2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsBatch2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsBatch2::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsBatch2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
