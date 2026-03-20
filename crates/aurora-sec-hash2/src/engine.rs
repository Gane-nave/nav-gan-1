/// sec hash2: sha256, sha512, blake3, verify, log
/// Phase 2040

#[derive(Debug, Clone)]
pub struct SecHash2 {
    pub sha256_ok: bool,
    pub sha512_ok: bool,
    pub blake3_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for SecHash2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SecHash2 {
    pub fn new() -> Self {
        Self {
            sha256_ok: true,
            sha512_ok: true,
            blake3_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sha256_ok && self.sha512_ok && self.blake3_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sha256_ok || !self.sha512_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sha256_ok {
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
        let c = SecHash2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecHash2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecHash2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecHash2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecHash2::new();
        c.sha256_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecHash2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
