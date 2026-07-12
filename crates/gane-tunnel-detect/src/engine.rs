/// Tunnel detection and navigation: ventilation, lighting adaptation, GPS loss handling.
#[derive(Debug, Clone, PartialEq)]
pub enum TunnelType {
    Road,
    Mountain,
    Underwater,
    Urban,
    Rail,
    Mining,
}

impl TunnelType {
    pub fn gps_loss_expected(&self) -> bool {
        true
    }

    pub fn ventilation_risk(&self) -> f64 {
        match self {
            TunnelType::Underwater => 0.9,
            TunnelType::Mountain => 0.7,
            TunnelType::Mining => 0.8,
            TunnelType::Urban => 0.4,
            TunnelType::Road => 0.5,
            TunnelType::Rail => 0.3,
        }
    }

    pub fn lighting_transition(&self) -> f64 {
        match self {
            TunnelType::Mountain => 0.9,
            TunnelType::Underwater => 0.7,
            TunnelType::Urban => 0.5,
            _ => 0.6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tunnel {
    pub name: String,
    pub tunnel_type: TunnelType,
    pub length_m: f64,
    pub lanes: u8,
    pub speed_limit_kmh: f64,
    pub has_emergency_exits: bool,
    pub has_ventilation: bool,
    pub clearance_m: f64,
}

impl Tunnel {
    pub fn new(name: &str, tunnel_type: TunnelType, length_m: f64) -> Self {
        Self {
            name: name.to_string(),
            tunnel_type,
            length_m,
            lanes: 2,
            speed_limit_kmh: 80.0,
            has_emergency_exits: length_m > 500.0,
            has_ventilation: length_m > 300.0,
            clearance_m: 4.5,
        }
    }

    pub fn transit_time_sec(&self) -> f64 {
        let speed_ms = self.speed_limit_kmh / 3.6;
        if speed_ms <= 0.0 {
            return f64::INFINITY;
        }
        self.length_m / speed_ms
    }

    pub fn gps_blackout_sec(&self) -> f64 {
        self.transit_time_sec()
    }

    pub fn is_long(&self) -> bool {
        self.length_m > 1000.0
    }

    pub fn safety_score(&self) -> f64 {
        let mut score = 70.0;
        if self.has_emergency_exits {
            score += 15.0;
        }
        if self.has_ventilation {
            score += 10.0;
        }
        let vent_risk = self.tunnel_type.ventilation_risk() * 10.0;
        (score - vent_risk).clamp(0.0, 100.0)
    }

    pub fn vehicle_can_enter(&self, height_m: f64) -> bool {
        height_m < self.clearance_m
    }

    pub fn recommended_headway_m(&self) -> f64 {
        let base = self.speed_limit_kmh / 3.6 * 2.0;
        if self.is_long() {
            base * 1.5
        } else {
            base
        }
    }

    pub fn risk_level(&self) -> &str {
        let score = self.safety_score();
        if score >= 80.0 {
            "Low"
        } else if score >= 60.0 {
            "Medium"
        } else if score >= 40.0 {
            "High"
        } else {
            "Critical"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gps_loss() {
        assert!(TunnelType::Road.gps_loss_expected());
    }

    #[test]
    fn test_ventilation_risk() {
        assert!(TunnelType::Underwater.ventilation_risk() > TunnelType::Urban.ventilation_risk());
    }

    #[test]
    fn test_transit_time() {
        let t = Tunnel::new("Test", TunnelType::Road, 1000.0);
        assert!(t.transit_time_sec() > 30.0 && t.transit_time_sec() < 60.0);
    }

    #[test]
    fn test_gps_blackout() {
        let t = Tunnel::new("Test", TunnelType::Mountain, 500.0);
        assert!(t.gps_blackout_sec() > 0.0);
    }

    #[test]
    fn test_is_long() {
        assert!(Tunnel::new("Long", TunnelType::Mountain, 2000.0).is_long());
        assert!(!Tunnel::new("Short", TunnelType::Urban, 200.0).is_long());
    }

    #[test]
    fn test_safety_score() {
        let t = Tunnel::new("Safe", TunnelType::Road, 800.0);
        assert!(t.safety_score() > 50.0);
    }

    #[test]
    fn test_clearance() {
        let t = Tunnel::new("Test", TunnelType::Road, 500.0);
        assert!(t.vehicle_can_enter(3.5));
        assert!(!t.vehicle_can_enter(5.0));
    }

    #[test]
    fn test_headway_long() {
        let long = Tunnel::new("Long", TunnelType::Mountain, 2000.0);
        let short = Tunnel::new("Short", TunnelType::Road, 200.0);
        assert!(long.recommended_headway_m() > short.recommended_headway_m());
    }

    #[test]
    fn test_risk_level() {
        let t = Tunnel::new("Safe", TunnelType::Road, 800.0);
        assert!(t.risk_level() == "Low" || t.risk_level() == "Medium");
    }

    #[test]
    fn test_emergency_exits_auto() {
        assert!(Tunnel::new("Long", TunnelType::Road, 600.0).has_emergency_exits);
        assert!(!Tunnel::new("Short", TunnelType::Road, 100.0).has_emergency_exits);
    }

    #[test]
    fn test_ventilation_auto() {
        assert!(Tunnel::new("Long", TunnelType::Road, 400.0).has_ventilation);
        assert!(!Tunnel::new("Short", TunnelType::Road, 100.0).has_ventilation);
    }
}
