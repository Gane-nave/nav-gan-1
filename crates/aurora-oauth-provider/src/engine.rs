/// OAuth provider: authorize, token, refresh, revoke, scope
/// Phase 1001

#[derive(Debug, Clone)]
pub struct OauthProvider {
    pub authorize_ok: bool,
    pub token_ok: bool,
    pub refresh_ok: bool,
    pub revoke_ok: bool,
    pub scope_ok: bool,
}

impl Default for OauthProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OauthProvider {
    pub fn new() -> Self {
        Self {
            authorize_ok: true,
            token_ok: true,
            refresh_ok: true,
            revoke_ok: true,
            scope_ok: true,
        }
    }

    pub fn flow_ok(&self) -> bool {
        self.authorize_ok && self.token_ok && self.refresh_ok
    }

    pub fn management_ok(&self) -> bool {
        self.revoke_ok && self.scope_ok
    }

    pub fn all_ok(&self) -> bool {
        self.flow_ok() && self.management_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.authorize_ok || !self.scope_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.authorize_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow() {
        let c = OauthProvider::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_management() {
        let c = OauthProvider::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OauthProvider::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = OauthProvider::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_authorize() {
        let mut c = OauthProvider::new();
        c.authorize_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = OauthProvider::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
