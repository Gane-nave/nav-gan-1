/// aurora-qa-coverage: qa coverage
/// Phase 2512

#[derive(Debug, Clone)]
pub struct QaCoverage {
    pub line_ok: bool,
    pub branch_ok: bool,
    pub function_ok: bool,
    pub report_ok: bool,
    pub threshold_ok: bool,
}

impl Default for QaCoverage {
    fn default() -> Self {
        Self::new()
    }
}

impl QaCoverage {
    pub fn new() -> Self {
        Self {
            line_ok: true,
            branch_ok: true,
            function_ok: true,
            report_ok: true,
            threshold_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.line_ok && self.branch_ok && self.function_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.threshold_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.line_ok || !self.branch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.line_ok {
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
        let c = QaCoverage::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaCoverage::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaCoverage::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaCoverage::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaCoverage::new();
        c.line_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaCoverage::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaCoverage::default();
        assert!(c.all_ok());
    }
}
