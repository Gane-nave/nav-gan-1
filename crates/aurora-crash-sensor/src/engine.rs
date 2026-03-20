/// Crash sensor: impact detection, severity classification, zone identification
/// Phase 227

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImpactZone {
    FrontCenter,
    FrontLeft,
    FrontRight,
    SideLeft,
    SideRight,
    Rear,
    None,
}

#[derive(Debug, Clone)]
pub struct CrashSensor {
    pub impact_g: f64,
    pub zone: ImpactZone,
    pub deceleration_rate_gps: f64,
    pub intrusion_mm: f64,
    pub sensor_count: u8,
    pub all_sensors_ok: bool,
}

impl Default for CrashSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashSensor {
    pub fn new() -> Self {
        Self {
            impact_g: 0.0,
            zone: ImpactZone::None,
            deceleration_rate_gps: 0.0,
            intrusion_mm: 0.0,
            sensor_count: 6,
            all_sensors_ok: true,
        }
    }

    pub fn impact_detected(&self) -> bool {
        self.impact_g > 2.0
    }

    pub fn severe_impact(&self) -> bool {
        self.impact_g > 15.0
    }

    pub fn deploy_airbags(&self) -> bool {
        self.impact_g > 5.0 && self.zone != ImpactZone::None
    }

    pub fn structural_damage(&self) -> bool {
        self.intrusion_mm > 50.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.all_sensors_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_impact() {
        let c = CrashSensor::new();
        assert!(!c.impact_detected());
    }

    #[test]
    fn test_no_severe() {
        let c = CrashSensor::new();
        assert!(!c.severe_impact());
    }

    #[test]
    fn test_no_deploy() {
        let c = CrashSensor::new();
        assert!(!c.deploy_airbags());
    }

    #[test]
    fn test_no_structural() {
        let c = CrashSensor::new();
        assert!(!c.structural_damage());
    }

    #[test]
    fn test_impact() {
        let mut c = CrashSensor::new();
        c.impact_g = 10.0;
        c.zone = ImpactZone::FrontCenter;
        assert!(c.deploy_airbags());
    }

    #[test]
    fn test_health() {
        let c = CrashSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
