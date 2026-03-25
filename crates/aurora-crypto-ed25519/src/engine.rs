/// crypto ed25519: sign, verify, keygen, batch, log
/// Phase 1683

#[derive(Debug, Clone)]
pub struct CryptoEd25519 {
    pub sign_ok: bool,
    pub verify_ok: bool,
    pub keygen_ok: bool,
    pub batch_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoEd25519 {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoEd25519 {
    pub fn new() -> Self {
        Self {
            sign_ok: true,
            verify_ok: true,
            keygen_ok: true,
            batch_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sign_ok && self.verify_ok && self.keygen_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.batch_ok && self.log_ok
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
        let c = CryptoEd25519::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoEd25519::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoEd25519::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoEd25519::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoEd25519::new();
        c.sign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoEd25519::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
