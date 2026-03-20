/// sec acl2: grant, revoke, check, list, log
/// Phase 2051

#[derive(Debug, Clone)]
pub struct SecAcl2 {
    pub grant_ok: bool,
    pub revoke_ok: bool,
    pub check_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for SecAcl2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SecAcl2 {
    pub fn new() -> Self {
        Self {
            grant_ok: true,
            revoke_ok: true,
            check_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.grant_ok && self.revoke_ok && self.check_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.grant_ok || !self.revoke_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.grant_ok {
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
        let c = SecAcl2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecAcl2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecAcl2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecAcl2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecAcl2::new();
        c.grant_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecAcl2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
