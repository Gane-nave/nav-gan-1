/// Pedestrian detection: camera-based detection, auto emergency braking
/// Phase 229

#[derive(Debug, Clone)]
pub struct PedestrianDetector {
    pub pedestrians_detected: u32,
    pub closest_distance_m: f64,
    pub collision_risk_pct: f64,
    pub aeb_armed: bool,
    pub aeb_activated: bool,
    pub camera_ok: bool,
}

impl Default for PedestrianDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl PedestrianDetector {
    pub fn new() -> Self {
        Self {
            pedestrians_detected: 0,
            closest_distance_m: 100.0,
            collision_risk_pct: 0.0,
            aeb_armed: true,
            aeb_activated: false,
            camera_ok: true,
        }
    }

    pub fn any_detected(&self) -> bool {
        self.pedestrians_detected > 0
    }

    pub fn danger_zone(&self) -> bool {
        self.closest_distance_m < 5.0
    }

    pub fn should_brake(&self) -> bool {
        self.aeb_armed && self.collision_risk_pct > 80.0
    }

    pub fn should_warn(&self) -> bool {
        self.collision_risk_pct > 50.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 20.0;
        }
        if !self.aeb_armed {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_detected() {
        let p = PedestrianDetector::new();
        assert!(!p.any_detected());
    }

    #[test]
    fn test_no_danger() {
        let p = PedestrianDetector::new();
        assert!(!p.danger_zone());
    }

    #[test]
    fn test_no_brake() {
        let p = PedestrianDetector::new();
        assert!(!p.should_brake());
    }

    #[test]
    fn test_no_warn() {
        let p = PedestrianDetector::new();
        assert!(!p.should_warn());
    }

    #[test]
    fn test_detected() {
        let mut p = PedestrianDetector::new();
        p.pedestrians_detected = 2;
        p.closest_distance_m = 3.0;
        assert!(p.any_detected());
        assert!(p.danger_zone());
    }

    #[test]
    fn test_health() {
        let p = PedestrianDetector::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
