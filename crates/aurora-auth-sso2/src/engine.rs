/// auth sso2: initiate, callback, validate, logout, log
/// Phase 2062

#[derive(Debug, Clone)]
pub struct AuthSso2 {
    pub initiate_ok: bool,
    pub callback_ok: bool,
    pub validate_ok: bool,
    pub logout_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthSso2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthSso2 {
    pub fn new() -> Self {
        Self {
            initiate_ok: true,
            callback_ok: true,
            validate_ok: true,
            logout_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.initiate_ok && self.callback_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.logout_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.initiate_ok || !self.callback_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.initiate_ok {
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
        let c = AuthSso2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthSso2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthSso2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthSso2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthSso2::new();
        c.initiate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthSso2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
