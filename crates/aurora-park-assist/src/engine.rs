/// Parking assist: ultrasonic sensors, distance display, guidance
/// Phase 267

#[derive(Debug, Clone)]
pub struct ParkAssist {
    pub front_distance_cm: f64,
    pub rear_distance_cm: f64,
    pub left_distance_cm: f64,
    pub right_distance_cm: f64,
    pub sensors_ok: bool,
    pub audio_alert: bool,
}

impl Default for ParkAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkAssist {
    pub fn new() -> Self {
        Self {
            front_distance_cm: 200.0,
            rear_distance_cm: 200.0,
            left_distance_cm: 100.0,
            right_distance_cm: 100.0,
            sensors_ok: true,
            audio_alert: true,
        }
    }

    pub fn min_distance_cm(&self) -> f64 {
        self.front_distance_cm
            .min(self.rear_distance_cm)
            .min(self.left_distance_cm)
            .min(self.right_distance_cm)
    }

    pub fn too_close(&self) -> bool {
        self.min_distance_cm() < 30.0
    }

    pub fn danger_zone(&self) -> bool {
        self.min_distance_cm() < 15.0
    }

    pub fn all_clear(&self) -> bool {
        self.min_distance_cm() > 100.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensors_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        let p = ParkAssist::new();
        assert!((p.min_distance_cm() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_not_close() {
        let p = ParkAssist::new();
        assert!(!p.too_close());
    }

    #[test]
    fn test_no_danger() {
        let p = ParkAssist::new();
        assert!(!p.danger_zone());
    }

    #[test]
    fn test_all_clear() {
        let p = ParkAssist::new();
        assert!(p.all_clear());
    }

    #[test]
    fn test_close() {
        let mut p = ParkAssist::new();
        p.rear_distance_cm = 10.0;
        assert!(p.danger_zone());
    }

    #[test]
    fn test_health() {
        let p = ParkAssist::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
