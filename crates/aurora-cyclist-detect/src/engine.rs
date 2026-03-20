/// Cyclist detection and safety: buffer zones, overtaking rules, infrastructure.
#[derive(Debug, Clone, PartialEq)]
pub enum CyclistType {
    Commuter,
    Sport,
    Cargo,
    Electric,
    Child,
    Tandem,
    Scooter,
}

impl CyclistType {
    pub fn typical_speed_kmh(&self) -> f64 {
        match self {
            CyclistType::Commuter => 20.0,
            CyclistType::Sport => 30.0,
            CyclistType::Cargo => 15.0,
            CyclistType::Electric => 25.0,
            CyclistType::Child => 10.0,
            CyclistType::Tandem => 22.0,
            CyclistType::Scooter => 18.0,
        }
    }

    pub fn required_buffer_m(&self) -> f64 {
        match self {
            CyclistType::Child => 2.0,
            CyclistType::Cargo => 1.8,
            CyclistType::Tandem => 1.8,
            _ => 1.5,
        }
    }

    pub fn vulnerability(&self) -> f64 {
        match self {
            CyclistType::Child => 1.0,
            CyclistType::Cargo => 0.7,
            _ => 0.8,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BikeInfra {
    DedicatedLane,
    SharedLane,
    BikePath,
    Sidewalk,
    NoInfra,
}

impl BikeInfra {
    pub fn safety_factor(&self) -> f64 {
        match self {
            BikeInfra::BikePath => 0.95,
            BikeInfra::DedicatedLane => 0.85,
            BikeInfra::SharedLane => 0.5,
            BikeInfra::Sidewalk => 0.4,
            BikeInfra::NoInfra => 0.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DetectedCyclist {
    pub cyclist_type: CyclistType,
    pub distance_m: f64,
    pub speed_kmh: f64,
    pub infrastructure: BikeInfra,
    pub same_direction: bool,
}

impl DetectedCyclist {
    pub fn new(cyclist_type: CyclistType, distance_m: f64) -> Self {
        let speed = cyclist_type.typical_speed_kmh();
        Self {
            cyclist_type,
            distance_m,
            speed_kmh: speed,
            infrastructure: BikeInfra::NoInfra,
            same_direction: true,
        }
    }

    pub fn safe_overtake_speed(&self, road_speed_limit: f64) -> f64 {
        let buffer_factor = if self.cyclist_type.required_buffer_m() >= 2.0 {
            0.6
        } else {
            0.7
        };
        road_speed_limit * buffer_factor
    }

    pub fn closing_speed_kmh(&self, vehicle_speed: f64) -> f64 {
        if self.same_direction {
            (vehicle_speed - self.speed_kmh).max(0.0)
        } else {
            vehicle_speed + self.speed_kmh
        }
    }

    pub fn time_to_reach_sec(&self, vehicle_speed: f64) -> f64 {
        let closing = self.closing_speed_kmh(vehicle_speed) / 3.6;
        if closing <= 0.0 {
            return f64::INFINITY;
        }
        self.distance_m / closing
    }

    pub fn risk_score(&self) -> f64 {
        let dist_risk = (30.0 - self.distance_m).max(0.0) * 3.0;
        let vuln = self.cyclist_type.vulnerability() * 15.0;
        let infra = (1.0 - self.infrastructure.safety_factor()) * 20.0;
        (dist_risk + vuln + infra).clamp(0.0, 100.0)
    }

    pub fn can_safely_pass(&self, lateral_clearance_m: f64) -> bool {
        lateral_clearance_m >= self.cyclist_type.required_buffer_m()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyclist_speed() {
        assert!(CyclistType::Sport.typical_speed_kmh() > CyclistType::Child.typical_speed_kmh());
    }

    #[test]
    fn test_buffer_child() {
        assert!(CyclistType::Child.required_buffer_m() > CyclistType::Commuter.required_buffer_m());
    }

    #[test]
    fn test_infra_safety() {
        assert!(BikeInfra::BikePath.safety_factor() > BikeInfra::NoInfra.safety_factor());
    }

    #[test]
    fn test_overtake_speed() {
        let c = DetectedCyclist::new(CyclistType::Commuter, 20.0);
        assert!(c.safe_overtake_speed(50.0) < 50.0);
    }

    #[test]
    fn test_closing_speed_same() {
        let c = DetectedCyclist::new(CyclistType::Commuter, 20.0);
        assert!((c.closing_speed_kmh(50.0) - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_closing_speed_opposite() {
        let mut c = DetectedCyclist::new(CyclistType::Commuter, 20.0);
        c.same_direction = false;
        assert!((c.closing_speed_kmh(50.0) - 70.0).abs() < 0.01);
    }

    #[test]
    fn test_time_to_reach() {
        let c = DetectedCyclist::new(CyclistType::Commuter, 30.0);
        let t = c.time_to_reach_sec(50.0);
        assert!(t > 0.0 && t < 10.0);
    }

    #[test]
    fn test_risk_close() {
        let close = DetectedCyclist::new(CyclistType::Child, 5.0);
        let far = DetectedCyclist::new(CyclistType::Commuter, 50.0);
        assert!(close.risk_score() > far.risk_score());
    }

    #[test]
    fn test_can_pass() {
        let c = DetectedCyclist::new(CyclistType::Commuter, 20.0);
        assert!(c.can_safely_pass(2.0));
        assert!(!c.can_safely_pass(1.0));
    }

    #[test]
    fn test_risk_with_infra() {
        let mut c1 = DetectedCyclist::new(CyclistType::Commuter, 20.0);
        let r1 = c1.risk_score();
        c1.infrastructure = BikeInfra::BikePath;
        assert!(c1.risk_score() < r1);
    }
}
