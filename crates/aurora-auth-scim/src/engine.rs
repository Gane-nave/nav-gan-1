/// auth scim: provision, deprovision, update, sync, log
/// Phase 1642

#[derive(Debug, Clone)]
pub struct AuthScim {
    pub provision_ok: bool,
    pub deprovision_ok: bool,
    pub update_ok: bool,
    pub sync_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthScim {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthScim {
    pub fn new() -> Self {
        Self {
            provision_ok: true,
            deprovision_ok: true,
            update_ok: true,
            sync_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.provision_ok && self.deprovision_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sync_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.provision_ok || !self.deprovision_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.provision_ok {
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
        let c = AuthScim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthScim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthScim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthScim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthScim::new();
        c.provision_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthScim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
