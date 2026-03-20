/// deploy helm2: install, upgrade, rollback, status, log
/// Phase 2114

#[derive(Debug, Clone)]
pub struct DeployHelm2 {
    pub install_ok: bool,
    pub upgrade_ok: bool,
    pub rollback_ok: bool,
    pub status_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployHelm2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployHelm2 {
    pub fn new() -> Self {
        Self {
            install_ok: true,
            upgrade_ok: true,
            rollback_ok: true,
            status_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.install_ok && self.upgrade_ok && self.rollback_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.status_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.install_ok || !self.upgrade_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.install_ok {
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
        let c = DeployHelm2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployHelm2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployHelm2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployHelm2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployHelm2::new();
        c.install_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployHelm2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
