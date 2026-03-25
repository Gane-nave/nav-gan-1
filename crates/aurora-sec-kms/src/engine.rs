/// sec kms: create, encrypt, decrypt, rotate, log
/// Phase 2048

#[derive(Debug, Clone)]
pub struct SecKms {
    pub create_ok: bool,
    pub encrypt_ok: bool,
    pub decrypt_ok: bool,
    pub rotate_ok: bool,
    pub log_ok: bool,
}

impl Default for SecKms {
    fn default() -> Self {
        Self::new()
    }
}

impl SecKms {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            encrypt_ok: true,
            decrypt_ok: true,
            rotate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.encrypt_ok && self.decrypt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rotate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.encrypt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = SecKms::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecKms::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecKms::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecKms::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecKms::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecKms::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
