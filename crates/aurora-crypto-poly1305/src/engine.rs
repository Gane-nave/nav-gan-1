/// crypto poly1305: auth, verify, keygen, tag, log
/// Phase 1691

#[derive(Debug, Clone)]
pub struct CryptoPoly1305 {
    pub auth_ok: bool,
    pub verify_ok: bool,
    pub keygen_ok: bool,
    pub tag_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoPoly1305 {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoPoly1305 {
    pub fn new() -> Self {
        Self {
            auth_ok: true,
            verify_ok: true,
            keygen_ok: true,
            tag_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.auth_ok && self.verify_ok && self.keygen_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tag_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.auth_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.auth_ok {
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
        let c = CryptoPoly1305::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoPoly1305::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoPoly1305::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoPoly1305::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoPoly1305::new();
        c.auth_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoPoly1305::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
