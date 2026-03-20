/// auth kerberos: auth, ticket, renew, validate, log
/// Phase 1630

#[derive(Debug, Clone)]
pub struct AuthKerberos {
    pub auth_ok: bool,
    pub ticket_ok: bool,
    pub renew_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthKerberos {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthKerberos {
    pub fn new() -> Self {
        Self {
            auth_ok: true,
            ticket_ok: true,
            renew_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.auth_ok && self.ticket_ok && self.renew_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.auth_ok || !self.ticket_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.auth_ok {
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
        let c = AuthKerberos::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthKerberos::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthKerberos::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthKerberos::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthKerberos::new();
        c.auth_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthKerberos::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
