/// infra compute: launch, scale, terminate, monitor, log
/// Phase 2142

#[derive(Debug, Clone)]
pub struct InfraCompute {
    pub launch_ok: bool,
    pub scale_ok: bool,
    pub terminate_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraCompute {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraCompute {
    pub fn new() -> Self {
        Self {
            launch_ok: true,
            scale_ok: true,
            terminate_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.launch_ok && self.scale_ok && self.terminate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.launch_ok || !self.scale_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.launch_ok {
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
        let c = InfraCompute::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraCompute::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraCompute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraCompute::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraCompute::new();
        c.launch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraCompute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
