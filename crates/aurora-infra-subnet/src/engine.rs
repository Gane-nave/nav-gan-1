/// infra subnet: create, configure, route, monitor, log
/// Phase 2131

#[derive(Debug, Clone)]
pub struct InfraSubnet {
    pub create_ok: bool,
    pub configure_ok: bool,
    pub route_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraSubnet {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraSubnet {
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
        let c = InfraSubnet::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraSubnet::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraSubnet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraSubnet::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraSubnet::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraSubnet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
