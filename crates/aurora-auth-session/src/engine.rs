/// auth session: create, validate, refresh, destroy, log
/// Phase 1636

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub create_ok: bool,
    pub validate_ok: bool,
    pub refresh_ok: bool,
    pub destroy_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthSession {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthSession {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            validate_ok: true,
            refresh_ok: true,
            destroy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.validate_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.destroy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.validate_ok
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
        let c = AuthSession::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthSession::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthSession::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthSession::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthSession::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthSession::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
