/// Downforce: aerodynamic load, grip enhancement, speed-dependent
/// Phase 350

#[derive(Debug, Clone)]
pub struct Downforce {
    pub force_n: f64,
    pub speed_kmh: f64,
    pub wing_angle_deg: f64,
    pub active_aero: bool,
}

impl Default for Downforce {
    fn default() -> Self {
        Self::new()
    }
}

impl Downforce {
    pub fn new() -> Self {
        Self {
            force_n: 200.0,
            speed_kmh: 100.0,
            wing_angle_deg: 5.0,
            active_aero: false,
        }
    }

    pub fn significant(&self) -> bool {
        self.force_n > 100.0
    }

    pub fn high_downforce(&self) -> bool {
        self.force_n > 500.0
    }

    pub fn grip_benefit(&self) -> bool {
        self.force_n > 50.0 && self.speed_kmh > 60.0
    }

    pub fn force_at_speed(&self, speed_kmh: f64) -> f64 {
        if self.speed_kmh <= 0.0 {
            return 0.0;
        }
        let ratio = speed_kmh / self.speed_kmh;
        self.force_n * ratio * ratio
    }

    pub fn health_score(&self) -> f64 {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_significant() {
        let d = Downforce::new();
        assert!(d.significant());
    }

    #[test]
    fn test_not_high() {
        let d = Downforce::new();
        assert!(!d.high_downforce());
    }

    #[test]
    fn test_grip() {
        let d = Downforce::new();
        assert!(d.grip_benefit());
    }

    #[test]
    fn test_force_at_speed() {
        let d = Downforce::new();
        assert!(d.force_at_speed(200.0) > 700.0);
    }

    #[test]
    fn test_no_grip_slow() {
        let mut d = Downforce::new();
        d.speed_kmh = 30.0;
        d.force_n = 10.0;
        assert!(!d.grip_benefit());
    }

    #[test]
    fn test_health() {
        let d = Downforce::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
