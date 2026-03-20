/// deploy k8s: apply, scale, rollout, service, log
/// Phase 1603

#[derive(Debug, Clone)]
pub struct DeployK8s {
    pub apply_ok: bool,
    pub scale_ok: bool,
    pub rollout_ok: bool,
    pub service_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployK8s {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployK8s {
    pub fn new() -> Self {
        Self {
            apply_ok: true,
            scale_ok: true,
            rollout_ok: true,
            service_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.apply_ok && self.scale_ok && self.rollout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.service_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.apply_ok || !self.scale_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.apply_ok {
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
        let c = DeployK8s::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployK8s::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployK8s::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployK8s::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployK8s::new();
        c.apply_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployK8s::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
