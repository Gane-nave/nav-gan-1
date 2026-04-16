/// auth radius: auth, acct, coa, disconnect, log
/// Phase 2064

#[derive(Debug, Clone)]
pub struct AuthRadius {
    pub auth_ok: bool,
    pub acct_ok: bool,
    pub coa_ok: bool,
    pub disconnect_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthRadius {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthRadius {
    pub fn new() -> Self {
        Self {
            auth_ok: true,
            acct_ok: true,
            coa_ok: true,
            disconnect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.auth_ok && self.acct_ok && self.coa_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.disconnect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.auth_ok || !self.acct_ok
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
        let c = AuthRadius::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthRadius::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthRadius::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthRadius::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthRadius::new();
        c.auth_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthRadius::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
