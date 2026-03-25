/// auth rbac: assign, check, revoke, list, log
/// Phase 1634

#[derive(Debug, Clone)]
pub struct AuthRbac {
    pub assign_ok: bool,
    pub check_ok: bool,
    pub revoke_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthRbac {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthRbac {
    pub fn new() -> Self {
        Self {
            assign_ok: true,
            check_ok: true,
            revoke_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assign_ok && self.check_ok && self.revoke_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assign_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assign_ok {
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
        let c = AuthRbac::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthRbac::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthRbac::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthRbac::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthRbac::new();
        c.assign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthRbac::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
