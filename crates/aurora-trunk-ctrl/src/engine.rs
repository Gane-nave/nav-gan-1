/// Trunk control: power liftgate, latch sensor, kick sensor
/// Phase 255

#[derive(Debug, Clone)]
pub struct TrunkController {
    pub is_open: bool,
    pub latch_ok: bool,
    pub power_liftgate: bool,
    pub kick_sensor_ok: bool,
    pub motor_ok: bool,
    pub position_pct: f64,
}

impl Default for TrunkController {
    fn default() -> Self {
        Self::new()
    }
}

impl TrunkController {
    pub fn new() -> Self {
        Self {
            is_open: false,
            latch_ok: true,
            power_liftgate: true,
            kick_sensor_ok: true,
            motor_ok: true,
            position_pct: 0.0,
        }
    }

    pub fn fully_open(&self) -> bool {
        self.position_pct >= 99.0
    }

    pub fn fully_closed(&self) -> bool {
        self.position_pct <= 1.0 && self.latch_ok
    }

    pub fn can_open(&self) -> bool {
        self.motor_ok && self.power_liftgate
    }

    pub fn ajar(&self) -> bool {
        self.position_pct > 5.0 && self.position_pct < 95.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.motor_ok {
            score -= 40.0;
        }
        if !self.latch_ok {
            score -= 30.0;
        }
        if !self.kick_sensor_ok {
            score -= 10.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_open() {
        let t = TrunkController::new();
        assert!(!t.fully_open());
    }

    #[test]
    fn test_closed() {
        let t = TrunkController::new();
        assert!(t.fully_closed());
    }

    #[test]
    fn test_can_open() {
        let t = TrunkController::new();
        assert!(t.can_open());
    }

    #[test]
    fn test_not_ajar() {
        let t = TrunkController::new();
        assert!(!t.ajar());
    }

    #[test]
    fn test_ajar() {
        let mut t = TrunkController::new();
        t.position_pct = 50.0;
        assert!(t.ajar());
    }

    #[test]
    fn test_health() {
        let t = TrunkController::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
