/// aurora-qa-static: qa static
/// Phase 2514

#[derive(Debug, Clone)]
pub struct QaStatic {
    pub lint_ok: bool,
    pub type_ok: bool,
    pub security_ok: bool,
    pub complexity_ok: bool,
    pub report_ok: bool,
}

impl Default for QaStatic {
    fn default() -> Self {
        Self::new()
    }
}

impl QaStatic {
    pub fn new() -> Self {
        Self {
            lint_ok: true,
            type_ok: true,
            security_ok: true,
            complexity_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.lint_ok && self.type_ok && self.security_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complexity_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.lint_ok || !self.type_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lint_ok {
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
        let c = QaStatic::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaStatic::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaStatic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaStatic::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaStatic::new();
        c.lint_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaStatic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaStatic::default();
        assert!(c.all_ok());
    }
}
