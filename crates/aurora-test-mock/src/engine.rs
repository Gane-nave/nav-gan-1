/// test mock: define, inject, verify, reset, log
/// Phase 1528

#[derive(Debug, Clone)]
pub struct TestMock {
    pub define_ok: bool,
    pub inject_ok: bool,
    pub verify_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for TestMock {
    fn default() -> Self {
        Self::new()
    }
}

impl TestMock {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            inject_ok: true,
            verify_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.inject_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.inject_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TestMock::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestMock::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestMock::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestMock::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestMock::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestMock::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
