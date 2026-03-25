/// auth apikey2: generate, validate, rotate, revoke, log
/// Phase 2069

#[derive(Debug, Clone)]
pub struct AuthApikey2 {
    pub generate_ok: bool,
    pub validate_ok: bool,
    pub rotate_ok: bool,
    pub revoke_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthApikey2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthApikey2 {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            validate_ok: true,
            rotate_ok: true,
            revoke_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.validate_ok && self.rotate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.revoke_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.validate_ok
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
        let c = AuthApikey2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthApikey2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthApikey2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthApikey2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthApikey2::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthApikey2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
