/// sec vault2: store, retrieve, rotate, audit, log
/// Phase 2047

#[derive(Debug, Clone)]
pub struct SecVault2 {
    pub store_ok: bool,
    pub retrieve_ok: bool,
    pub rotate_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for SecVault2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SecVault2 {
    pub fn new() -> Self {
        Self {
            store_ok: true,
            retrieve_ok: true,
            rotate_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.store_ok && self.retrieve_ok && self.rotate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.store_ok || !self.retrieve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.store_ok {
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
        let c = SecVault2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecVault2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecVault2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecVault2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecVault2::new();
        c.store_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecVault2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
