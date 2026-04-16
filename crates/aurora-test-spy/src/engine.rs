/// aurora-test-spy: test spy
/// Phase 2479

#[derive(Debug, Clone)]
pub struct TestSpy {
    pub track_ok: bool,
    pub verify_ok: bool,
    pub reset_ok: bool,
    pub record_ok: bool,
    pub assert_ok: bool,
}

impl Default for TestSpy {
    fn default() -> Self {
        Self::new()
    }
}

impl TestSpy {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            verify_ok: true,
            reset_ok: true,
            record_ok: true,
            assert_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.verify_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.record_ok && self.assert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.verify_ok
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
        let c = TestSpy::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestSpy::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestSpy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestSpy::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestSpy::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestSpy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestSpy::default();
        assert!(c.all_ok());
    }
}
