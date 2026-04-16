/// aurora-svc-auth: svc auth
/// Phase 2568

#[derive(Debug, Clone)]
pub struct SvcAuth {
    pub verify_ok: bool,
    pub issue_ok: bool,
    pub revoke_ok: bool,
    pub refresh_ok: bool,
    pub audit_ok: bool,
}

impl Default for SvcAuth {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcAuth {
    pub fn new() -> Self {
        Self {
            verify_ok: true,
            issue_ok: true,
            revoke_ok: true,
            refresh_ok: true,
            audit_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.verify_ok && self.issue_ok && self.revoke_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.refresh_ok && self.audit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.verify_ok || !self.issue_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.verify_ok {
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
        let c = SvcAuth::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcAuth::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcAuth::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcAuth::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcAuth::new();
        c.verify_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcAuth::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcAuth::default();
        assert!(c.all_ok());
    }
}
