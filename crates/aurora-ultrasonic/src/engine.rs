/// Ultrasonic sensors: parking assist, close-range detection, curb detection
/// Phase 184

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorZone {
    FrontCenter,
    FrontLeft,
    FrontRight,
    RearCenter,
    RearLeft,
    RearRight,
    SideLeft,
    SideRight,
}

#[derive(Debug, Clone)]
pub struct UltrasonicSensor {
    pub zone: SensorZone,
    pub distance_cm: f64,
    pub active: bool,
}

impl UltrasonicSensor {
    pub fn new(zone: SensorZone) -> Self {
        Self {
            zone,
            distance_cm: 300.0,
            active: true,
        }
    }

    pub fn obstacle_near(&self) -> bool {
        self.active && self.distance_cm < 50.0
    }

    pub fn obstacle_critical(&self) -> bool {
        self.active && self.distance_cm < 15.0
    }
}

#[derive(Debug, Clone)]
pub struct UltrasonicSystem {
    pub sensors: Vec<UltrasonicSensor>,
}

impl Default for UltrasonicSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl UltrasonicSystem {
    pub fn new() -> Self {
        Self {
            sensors: vec![
                UltrasonicSensor::new(SensorZone::FrontCenter),
                UltrasonicSensor::new(SensorZone::FrontLeft),
                UltrasonicSensor::new(SensorZone::FrontRight),
                UltrasonicSensor::new(SensorZone::RearCenter),
                UltrasonicSensor::new(SensorZone::RearLeft),
                UltrasonicSensor::new(SensorZone::RearRight),
            ],
        }
    }

    pub fn any_near(&self) -> bool {
        self.sensors.iter().any(|s| s.obstacle_near())
    }

    pub fn any_critical(&self) -> bool {
        self.sensors.iter().any(|s| s.obstacle_critical())
    }

    pub fn closest_cm(&self) -> f64 {
        self.sensors
            .iter()
            .filter(|s| s.active)
            .map(|s| s.distance_cm)
            .fold(f64::MAX, f64::min)
    }

    pub fn all_clear(&self) -> bool {
        self.sensors
            .iter()
            .filter(|s| s.active)
            .all(|s| s.distance_cm > 100.0)
    }

    pub fn active_count(&self) -> usize {
        self.sensors.iter().filter(|s| s.active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_obstacle() {
        let s = UltrasonicSensor::new(SensorZone::FrontCenter);
        assert!(!s.obstacle_near());
    }

    #[test]
    fn test_near() {
        let mut s = UltrasonicSensor::new(SensorZone::FrontCenter);
        s.distance_cm = 30.0;
        assert!(s.obstacle_near());
    }

    #[test]
    fn test_critical() {
        let mut s = UltrasonicSensor::new(SensorZone::RearCenter);
        s.distance_cm = 10.0;
        assert!(s.obstacle_critical());
    }

    #[test]
    fn test_system_clear() {
        let s = UltrasonicSystem::new();
        assert!(s.all_clear());
    }

    #[test]
    fn test_system_not_near() {
        let s = UltrasonicSystem::new();
        assert!(!s.any_near());
    }

    #[test]
    fn test_closest() {
        let mut s = UltrasonicSystem::new();
        s.sensors[0].distance_cm = 20.0;
        assert!((s.closest_cm() - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_active_count() {
        let s = UltrasonicSystem::new();
        assert_eq!(s.active_count(), 6);
    }
}
