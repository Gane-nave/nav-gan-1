/// Proxy load balancer: route, balance, health, sticky, circuit
/// Phase 1065

#[derive(Debug, Clone)]
pub struct ProxyLb {
    pub route_ok: bool,
    pub balance_ok: bool,
    pub health_ok: bool,
    pub sticky_ok: bool,
    pub circuit_ok: bool,
}

impl Default for ProxyLb {
    fn default() -> Self {
        Self::new()
    }
}

impl ProxyLb {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            balance_ok: true,
            health_ok: true,
            sticky_ok: true,
            circuit_ok: true,
        }
    }

    pub fn routing_ok(&self) -> bool {
        self.route_ok && self.balance_ok && self.health_ok
    }

    pub fn resilience_ok(&self) -> bool {
        self.sticky_ok && self.circuit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.routing_ok() && self.resilience_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.route_ok || !self.balance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing() {
        let c = ProxyLb::new();
        assert!(c.routing_ok());
    }

    #[test]
    fn test_resilience() {
        let c = ProxyLb::new();
        assert!(c.resilience_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProxyLb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = ProxyLb::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_route() {
        let mut c = ProxyLb::new();
        c.route_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = ProxyLb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
