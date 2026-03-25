/// sec cert2: generate, sign, verify, revoke, log
/// Phase 2046

#[derive(Debug, Clone)]
pub struct SecCert2 {
    pub generate_ok: bool,
    pub sign_ok: bool,
    pub verify_ok: bool,
    pub revoke_ok: bool,
    pub log_ok: bool,
}

impl Default for SecCert2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SecCert2 {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            sign_ok: true,
            verify_ok: true,
            revoke_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.sign_ok && self.verify_ok
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
        if !self.generate_ok {
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
        let c = SecCert2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecCert2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecCert2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecCert2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecCert2::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecCert2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
