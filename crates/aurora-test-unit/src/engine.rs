/// test unit: discover, run, assert, report, log
/// Phase 1518

#[derive(Debug, Clone)]
pub struct TestUnit {
    pub discover_ok: bool,
    pub run_ok: bool,
    pub assert_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for TestUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl TestUnit {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            run_ok: true,
            assert_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.discover_ok && self.run_ok && self.assert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.discover_ok || !self.run_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.discover_ok {
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
        let c = TestUnit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestUnit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestUnit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestUnit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestUnit::new();
        c.discover_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestUnit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
