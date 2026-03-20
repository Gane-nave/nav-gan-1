/// Helm chart: template, install, upgrade, rollback, test
/// Phase 1070

#[derive(Debug, Clone)]
pub struct HelmChart {
    pub template_ok: bool,
    pub install_ok: bool,
    pub upgrade_ok: bool,
    pub rollback_ok: bool,
    pub test_ok: bool,
}

impl Default for HelmChart {
    fn default() -> Self {
        Self::new()
    }
}

impl HelmChart {
    pub fn new() -> Self {
        Self {
            template_ok: true,
            install_ok: true,
            upgrade_ok: true,
            rollback_ok: true,
            test_ok: true,
        }
    }

    pub fn deployment_ok(&self) -> bool {
        self.template_ok && self.install_ok && self.upgrade_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.rollback_ok && self.test_ok
    }

    pub fn all_ok(&self) -> bool {
        self.deployment_ok() && self.safety_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.template_ok || !self.install_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.template_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment() {
        let c = HelmChart::new();
        assert!(c.deployment_ok());
    }

    #[test]
    fn test_safety() {
        let c = HelmChart::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HelmChart::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = HelmChart::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_template() {
        let mut c = HelmChart::new();
        c.template_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = HelmChart::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
