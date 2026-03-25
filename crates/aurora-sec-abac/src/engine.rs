/// sec abac: evaluate, define, update, audit, log
/// Phase 2053

#[derive(Debug, Clone)]
pub struct SecAbac {
    pub evaluate_ok: bool,
    pub define_ok: bool,
    pub update_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for SecAbac {
    fn default() -> Self {
        Self::new()
    }
}

impl SecAbac {
    pub fn new() -> Self {
        Self {
            evaluate_ok: true,
            define_ok: true,
            update_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.evaluate_ok && self.define_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.evaluate_ok || !self.define_ok
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
        let c = SecAbac::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecAbac::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecAbac::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecAbac::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecAbac::new();
        c.evaluate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecAbac::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
