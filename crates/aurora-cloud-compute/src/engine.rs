/// aurora-cloud-compute: cloud compute
/// Phase 2545

#[derive(Debug, Clone)]
pub struct CloudCompute {
    pub provision_ok: bool,
    pub scale_ok: bool,
    pub terminate_ok: bool,
    pub monitor_ok: bool,
    pub cost_ok: bool,
}

impl Default for CloudCompute {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudCompute {
    pub fn new() -> Self {
        Self {
            provision_ok: true,
            scale_ok: true,
            terminate_ok: true,
            monitor_ok: true,
            cost_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.provision_ok && self.scale_ok && self.terminate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.cost_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.provision_ok || !self.scale_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.provision_ok {
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
        let c = CloudCompute::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudCompute::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudCompute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudCompute::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudCompute::new();
        c.provision_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudCompute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = CloudCompute::default();
        assert!(c.all_ok());
    }
}
