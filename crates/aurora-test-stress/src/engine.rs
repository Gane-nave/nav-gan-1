/// test stress: saturate, monitor, degrade, recover, log
/// Phase 1523

#[derive(Debug, Clone)]
pub struct TestStress {
    pub saturate_ok: bool,
    pub monitor_ok: bool,
    pub degrade_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for TestStress {
    fn default() -> Self {
        Self::new()
    }
}

impl TestStress {
    pub fn new() -> Self {
        Self {
            saturate_ok: true,
            monitor_ok: true,
            degrade_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.saturate_ok && self.monitor_ok && self.degrade_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.saturate_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.saturate_ok {
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
        let c = TestStress::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestStress::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestStress::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestStress::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestStress::new();
        c.saturate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestStress::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
