/// sec hmac2: sign, verify, rotate, validate, log
/// Phase 2041

#[derive(Debug, Clone)]
pub struct SecHmac2 {
    pub sign_ok: bool,
    pub verify_ok: bool,
    pub rotate_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for SecHmac2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SecHmac2 {
    pub fn new() -> Self {
        Self {
            sign_ok: true,
            verify_ok: true,
            rotate_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sign_ok && self.verify_ok && self.rotate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sign_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sign_ok {
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
        let c = SecHmac2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecHmac2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecHmac2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecHmac2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecHmac2::new();
        c.sign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecHmac2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
