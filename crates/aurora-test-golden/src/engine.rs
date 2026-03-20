/// test golden: capture, compare, update, review, log
/// Phase 2104

#[derive(Debug, Clone)]
pub struct TestGolden {
    pub capture_ok: bool,
    pub compare_ok: bool,
    pub update_ok: bool,
    pub review_ok: bool,
    pub log_ok: bool,
}

impl Default for TestGolden {
    fn default() -> Self {
        Self::new()
    }
}

impl TestGolden {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            compare_ok: true,
            update_ok: true,
            review_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.compare_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.review_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.compare_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
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
        let c = TestGolden::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestGolden::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestGolden::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestGolden::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestGolden::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestGolden::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
