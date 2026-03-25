/// test mock2: create, expect, verify, reset, log
/// Phase 2108

#[derive(Debug, Clone)]
pub struct TestMock2 {
    pub create_ok: bool,
    pub expect_ok: bool,
    pub verify_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for TestMock2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TestMock2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            expect_ok: true,
            verify_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.expect_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.expect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = TestMock2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestMock2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestMock2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestMock2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestMock2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestMock2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
