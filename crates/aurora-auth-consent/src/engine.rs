/// auth consent: request, grant, revoke, audit, log
/// Phase 2072

#[derive(Debug, Clone)]
pub struct AuthConsent {
    pub request_ok: bool,
    pub grant_ok: bool,
    pub revoke_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthConsent {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthConsent {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            grant_ok: true,
            revoke_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.grant_ok && self.revoke_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.grant_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = AuthConsent::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthConsent::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthConsent::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthConsent::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthConsent::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthConsent::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
