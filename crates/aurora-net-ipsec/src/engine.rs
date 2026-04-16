/// net ipsec: negotiate, encrypt, decrypt, rekey, log
/// Phase 2272

#[derive(Debug, Clone)]
pub struct NetIpsec {
    pub negotiate_ok: bool,
    pub encrypt_ok: bool,
    pub decrypt_ok: bool,
    pub rekey_ok: bool,
    pub log_ok: bool,
}

impl Default for NetIpsec {
    fn default() -> Self {
        Self::new()
    }
}

impl NetIpsec {
    pub fn new() -> Self {
        Self {
            negotiate_ok: true,
            encrypt_ok: true,
            decrypt_ok: true,
            rekey_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.negotiate_ok && self.encrypt_ok && self.decrypt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rekey_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.negotiate_ok || !self.encrypt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.negotiate_ok {
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
        let c = NetIpsec::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetIpsec::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetIpsec::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetIpsec::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetIpsec::new();
        c.negotiate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetIpsec::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
