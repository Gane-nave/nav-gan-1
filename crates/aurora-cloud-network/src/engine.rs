/// aurora-cloud-network: cloud network
/// Phase 2546

#[derive(Debug, Clone)]
pub struct CloudNetwork {
    pub route_ok: bool,
    pub firewall_ok: bool,
    pub nat_ok: bool,
    pub vpn_ok: bool,
    pub peering_ok: bool,
}

impl Default for CloudNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudNetwork {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            firewall_ok: true,
            nat_ok: true,
            vpn_ok: true,
            peering_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.firewall_ok && self.nat_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.vpn_ok && self.peering_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.firewall_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
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
        let c = CloudNetwork::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudNetwork::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudNetwork::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudNetwork::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudNetwork::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudNetwork::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = CloudNetwork::default();
        assert!(c.all_ok());
    }
}
