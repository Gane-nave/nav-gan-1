/// test fixture2: setup, teardown, share, isolate, log
/// Phase 2110

#[derive(Debug, Clone)]
pub struct TestFixture2 {
    pub setup_ok: bool,
    pub teardown_ok: bool,
    pub share_ok: bool,
    pub isolate_ok: bool,
    pub log_ok: bool,
}

impl Default for TestFixture2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TestFixture2 {
    pub fn new() -> Self {
        Self {
            setup_ok: true,
            teardown_ok: true,
            share_ok: true,
            isolate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.setup_ok && self.teardown_ok && self.share_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.isolate_ok && self.log_ok
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
        let c = TestFixture2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestFixture2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestFixture2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestFixture2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestFixture2::new();
        c.setup_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestFixture2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
