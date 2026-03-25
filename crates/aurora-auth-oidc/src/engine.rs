/// auth oidc: discover, authorize, token, userinfo, log
/// Phase 1643

#[derive(Debug, Clone)]
pub struct AuthOidc {
    pub discover_ok: bool,
    pub authorize_ok: bool,
    pub token_ok: bool,
    pub userinfo_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthOidc {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthOidc {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            authorize_ok: true,
            token_ok: true,
            userinfo_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.discover_ok && self.authorize_ok && self.token_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.userinfo_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.discover_ok || !self.authorize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.discover_ok {
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
        let c = AuthOidc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthOidc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthOidc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthOidc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthOidc::new();
        c.discover_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthOidc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
