/// crypto x25519: exchange, keygen, shared, verify, log
/// Phase 1694

#[derive(Debug, Clone)]
pub struct CryptoX25519 {
    pub exchange_ok: bool,
    pub keygen_ok: bool,
    pub shared_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoX25519 {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoX25519 {
    pub fn new() -> Self {
        Self {
            exchange_ok: true,
            keygen_ok: true,
            shared_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.exchange_ok && self.keygen_ok && self.shared_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.exchange_ok || !self.keygen_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.exchange_ok {
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
        let c = CryptoX25519::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoX25519::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoX25519::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoX25519::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoX25519::new();
        c.exchange_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoX25519::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
