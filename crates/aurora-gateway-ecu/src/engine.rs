/// Gateway ECU: CAN routing, firewall, diagnostics
/// Phase 715

#[derive(Debug, Clone)]
pub struct GatewayEcu {
    pub routing_ok: bool,
    pub firewall_ok: bool,
    pub diag_ok: bool,
    pub latency_ok: bool,
    pub comm_ok: bool,
}

impl Default for GatewayEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl GatewayEcu {
    pub fn new() -> Self {
        Self {
            routing_ok: true,
            firewall_ok: true,
            diag_ok: true,
            latency_ok: true,
            comm_ok: true,
        }
    }

    pub fn network_ok(&self) -> bool {
        self.routing_ok && self.latency_ok && self.comm_ok
    }

    pub fn security_ok(&self) -> bool {
        self.firewall_ok && self.diag_ok
    }

    pub fn all_ok(&self) -> bool {
        self.network_ok() && self.security_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.routing_ok || !self.firewall_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.routing_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network() {
        let c = GatewayEcu::new();
        assert!(c.network_ok());
    }

    #[test]
    fn test_security() {
        let c = GatewayEcu::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GatewayEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = GatewayEcu::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_routing() {
        let mut c = GatewayEcu::new();
        c.routing_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = GatewayEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
