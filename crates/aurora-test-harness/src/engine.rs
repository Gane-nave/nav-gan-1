/// aurora-test-harness: test harness
/// Phase 2480

#[derive(Debug, Clone)]
pub struct TestHarness {
    pub setup_ok: bool,
    pub teardown_ok: bool,
    pub run_ok: bool,
    pub report_ok: bool,
    pub cleanup_ok: bool,
}

impl Default for TestHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl TestHarness {
    pub fn new() -> Self {
        Self {
            setup_ok: true,
            teardown_ok: true,
            run_ok: true,
            report_ok: true,
            cleanup_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.setup_ok && self.teardown_ok && self.run_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.cleanup_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.setup_ok || !self.teardown_ok
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
        let c = TestHarness::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestHarness::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestHarness::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestHarness::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestHarness::new();
        c.setup_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestHarness::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestHarness::default();
        assert!(c.all_ok());
    }
}
