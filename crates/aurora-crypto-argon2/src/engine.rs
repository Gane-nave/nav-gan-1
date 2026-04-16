/// crypto argon2: hash, verify, config, bench, log
/// Phase 1687

#[derive(Debug, Clone)]
pub struct CryptoArgon2 {
    pub hash_ok: bool,
    pub verify_ok: bool,
    pub config_ok: bool,
    pub bench_ok: bool,
    pub log_ok: bool,
}

impl Default for CryptoArgon2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoArgon2 {
    pub fn new() -> Self {
        Self {
            hash_ok: true,
            verify_ok: true,
            config_ok: true,
            bench_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.hash_ok && self.verify_ok && self.config_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.bench_ok && self.log_ok
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
        let c = CryptoArgon2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CryptoArgon2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CryptoArgon2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CryptoArgon2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CryptoArgon2::new();
        c.hash_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CryptoArgon2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
