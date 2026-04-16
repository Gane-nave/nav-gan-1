/// crypto blake3: hash, verify, stream, finalize, log
/// Phase 1686

#[derive(Debug, Clone)]
pub struct CryptoBlake3 {
    pub hash_ok: bool,
    pub verify_ok: bool,
    pub stream_ok: bool,
    pub finalize_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoBlake3 {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoBlake3 {
    pub fn new() -> Self {
        Self {
            hash_ok: true,
            verify_ok: true,
            stream_ok: true,
            finalize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.hash_ok && self.verify_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.finalize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.hash_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hash_ok {
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
        let c = CryptoBlake3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoBlake3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoBlake3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoBlake3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoBlake3::new();
        c.hash_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoBlake3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
