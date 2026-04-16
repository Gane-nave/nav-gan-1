/// net tls3: handshake, encrypt, decrypt, verify, log
/// Phase 2262

#[derive(Debug, Clone)]
pub struct NetTls3 {
    pub handshake_ok: bool,
    pub encrypt_ok: bool,
    pub decrypt_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for NetTls3 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetTls3 {
    pub fn new() -> Self {
        Self {
            handshake_ok: true,
            encrypt_ok: true,
            decrypt_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.handshake_ok && self.encrypt_ok && self.decrypt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
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
        let c = NetTls3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetTls3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetTls3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetTls3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetTls3::new();
        c.handshake_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetTls3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
