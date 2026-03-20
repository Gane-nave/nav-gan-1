/// auth passkey: create, authenticate, manage, revoke, log
/// Phase 2068

#[derive(Debug, Clone)]
pub struct AuthPasskey {
    pub create_ok: bool,
    pub authenticate_ok: bool,
    pub manage_ok: bool,
    pub revoke_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthPasskey {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPasskey {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            authenticate_ok: true,
            manage_ok: true,
            revoke_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.authenticate_ok && self.manage_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.revoke_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.authenticate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = AuthPasskey::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthPasskey::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthPasskey::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthPasskey::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthPasskey::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthPasskey::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
