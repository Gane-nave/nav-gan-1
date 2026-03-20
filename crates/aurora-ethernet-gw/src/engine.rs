/// ethernet gw: switch, route, vlan, qos, check
/// Phase 1272

#[derive(Debug, Clone)]
pub struct EthernetGw {
    pub switch_ok: bool,
    pub route_ok: bool,
    pub vlan_ok: bool,
    pub qos_ok: bool,
    pub check_ok: bool,
}

impl Default for EthernetGw {
    fn default() -> Self {
        Self::new()
    }
}

impl EthernetGw {
    pub fn new() -> Self {
        Self {
            switch_ok: true,
            route_ok: true,
            vlan_ok: true,
            qos_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.switch_ok && self.route_ok && self.vlan_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.qos_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.switch_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.switch_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = EthernetGw::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EthernetGw::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EthernetGw::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EthernetGw::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EthernetGw::new();
        c.switch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EthernetGw::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
