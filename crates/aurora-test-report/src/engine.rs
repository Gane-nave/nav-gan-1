/// aurora-test-report: test report
/// Phase 2473

#[derive(Debug, Clone)]
pub struct TestReport {
    pub generate_ok: bool,
    pub export_ok: bool,
    pub publish_ok: bool,
    pub archive_ok: bool,
    pub compare_ok: bool,
}

impl Default for TestReport {
    fn default() -> Self {
        Self::new()
    }
}

impl TestReport {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            export_ok: true,
            publish_ok: true,
            archive_ok: true,
            compare_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.export_ok && self.publish_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.archive_ok && self.compare_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.export_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = TestReport::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestReport::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestReport::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestReport::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestReport::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestReport::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestReport::default();
        assert!(c.all_ok());
    }
}
