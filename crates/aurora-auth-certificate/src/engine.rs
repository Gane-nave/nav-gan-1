/// auth certificate: issue, verify, revoke, renew, log
/// Phase 1638

#[derive(Debug, Clone)]
pub struct AuthCertificate {
    pub issue_ok: bool,
    pub verify_ok: bool,
    pub revoke_ok: bool,
    pub renew_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthCertificate {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthCertificate {
    pub fn new() -> Self {
        Self {
            issue_ok: true,
            verify_ok: true,
            revoke_ok: true,
            renew_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.issue_ok && self.verify_ok && self.revoke_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.renew_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.issue_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.issue_ok {
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
        let c = AuthCertificate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthCertificate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthCertificate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthCertificate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthCertificate::new();
        c.issue_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthCertificate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
