/// crypto tls13: handshake, verify, encrypt, decrypt, log
/// Phase 1696

#[derive(Debug, Clone)]
pub struct CryptoTls13 {
    pub handshake_ok: bool,
    pub verify_ok: bool,
    pub encrypt_ok: bool,
    pub decrypt_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoTls13 {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoTls13 {
    pub fn new() -> Self {
        Self {
            handshake_ok: true,
            verify_ok: true,
            encrypt_ok: true,
            decrypt_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.handshake_ok && self.verify_ok && self.encrypt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.decrypt_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.handshake_ok || !self.verify_ok
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
        let c = CryptoTls13::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoTls13::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoTls13::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoTls13::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoTls13::new();
        c.handshake_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoTls13::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
