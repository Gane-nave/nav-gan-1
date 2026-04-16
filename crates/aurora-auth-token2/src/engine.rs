/// auth token2: issue, validate, refresh, revoke, log
/// Phase 2059

#[derive(Debug, Clone)]
pub struct AuthToken2 {
    pub issue_ok: bool,
    pub validate_ok: bool,
    pub refresh_ok: bool,
    pub revoke_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthToken2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthToken2 {
    pub fn new() -> Self {
        Self {
            issue_ok: true,
            validate_ok: true,
            refresh_ok: true,
            revoke_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.issue_ok && self.validate_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.revoke_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.issue_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.issue_ok {
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
        let c = AuthToken2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthToken2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthToken2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthToken2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthToken2::new();
        c.issue_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthToken2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
