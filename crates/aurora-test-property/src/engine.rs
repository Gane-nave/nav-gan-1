/// test property: generate, shrink, verify, report, log
/// Phase 1533

#[derive(Debug, Clone)]
pub struct TestProperty {
    pub generate_ok: bool,
    pub shrink_ok: bool,
    pub verify_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for TestProperty {
    fn default() -> Self {
        Self::new()
    }
}

impl TestProperty {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            shrink_ok: true,
            verify_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.shrink_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.shrink_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TestProperty::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestProperty::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestProperty::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestProperty::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestProperty::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestProperty::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
