/// Cross traffic alert: rear cross traffic, intersection warning
/// Phase 266

#[derive(Debug, Clone)]
pub struct CrossTrafficAlert {
    pub active: bool,
    pub left_approaching: bool,
    pub right_approaching: bool,
    pub distance_m: f64,
    pub speed_kmh: f64,
    pub sensor_ok: bool,
}

impl Default for CrossTrafficAlert {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossTrafficAlert {
    pub fn new() -> Self {
        Self {
            active: true,
            left_approaching: false,
            right_approaching: false,
            distance_m: 50.0,
            speed_kmh: 0.0,
            sensor_ok: true,
        }
    }

    pub fn threat_detected(&self) -> bool {
        self.left_approaching || self.right_approaching
    }

    pub fn imminent_collision(&self) -> bool {
        self.threat_detected() && self.distance_m < 5.0
    }

    pub fn all_clear(&self) -> bool {
        !self.left_approaching && !self.right_approaching
    }

    pub fn should_brake(&self) -> bool {
        self.imminent_collision() && self.speed_kmh < 20.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_threat() {
        let c = CrossTrafficAlert::new();
        assert!(!c.threat_detected());
    }

    #[test]
    fn test_no_collision() {
        let c = CrossTrafficAlert::new();
        assert!(!c.imminent_collision());
    }

    #[test]
    fn test_clear() {
        let c = CrossTrafficAlert::new();
        assert!(c.all_clear());
    }

    #[test]
    fn test_no_brake() {
        let c = CrossTrafficAlert::new();
        assert!(!c.should_brake());
    }

    #[test]
    fn test_approaching() {
        let mut c = CrossTrafficAlert::new();
        c.left_approaching = true;
        assert!(c.threat_detected());
    }

    #[test]
    fn test_health() {
        let c = CrossTrafficAlert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
