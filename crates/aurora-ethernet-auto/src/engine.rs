/// Automotive Ethernet: high-bandwidth vehicle network, 100BASE-T1
/// Phase 280

#[derive(Debug, Clone)]
pub struct AutomotiveEthernet {
    pub link_up: bool,
    pub speed_mbps: u32,
    pub packet_loss_pct: f64,
    pub latency_us: u32,
    pub switch_ok: bool,
    pub phy_ok: bool,
}

impl Default for AutomotiveEthernet {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomotiveEthernet {
    pub fn new() -> Self {
        Self {
            link_up: true,
            speed_mbps: 100,
            packet_loss_pct: 0.0,
            latency_us: 50,
            switch_ok: true,
            phy_ok: true,
        }
    }

    pub fn connected(&self) -> bool {
        self.link_up && self.phy_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.packet_loss_pct < 0.1 && self.latency_us < 1000
    }

    pub fn gigabit(&self) -> bool {
        self.speed_mbps >= 1000
    }

    pub fn needs_service(&self) -> bool {
        !self.link_up || !self.switch_ok || !self.phy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.link_up {
            return 0.0;
        }
        if !self.quality_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connected() {
        let e = AutomotiveEthernet::new();
        assert!(e.connected());
    }

    #[test]
    fn test_quality() {
        let e = AutomotiveEthernet::new();
        assert!(e.quality_ok());
    }

    #[test]
    fn test_not_gigabit() {
        let e = AutomotiveEthernet::new();
        assert!(!e.gigabit());
    }

    #[test]
    fn test_no_service() {
        let e = AutomotiveEthernet::new();
        assert!(!e.needs_service());
    }

    #[test]
    fn test_link_down() {
        let mut e = AutomotiveEthernet::new();
        e.link_up = false;
        assert!(e.needs_service());
    }

    #[test]
    fn test_health() {
        let e = AutomotiveEthernet::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
