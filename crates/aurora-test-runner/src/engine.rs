/// aurora-test-runner: test runner
/// Phase 2472

#[derive(Debug, Clone)]
pub struct TestRunner {
    pub run_ok: bool,
    pub stop_ok: bool,
    pub retry_ok: bool,
    pub report_ok: bool,
    pub schedule_ok: bool,
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            run_ok: true,
            stop_ok: true,
            retry_ok: true,
            report_ok: true,
            schedule_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.run_ok && self.stop_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.schedule_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.run_ok || !self.stop_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.run_ok {
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
        let c = TestRunner::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestRunner::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestRunner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestRunner::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestRunner::new();
        c.run_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestRunner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestRunner::default();
        assert!(c.all_ok());
    }
}
