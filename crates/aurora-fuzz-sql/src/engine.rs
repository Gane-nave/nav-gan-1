/// aurora-fuzz-sql: fuzz sql
/// Phase 2496

#[derive(Debug, Clone)]
pub struct FuzzSql {
    pub generate_ok: bool,
    pub inject_ok: bool,
    pub sanitize_ok: bool,
    pub validate_ok: bool,
    pub report_ok: bool,
}

impl Default for FuzzSql {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzSql {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            inject_ok: true,
            sanitize_ok: true,
            validate_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.inject_ok && self.sanitize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.inject_ok
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
        let c = FuzzSql::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuzzSql::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuzzSql::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuzzSql::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuzzSql::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuzzSql::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = FuzzSql::default();
        assert!(c.all_ok());
    }
}
