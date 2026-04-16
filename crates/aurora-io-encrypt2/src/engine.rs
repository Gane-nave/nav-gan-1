/// io encrypt2: encrypt, decrypt, stream, flush, log
/// Phase 1997

#[derive(Debug, Clone)]
pub struct IoEncrypt2 {
    pub encrypt_ok: bool,
    pub decrypt_ok: bool,
    pub stream_ok: bool,
    pub flush_ok: bool,
    pub log_ok: bool,
}

impl Default for IoEncrypt2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IoEncrypt2 {
    pub fn new() -> Self {
        Self {
            encrypt_ok: true,
            decrypt_ok: true,
            stream_ok: true,
            flush_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.encrypt_ok && self.decrypt_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flush_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.encrypt_ok || !self.decrypt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.encrypt_ok {
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
        let c = IoEncrypt2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoEncrypt2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoEncrypt2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoEncrypt2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoEncrypt2::new();
        c.encrypt_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoEncrypt2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
