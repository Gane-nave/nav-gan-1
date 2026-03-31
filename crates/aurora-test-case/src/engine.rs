/// aurora-test-case: test case
/// Phase 2475

#[derive(Debug, Clone)]
pub struct TestCase {
    pub define_ok: bool,
    pub run_ok: bool,
    pub assert_ok: bool,
    pub teardown_ok: bool,
    pub log_ok: bool,
}

impl Default for TestCase {
    fn default() -> Self {
        Self::new()
    }
}

impl TestCase {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            run_ok: true,
            assert_ok: true,
            teardown_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.run_ok && self.assert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.teardown_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.run_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = TestCase::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestCase::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestCase::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestCase::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestCase::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestCase::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestCase::default();
        assert!(c.all_ok());
    }
}
