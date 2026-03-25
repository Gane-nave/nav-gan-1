/// auth kerberos: init, tgt, service, validate, log
/// Phase 2065

#[derive(Debug, Clone)]
pub struct AuthKerberos {
    pub init_ok: bool,
    pub tgt_ok: bool,
    pub service_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthKerberos {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthKerberos {
    pub fn new() -> Self {
        Self {
            init_ok: true,
            tgt_ok: true,
            service_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.init_ok && self.tgt_ok && self.service_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.init_ok || !self.tgt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.init_ok {
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
        let c = AuthKerberos::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthKerberos::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthKerberos::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthKerberos::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthKerberos::new();
        c.init_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthKerberos::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
