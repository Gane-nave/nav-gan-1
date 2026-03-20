/// Blower motor: fan speed control, resistor pack, noise monitoring
/// Phase 238

#[derive(Debug, Clone)]
pub struct BlowerMotor {
    pub speed_level: u8,
    pub max_speed: u8,
    pub current_draw_a: f64,
    pub noise_db: f64,
    pub resistor_ok: bool,
    pub motor_ok: bool,
}

impl Default for BlowerMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl BlowerMotor {
    pub fn new() -> Self {
        Self {
            speed_level: 3,
            max_speed: 5,
            current_draw_a: 8.0,
            noise_db: 35.0,
            resistor_ok: true,
            motor_ok: true,
        }
    }

    pub fn is_running(&self) -> bool {
        self.speed_level > 0 && self.motor_ok
    }

    pub fn speed_pct(&self) -> f64 {
        if self.max_speed == 0 {
            return 0.0;
        }
        self.speed_level as f64 / self.max_speed as f64 * 100.0
    }

    pub fn noise_ok(&self) -> bool {
        self.noise_db < 50.0
    }

    pub fn current_ok(&self) -> bool {
        self.current_draw_a < 20.0
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.resistor_ok || !self.noise_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        let mut score: f64 = 100.0;
        if !self.resistor_ok {
            score -= 30.0;
        }
        if !self.noise_ok() {
            score -= 20.0;
        }
        if !self.current_ok() {
            score -= 15.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running() {
        let b = BlowerMotor::new();
        assert!(b.is_running());
    }

    #[test]
    fn test_speed_pct() {
        let b = BlowerMotor::new();
        assert!((b.speed_pct() - 60.0).abs() < 0.1);
    }

    #[test]
    fn test_noise_ok() {
        let b = BlowerMotor::new();
        assert!(b.noise_ok());
    }

    #[test]
    fn test_current_ok() {
        let b = BlowerMotor::new();
        assert!(b.current_ok());
    }

    #[test]
    fn test_no_service() {
        let b = BlowerMotor::new();
        assert!(!b.needs_service());
    }

    #[test]
    fn test_health() {
        let b = BlowerMotor::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
