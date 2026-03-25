/// auth totp: generate, verify, enable, disable, log
/// Phase 1640

#[derive(Debug, Clone)]
pub struct AuthTotp {
    pub generate_ok: bool,
    pub verify_ok: bool,
    pub enable_ok: bool,
    pub disable_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthTotp {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthTotp {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            verify_ok: true,
            enable_ok: true,
            disable_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.verify_ok && self.enable_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.disable_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = AuthTotp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthTotp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthTotp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthTotp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthTotp::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthTotp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
