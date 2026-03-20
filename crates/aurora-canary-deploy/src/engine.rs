/// Canary deployment: split, monitor, promote, rollback, metric
/// Phase 1076

#[derive(Debug, Clone)]
pub struct CanaryDeploy {
    pub split_ok: bool,
    pub monitor_ok: bool,
    pub promote_ok: bool,
    pub rollback_ok: bool,
    pub metric_ok: bool,
}

impl Default for CanaryDeploy {
    fn default() -> Self {
        Self::new()
    }
}

impl CanaryDeploy {
    pub fn new() -> Self {
        Self {
            split_ok: true,
            monitor_ok: true,
            promote_ok: true,
            rollback_ok: true,
            metric_ok: true,
        }
    }

    pub fn deployment_ok(&self) -> bool {
        self.split_ok && self.monitor_ok && self.promote_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.rollback_ok && self.metric_ok
    }

    pub fn all_ok(&self) -> bool {
        self.deployment_ok() && self.safety_ok()
    }

    pub fn needs_rollback(&self) -> bool {
        !self.promote_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.split_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment() {
        let c = CanaryDeploy::new();
        assert!(c.deployment_ok());
    }

    #[test]
    fn test_safety() {
        let c = CanaryDeploy::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CanaryDeploy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rollback() {
        let c = CanaryDeploy::new();
        assert!(!c.needs_rollback());
    }

    #[test]
    fn test_promote() {
        let mut c = CanaryDeploy::new();
        c.promote_ok = false;
        assert!(c.needs_rollback());
    }

    #[test]
    fn test_health() {
        let c = CanaryDeploy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
