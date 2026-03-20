/// Steering wheel: angle sensor, torque sensor, buttons, paddle shifters
/// Phase 258

#[derive(Debug, Clone)]
pub struct SteeringWheel {
    pub angle_deg: f64,
    pub torque_nm: f64,
    pub buttons_ok: bool,
    pub heated: bool,
    pub paddle_left: bool,
    pub paddle_right: bool,
}

impl Default for SteeringWheel {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringWheel {
    pub fn new() -> Self {
        Self {
            angle_deg: 0.0,
            torque_nm: 0.0,
            buttons_ok: true,
            heated: false,
            paddle_left: false,
            paddle_right: false,
        }
    }

    pub fn centered(&self) -> bool {
        self.angle_deg.abs() < 5.0
    }

    pub fn full_lock(&self) -> bool {
        self.angle_deg.abs() > 450.0
    }

    pub fn turns(&self) -> f64 {
        self.angle_deg.abs() / 360.0
    }

    pub fn driver_input(&self) -> bool {
        self.torque_nm.abs() > 0.5
    }

    pub fn health_score(&self) -> f64 {
        if !self.buttons_ok {
            return 70.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered() {
        let s = SteeringWheel::new();
        assert!(s.centered());
    }

    #[test]
    fn test_not_full_lock() {
        let s = SteeringWheel::new();
        assert!(!s.full_lock());
    }

    #[test]
    fn test_zero_turns() {
        let s = SteeringWheel::new();
        assert!(s.turns() < 0.1);
    }

    #[test]
    fn test_no_input() {
        let s = SteeringWheel::new();
        assert!(!s.driver_input());
    }

    #[test]
    fn test_turned() {
        let mut s = SteeringWheel::new();
        s.angle_deg = 90.0;
        assert!(!s.centered());
    }

    #[test]
    fn test_health() {
        let s = SteeringWheel::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
