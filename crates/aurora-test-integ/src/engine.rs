/// test integ: setup, run, verify, teardown, log
/// Phase 1519

#[derive(Debug, Clone)]
pub struct TestInteg {
    pub setup_ok: bool,
    pub run_ok: bool,
    pub verify_ok: bool,
    pub teardown_ok: bool,
    pub log_ok: bool,
}

impl Default for TestInteg {
    fn default() -> Self {
        Self::new()
    }
}

impl TestInteg {
    pub fn new() -> Self {
        Self {
            setup_ok: true,
            run_ok: true,
            verify_ok: true,
            teardown_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.setup_ok && self.run_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.teardown_ok && self.log_ok
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
        let c = TestInteg::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestInteg::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestInteg::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestInteg::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestInteg::new();
        c.setup_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestInteg::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
