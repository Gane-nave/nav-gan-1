/// sec oidc: discover, authorize, token, verify, log
/// Phase 2045

#[derive(Debug, Clone)]
pub struct SecOidc {
    pub discover_ok: bool,
    pub authorize_ok: bool,
    pub token_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for SecOidc {
    fn default() -> Self {
        Self::new()
    }
}

impl SecOidc {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            authorize_ok: true,
            token_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.discover_ok && self.authorize_ok && self.token_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
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
        let c = SecOidc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecOidc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecOidc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecOidc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecOidc::new();
        c.discover_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecOidc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
