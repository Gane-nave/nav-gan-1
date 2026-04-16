/// sec oauth3: authorize, token, refresh, revoke, log
/// Phase 2043

#[derive(Debug, Clone)]
pub struct SecOauth3 {
    pub authorize_ok: bool,
    pub token_ok: bool,
    pub refresh_ok: bool,
    pub revoke_ok: bool,
    pub log_ok: bool,
}

impl Default for SecOauth3 {
    fn default() -> Self {
        Self::new()
    }
}

impl SecOauth3 {
    pub fn new() -> Self {
        Self {
            authorize_ok: true,
            token_ok: true,
            refresh_ok: true,
            revoke_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.authorize_ok && self.token_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.revoke_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.authorize_ok || !self.token_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.authorize_ok {
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
        let c = SecOauth3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecOauth3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecOauth3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecOauth3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecOauth3::new();
        c.authorize_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecOauth3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
