/// deploy helm: template, install, upgrade, rollback, log
/// Phase 1599

#[derive(Debug, Clone)]
pub struct DeployHelm {
    pub template_ok: bool,
    pub install_ok: bool,
    pub upgrade_ok: bool,
    pub rollback_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployHelm {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployHelm {
    pub fn new() -> Self {
        Self {
            template_ok: true,
            install_ok: true,
            upgrade_ok: true,
            rollback_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.template_ok && self.install_ok && self.upgrade_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rollback_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.template_ok || !self.install_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.template_ok {
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
        let c = DeployHelm::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployHelm::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployHelm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployHelm::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployHelm::new();
        c.template_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployHelm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
