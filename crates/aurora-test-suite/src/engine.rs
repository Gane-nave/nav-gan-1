/// aurora-test-suite: test suite
/// Phase 2474

#[derive(Debug, Clone)]
pub struct TestSuite {
    pub add_ok: bool,
    pub remove_ok: bool,
    pub run_ok: bool,
    pub tag_ok: bool,
    pub filter_ok: bool,
}

impl Default for TestSuite {
    fn default() -> Self {
        Self::new()
    }
}

impl TestSuite {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            remove_ok: true,
            run_ok: true,
            tag_ok: true,
            filter_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.remove_ok && self.run_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tag_ok && self.filter_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.remove_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.add_ok {
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
        let c = TestSuite::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestSuite::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestSuite::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestSuite::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestSuite::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestSuite::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestSuite::default();
        assert!(c.all_ok());
    }
}
