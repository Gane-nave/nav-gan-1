/// deploy istio: inject, route, policy, telemetry, log
/// Phase 1604

#[derive(Debug, Clone)]
pub struct DeployIstio {
    pub inject_ok: bool,
    pub route_ok: bool,
    pub policy_ok: bool,
    pub telemetry_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployIstio {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployIstio {
    pub fn new() -> Self {
        Self {
            inject_ok: true,
            route_ok: true,
            policy_ok: true,
            telemetry_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inject_ok && self.route_ok && self.policy_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.telemetry_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inject_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inject_ok {
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
        let c = DeployIstio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployIstio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployIstio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployIstio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployIstio::new();
        c.inject_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployIstio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
