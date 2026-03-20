/// Bridge navigation: weight limits, clearance, wind advisories, ice detection.
#[derive(Debug, Clone, PartialEq)]
pub enum BridgeType {
    Beam,
    Arch,
    Suspension,
    CableStayed,
    Truss,
    Cantilever,
    Movable,
    Pontoon,
}

impl BridgeType {
    pub fn wind_sensitivity(&self) -> f64 {
        match self {
            BridgeType::Suspension => 0.9,
            BridgeType::CableStayed => 0.8,
            BridgeType::Pontoon => 0.7,
            BridgeType::Movable => 0.5,
            BridgeType::Cantilever => 0.4,
            BridgeType::Truss => 0.3,
            BridgeType::Arch => 0.2,
            BridgeType::Beam => 0.1,
        }
    }

    pub fn ice_risk(&self) -> f64 {
        match self {
            BridgeType::Suspension | BridgeType::CableStayed => 0.9,
            BridgeType::Pontoon => 0.3,
            _ => 0.7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bridge {
    pub name: String,
    pub bridge_type: BridgeType,
    pub length_m: f64,
    pub clearance_m: f64,
    pub weight_limit_tonnes: f64,
    pub width_m: f64,
    pub wind_speed_kmh: f64,
    pub temperature_c: f64,
}

impl Bridge {
    pub fn new(name: &str, bridge_type: BridgeType, length_m: f64) -> Self {
        Self {
            name: name.to_string(),
            bridge_type,
            length_m,
            clearance_m: 4.5,
            weight_limit_tonnes: 40.0,
            width_m: 12.0,
            wind_speed_kmh: 0.0,
            temperature_c: 20.0,
        }
    }

    pub fn vehicle_can_cross(&self, height_m: f64, weight_tonnes: f64) -> bool {
        height_m < self.clearance_m && weight_tonnes <= self.weight_limit_tonnes
    }

    pub fn wind_advisory(&self) -> bool {
        self.wind_speed_kmh * self.bridge_type.wind_sensitivity() > 40.0
    }

    pub fn wind_closure(&self) -> bool {
        self.wind_speed_kmh * self.bridge_type.wind_sensitivity() > 70.0
    }

    pub fn ice_warning(&self) -> bool {
        self.temperature_c < 3.0 && self.bridge_type.ice_risk() > 0.5
    }

    pub fn safe_speed_kmh(&self) -> f64 {
        let base = 80.0;
        let wind_factor = if self.wind_advisory() { 0.6 } else { 1.0 };
        let ice_factor = if self.ice_warning() { 0.5 } else { 1.0 };
        let speed: f64 = base * wind_factor * ice_factor;
        speed.max(20.0)
    }

    pub fn crossing_time_sec(&self) -> f64 {
        let speed_ms = self.safe_speed_kmh() / 3.6;
        if speed_ms <= 0.0 {
            return f64::INFINITY;
        }
        self.length_m / speed_ms
    }

    pub fn risk_score(&self) -> f64 {
        let wind = self.wind_speed_kmh * self.bridge_type.wind_sensitivity() * 0.5;
        let ice = if self.ice_warning() { 30.0 } else { 0.0 };
        let length_risk = (self.length_m / 100.0).min(20.0);
        (wind + ice + length_risk).clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wind_sensitivity() {
        assert!(BridgeType::Suspension.wind_sensitivity() > BridgeType::Beam.wind_sensitivity());
    }

    #[test]
    fn test_vehicle_clearance() {
        let b = Bridge::new("Test", BridgeType::Beam, 100.0);
        assert!(b.vehicle_can_cross(3.0, 20.0));
        assert!(!b.vehicle_can_cross(5.0, 20.0));
    }

    #[test]
    fn test_weight_limit() {
        let b = Bridge::new("Test", BridgeType::Beam, 100.0);
        assert!(!b.vehicle_can_cross(3.0, 50.0));
    }

    #[test]
    fn test_wind_advisory() {
        let mut b = Bridge::new("Susp", BridgeType::Suspension, 500.0);
        b.wind_speed_kmh = 50.0;
        assert!(b.wind_advisory());
    }

    #[test]
    fn test_no_wind_advisory() {
        let b = Bridge::new("Beam", BridgeType::Beam, 100.0);
        assert!(!b.wind_advisory());
    }

    #[test]
    fn test_ice_warning() {
        let mut b = Bridge::new("Test", BridgeType::Suspension, 200.0);
        b.temperature_c = 1.0;
        assert!(b.ice_warning());
    }

    #[test]
    fn test_no_ice_warm() {
        let b = Bridge::new("Test", BridgeType::Suspension, 200.0);
        assert!(!b.ice_warning());
    }

    #[test]
    fn test_safe_speed_normal() {
        let b = Bridge::new("Test", BridgeType::Beam, 100.0);
        assert!((b.safe_speed_kmh() - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_safe_speed_wind() {
        let mut b = Bridge::new("Susp", BridgeType::Suspension, 500.0);
        b.wind_speed_kmh = 60.0;
        assert!(b.safe_speed_kmh() < 60.0);
    }

    #[test]
    fn test_crossing_time() {
        let b = Bridge::new("Test", BridgeType::Beam, 200.0);
        assert!(b.crossing_time_sec() > 5.0 && b.crossing_time_sec() < 30.0);
    }

    #[test]
    fn test_risk_score_range() {
        let b = Bridge::new("Test", BridgeType::Beam, 100.0);
        assert!((0.0..=100.0).contains(&b.risk_score()));
    }

    #[test]
    fn test_pontoon_low_ice() {
        assert!(BridgeType::Pontoon.ice_risk() < 0.5);
    }
}
