/// infra gateway2: create, configure, route, monitor, log
/// Phase 2146

#[derive(Debug, Clone)]
pub struct InfraGateway2 {
    pub create_ok: bool,
    pub configure_ok: bool,
    pub route_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraGateway2 {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraGateway2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            configure_ok: true,
            route_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.configure_ok && self.route_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.configure_ok
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
        let c = InfraGateway2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraGateway2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraGateway2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraGateway2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraGateway2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraGateway2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
