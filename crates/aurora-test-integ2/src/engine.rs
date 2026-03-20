/// test integ2: setup, run, teardown, report, log
/// Phase 2095

#[derive(Debug, Clone)]
pub struct TestInteg2 {
    pub setup_ok: bool,
    pub run_ok: bool,
    pub teardown_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for TestInteg2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TestInteg2 {
    pub fn new() -> Self {
        Self {
            setup_ok: true,
            run_ok: true,
            teardown_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.setup_ok && self.run_ok && self.teardown_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.setup_ok || !self.run_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.setup_ok {
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
        let c = TestInteg2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestInteg2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestInteg2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestInteg2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestInteg2::new();
        c.setup_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestInteg2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
