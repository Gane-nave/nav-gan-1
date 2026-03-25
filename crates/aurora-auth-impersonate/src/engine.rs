/// auth impersonate: start, validate, stop, audit, log
/// Phase 2074

#[derive(Debug, Clone)]
pub struct AuthImpersonate {
    pub start_ok: bool,
    pub validate_ok: bool,
    pub stop_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthImpersonate {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthImpersonate {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            validate_ok: true,
            stop_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.validate_ok && self.stop_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.validate_ok
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
        let c = AuthImpersonate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthImpersonate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthImpersonate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthImpersonate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthImpersonate::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthImpersonate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
