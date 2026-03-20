/// test chaos2: inject, observe, recover, report, log
/// Phase 2100

#[derive(Debug, Clone)]
pub struct TestChaos2 {
    pub inject_ok: bool,
    pub observe_ok: bool,
    pub recover_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for TestChaos2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TestChaos2 {
    pub fn new() -> Self {
        Self {
            inject_ok: true,
            observe_ok: true,
            recover_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inject_ok && self.observe_ok && self.recover_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inject_ok || !self.observe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inject_ok {
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
        let c = TestChaos2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestChaos2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestChaos2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestChaos2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestChaos2::new();
        c.inject_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestChaos2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
