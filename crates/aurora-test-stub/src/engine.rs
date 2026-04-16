/// aurora-test-stub: test stub
/// Phase 2478

#[derive(Debug, Clone)]
pub struct TestStub {
    pub define_ok: bool,
    pub invoke_ok: bool,
    pub reset_ok: bool,
    pub record_ok: bool,
    pub verify_ok: bool,
}

impl Default for TestStub {
    fn default() -> Self {
        Self::new()
    }
}

impl TestStub {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            invoke_ok: true,
            reset_ok: true,
            record_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.invoke_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.record_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.invoke_ok
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
        let c = TestStub::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestStub::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestStub::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestStub::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestStub::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestStub::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestStub::default();
        assert!(c.all_ok());
    }
}
