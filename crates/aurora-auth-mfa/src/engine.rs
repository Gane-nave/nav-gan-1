/// auth mfa: enroll, verify, disable, recover, log
/// Phase 1631

#[derive(Debug, Clone)]
pub struct AuthMfa {
    pub enroll_ok: bool,
    pub verify_ok: bool,
    pub disable_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthMfa {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthMfa {
    pub fn new() -> Self {
        Self {
            enroll_ok: true,
            verify_ok: true,
            disable_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.enroll_ok && self.verify_ok && self.disable_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.enroll_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.enroll_ok {
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
        let c = AuthMfa::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthMfa::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthMfa::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthMfa::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthMfa::new();
        c.enroll_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthMfa::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
