/// auth webauthn: register, authenticate, verify, list, log
/// Phase 2067

#[derive(Debug, Clone)]
pub struct AuthWebauthn {
    pub register_ok: bool,
    pub authenticate_ok: bool,
    pub verify_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthWebauthn {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthWebauthn {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            authenticate_ok: true,
            verify_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.authenticate_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.authenticate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = AuthWebauthn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthWebauthn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthWebauthn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthWebauthn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthWebauthn::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthWebauthn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
