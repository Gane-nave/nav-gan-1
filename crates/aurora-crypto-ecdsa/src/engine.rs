/// crypto ecdsa: sign, verify, keygen, recover, log
/// Phase 1682

#[derive(Debug, Clone)]
pub struct CryptoEcdsa {
    pub sign_ok: bool,
    pub verify_ok: bool,
    pub keygen_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoEcdsa {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoEcdsa {
    pub fn new() -> Self {
        Self {
            sign_ok: true,
            verify_ok: true,
            keygen_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sign_ok && self.verify_ok && self.keygen_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
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
        let c = CryptoEcdsa::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoEcdsa::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoEcdsa::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoEcdsa::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoEcdsa::new();
        c.sign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoEcdsa::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
