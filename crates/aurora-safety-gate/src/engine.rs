/// safety gate: check, enforce, override, audit, log
/// Phase 1483

#[derive(Debug, Clone)]
pub struct SafetyGate {
    pub check_ok: bool,
    pub enforce_ok: bool,
    pub override_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for SafetyGate {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyGate {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            enforce_ok: true,
            override_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.enforce_ok && self.override_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.enforce_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = SafetyGate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SafetyGate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SafetyGate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SafetyGate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SafetyGate::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SafetyGate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
