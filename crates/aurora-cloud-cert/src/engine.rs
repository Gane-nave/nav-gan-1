/// cloud cert: generate, sign, renew, revoke, log
/// Phase 1457

#[derive(Debug, Clone)]
pub struct CloudCert {
    pub generate_ok: bool,
    pub sign_ok: bool,
    pub renew_ok: bool,
    pub revoke_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudCert {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudCert {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            sign_ok: true,
            renew_ok: true,
            revoke_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.sign_ok && self.renew_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.revoke_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.sign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudCert::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudCert::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudCert::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudCert::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudCert::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudCert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
