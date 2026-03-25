/// crypto noise: handshake, encrypt, decrypt, rekey, log
/// Phase 1697

#[derive(Debug, Clone)]
pub struct CryptoNoise {
    pub handshake_ok: bool,
    pub encrypt_ok: bool,
    pub decrypt_ok: bool,
    pub rekey_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoNoise {
    pub fn new() -> Self {
        Self {
            handshake_ok: true,
            encrypt_ok: true,
            decrypt_ok: true,
            rekey_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.handshake_ok && self.encrypt_ok && self.decrypt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rekey_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.handshake_ok || !self.encrypt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.handshake_ok {
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
        let c = CryptoNoise::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoNoise::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoNoise::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoNoise::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoNoise::new();
        c.handshake_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoNoise::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
