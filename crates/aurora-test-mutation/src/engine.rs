/// test mutation: mutate, run, detect, report, log
/// Phase 1532

#[derive(Debug, Clone)]
pub struct TestMutation {
    pub mutate_ok: bool,
    pub run_ok: bool,
    pub detect_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for TestMutation {
    fn default() -> Self {
        Self::new()
    }
}

impl TestMutation {
    pub fn new() -> Self {
        Self {
            mutate_ok: true,
            run_ok: true,
            detect_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mutate_ok && self.run_ok && self.detect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mutate_ok || !self.run_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mutate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TestMutation::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestMutation::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestMutation::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestMutation::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestMutation::new();
        c.mutate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestMutation::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
