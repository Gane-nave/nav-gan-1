/// auth mfa2: enroll, verify, disable, backup, log
/// Phase 2061

#[derive(Debug, Clone)]
pub struct AuthMfa2 {
    pub enroll_ok: bool,
    pub verify_ok: bool,
    pub disable_ok: bool,
    pub backup_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthMfa2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthMfa2 {
    pub fn new() -> Self {
        Self {
            enroll_ok: true,
            verify_ok: true,
            disable_ok: true,
            backup_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.enroll_ok && self.verify_ok && self.disable_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.backup_ok && self.log_ok
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
        let c = AuthMfa2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthMfa2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthMfa2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthMfa2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthMfa2::new();
        c.enroll_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthMfa2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
