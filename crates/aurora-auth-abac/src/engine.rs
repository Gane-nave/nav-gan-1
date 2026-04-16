/// auth abac: evaluate, enforce, audit, update, log
/// Phase 1635

#[derive(Debug, Clone)]
pub struct AuthAbac {
    pub evaluate_ok: bool,
    pub enforce_ok: bool,
    pub audit_ok: bool,
    pub update_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthAbac {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthAbac {
    pub fn new() -> Self {
        Self {
            evaluate_ok: true,
            enforce_ok: true,
            audit_ok: true,
            update_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.evaluate_ok && self.enforce_ok && self.audit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.update_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.evaluate_ok || !self.enforce_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.evaluate_ok {
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
        let c = AuthAbac::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthAbac::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthAbac::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthAbac::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthAbac::new();
        c.evaluate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthAbac::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
