/// aurora-mock-svc: mock svc
/// Phase 2491

#[derive(Debug, Clone)]
pub struct MockSvc {
    pub start_ok: bool,
    pub stop_ok: bool,
    pub invoke_ok: bool,
    pub respond_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockSvc {
    fn default() -> Self {
        Self::new()
    }
}

impl MockSvc {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            stop_ok: true,
            invoke_ok: true,
            respond_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.stop_ok && self.invoke_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.respond_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.stop_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
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
        let c = MockSvc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockSvc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockSvc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockSvc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockSvc::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockSvc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockSvc::default();
        assert!(c.all_ok());
    }
}
