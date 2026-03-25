/// test coverage2: instrument, collect, report, enforce, log
/// Phase 2107

#[derive(Debug, Clone)]
pub struct TestCoverage2 {
    pub instrument_ok: bool,
    pub collect_ok: bool,
    pub report_ok: bool,
    pub enforce_ok: bool,
    pub log_ok: bool,
}

impl Default for TestCoverage2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TestCoverage2 {
    pub fn new() -> Self {
        Self {
            instrument_ok: true,
            collect_ok: true,
            report_ok: true,
            enforce_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.instrument_ok && self.collect_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.enforce_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.instrument_ok || !self.collect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.instrument_ok {
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
        let c = TestCoverage2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestCoverage2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestCoverage2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestCoverage2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestCoverage2::new();
        c.instrument_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestCoverage2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
