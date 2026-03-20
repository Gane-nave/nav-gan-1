/// Automotive Ethernet: PHY, switch, VLAN, QoS
/// Phase 730

#[derive(Debug, Clone)]
pub struct EthernetAuto {
    pub phy_ok: bool,
    pub switch_ok: bool,
    pub vlan_ok: bool,
    pub qos_ok: bool,
    pub link_ok: bool,
}

impl Default for EthernetAuto {
    fn default() -> Self {
        Self::new()
    }
}

impl EthernetAuto {
    pub fn new() -> Self {
        Self {
            phy_ok: true,
            switch_ok: true,
            vlan_ok: true,
            qos_ok: true,
            link_ok: true,
        }
    }

    pub fn network_ok(&self) -> bool {
        self.phy_ok && self.switch_ok && self.link_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.vlan_ok && self.qos_ok
    }

    pub fn all_ok(&self) -> bool {
        self.network_ok() && self.quality_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.phy_ok || !self.link_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.phy_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network() {
        let c = EthernetAuto::new();
        assert!(c.network_ok());
    }

    #[test]
    fn test_quality() {
        let c = EthernetAuto::new();
        assert!(c.quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EthernetAuto::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EthernetAuto::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_phy() {
        let mut c = EthernetAuto::new();
        c.phy_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EthernetAuto::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
