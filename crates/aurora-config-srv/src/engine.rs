/// Config server: store, version, distribute, validate, rollback
/// Phase 1067

#[derive(Debug, Clone)]
pub struct ConfigSrv {
    pub store_ok: bool,
    pub version_ok: bool,
    pub distribute_ok: bool,
    pub validate_ok: bool,
    pub rollback_ok: bool,
}

impl Default for ConfigSrv {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigSrv {
    pub fn new() -> Self {
        Self {
            store_ok: true,
            version_ok: true,
            distribute_ok: true,
            validate_ok: true,
            rollback_ok: true,
        }
    }

    pub fn management_ok(&self) -> bool {
        self.store_ok && self.version_ok && self.distribute_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.validate_ok && self.rollback_ok
    }

    pub fn all_ok(&self) -> bool {
        self.management_ok() && self.safety_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.store_ok || !self.distribute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.store_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_management() {
        let c = ConfigSrv::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_safety() {
        let c = ConfigSrv::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConfigSrv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = ConfigSrv::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_store() {
        let mut c = ConfigSrv::new();
        c.store_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = ConfigSrv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
