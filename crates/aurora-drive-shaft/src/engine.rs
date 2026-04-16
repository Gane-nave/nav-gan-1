/// Drive shaft monitoring: vibration, balance, U-joint wear
/// Phase 196

#[derive(Debug, Clone)]
pub struct DriveShaft {
    pub vibration_g: f64,
    pub speed_rpm: f64,
    pub balance_offset_mm: f64,
    pub u_joint_wear_pct: f64,
    pub length_mm: f64,
}

impl Default for DriveShaft {
    fn default() -> Self {
        Self::new()
    }
}

impl DriveShaft {
    pub fn new() -> Self {
        Self {
            vibration_g: 0.05,
            speed_rpm: 2000.0,
            balance_offset_mm: 0.1,
            u_joint_wear_pct: 10.0,
            length_mm: 1200.0,
        }
    }

    pub fn vibration_ok(&self) -> bool {
        self.vibration_g < 0.5
    }

    pub fn balance_ok(&self) -> bool {
        self.balance_offset_mm < 0.5
    }

    pub fn u_joint_needs_service(&self) -> bool {
        self.u_joint_wear_pct > 70.0
    }

    pub fn needs_attention(&self) -> bool {
        !self.vibration_ok() || !self.balance_ok() || self.u_joint_needs_service()
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.vibration_ok() {
            score -= 30.0;
        }
        if !self.balance_ok() {
            score -= 20.0;
        }
        if self.u_joint_needs_service() {
            score -= 30.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy() {
        let d = DriveShaft::new();
        assert!(!d.needs_attention());
    }

    #[test]
    fn test_vibration_ok() {
        let d = DriveShaft::new();
        assert!(d.vibration_ok());
    }

    #[test]
    fn test_balance_ok() {
        let d = DriveShaft::new();
        assert!(d.balance_ok());
    }

    #[test]
    fn test_u_joint_ok() {
        let d = DriveShaft::new();
        assert!(!d.u_joint_needs_service());
    }

    #[test]
    fn test_worn_u_joint() {
        let mut d = DriveShaft::new();
        d.u_joint_wear_pct = 80.0;
        assert!(d.u_joint_needs_service());
    }

    #[test]
    fn test_health() {
        let d = DriveShaft::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
