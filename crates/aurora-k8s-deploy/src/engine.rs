/// aurora-k8s-deploy: k8s deploy
/// Phase 2556

#[derive(Debug, Clone)]
pub struct K8sDeploy {
    pub create_ok: bool,
    pub scale_ok: bool,
    pub rollout_ok: bool,
    pub rollback_ok: bool,
    pub watch_ok: bool,
}

impl Default for K8sDeploy {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sDeploy {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            scale_ok: true,
            rollout_ok: true,
            rollback_ok: true,
            watch_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.scale_ok && self.rollout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rollback_ok && self.watch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.scale_ok
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
        let c = K8sDeploy::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = K8sDeploy::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = K8sDeploy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = K8sDeploy::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = K8sDeploy::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = K8sDeploy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = K8sDeploy::default();
        assert!(c.all_ok());
    }
}
