/// auth sso: login, validate, logout, refresh, log
/// Phase 1639

#[derive(Debug, Clone)]
pub struct AuthSso {
    pub login_ok: bool,
    pub validate_ok: bool,
    pub logout_ok: bool,
    pub refresh_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthSso {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthSso {
    pub fn new() -> Self {
        Self {
            login_ok: true,
            validate_ok: true,
            logout_ok: true,
            refresh_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.login_ok && self.validate_ok && self.logout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.refresh_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.login_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.login_ok {
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
        let c = AuthSso::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthSso::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthSso::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthSso::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthSso::new();
        c.login_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthSso::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
