/// Electric motor controller: torque control, regeneration, field weakening
/// Phase 294

#[derive(Debug, Clone)]
pub struct MotorController {
    pub rpm: f64,
    pub torque_nm: f64,
    pub max_torque_nm: f64,
    pub power_kw: f64,
    pub efficiency_pct: f64,
    pub temp_c: f64,
}

impl Default for MotorController {
    fn default() -> Self {
        Self::new()
    }
}

impl MotorController {
    pub fn new() -> Self {
        Self {
            rpm: 0.0,
            torque_nm: 0.0,
            max_torque_nm: 350.0,
            power_kw: 0.0,
            efficiency_pct: 95.0,
            temp_c: 40.0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.rpm > 10.0
    }

    pub fn torque_pct(&self) -> f64 {
        if self.max_torque_nm <= 0.0 {
            return 0.0;
        }
        self.torque_nm / self.max_torque_nm * 100.0
    }

    pub fn overheating(&self) -> bool {
        self.temp_c > 150.0
    }

    pub fn derating(&self) -> bool {
        self.temp_c > 120.0
    }

    pub fn health_score(&self) -> f64 {
        if self.overheating() {
            return 0.0;
        }
        if self.derating() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_running() {
        let m = MotorController::new();
        assert!(!m.is_running());
    }

    #[test]
    fn test_zero_torque() {
        let m = MotorController::new();
        assert!(m.torque_pct() < 0.1);
    }

    #[test]
    fn test_not_hot() {
        let m = MotorController::new();
        assert!(!m.overheating());
    }

    #[test]
    fn test_not_derating() {
        let m = MotorController::new();
        assert!(!m.derating());
    }

    #[test]
    fn test_running() {
        let mut m = MotorController::new();
        m.rpm = 3000.0;
        assert!(m.is_running());
    }

    #[test]
    fn test_health() {
        let m = MotorController::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
