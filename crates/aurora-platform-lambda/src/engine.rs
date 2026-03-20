/// platform lambda: deploy, invoke, configure, monitor, log
/// Phase 1816

#[derive(Debug, Clone)]
pub struct PlatformLambda {
    pub deploy_ok: bool,
    pub invoke_ok: bool,
    pub configure_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for PlatformLambda {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformLambda {
    pub fn new() -> Self {
        Self {
            deploy_ok: true,
            invoke_ok: true,
            configure_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.deploy_ok && self.invoke_ok && self.configure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.deploy_ok || !self.invoke_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.deploy_ok {
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
        let c = PlatformLambda::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PlatformLambda::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PlatformLambda::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PlatformLambda::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PlatformLambda::new();
        c.deploy_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PlatformLambda::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
