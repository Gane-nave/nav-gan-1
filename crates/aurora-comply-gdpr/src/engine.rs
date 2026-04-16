/// comply gdpr: consent, access, erase, port, log
/// Phase 1488

#[derive(Debug, Clone)]
pub struct ComplyGdpr {
    pub consent_ok: bool,
    pub access_ok: bool,
    pub erase_ok: bool,
    pub port_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyGdpr {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyGdpr {
    pub fn new() -> Self {
        Self {
            consent_ok: true,
            access_ok: true,
            erase_ok: true,
            port_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.consent_ok && self.access_ok && self.erase_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.port_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.consent_ok || !self.access_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.consent_ok {
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
        let c = ComplyGdpr::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyGdpr::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyGdpr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyGdpr::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyGdpr::new();
        c.consent_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyGdpr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
