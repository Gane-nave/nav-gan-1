/// Driveshaft balance: rotational balance, U-joint, center bearing
/// Phase 462

#[derive(Debug, Clone)]
pub struct DriveshaftBal {
    pub vibration_ok: bool,
    pub u_joint_ok: bool,
    pub center_bearing_ok: bool,
    pub balance_g: f64,
    pub max_balance_g: f64,
}

impl Default for DriveshaftBal {
    fn default() -> Self {
        Self::new()
    }
}

impl DriveshaftBal {
    pub fn new() -> Self {
        Self {
            vibration_ok: true,
            u_joint_ok: true,
            center_bearing_ok: true,
            balance_g: 2.0,
            max_balance_g: 10.0,
        }
    }

    pub fn balanced(&self) -> bool {
        self.balance_g < self.max_balance_g
    }

    pub fn all_ok(&self) -> bool {
        self.balanced() && self.u_joint_ok && self.center_bearing_ok && self.vibration_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.u_joint_ok || !self.center_bearing_ok
    }

    pub fn smooth(&self) -> bool {
        self.vibration_ok && self.balanced()
    }

    pub fn health_score(&self) -> f64 {
        if !self.u_joint_ok {
            return 10.0;
        }
        if !self.center_bearing_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        let d = DriveshaftBal::new();
        assert!(d.balanced());
    }

    #[test]
    fn test_all_ok() {
        let d = DriveshaftBal::new();
        assert!(d.all_ok());
    }

    #[test]
    fn test_no_service() {
        let d = DriveshaftBal::new();
        assert!(!d.needs_service());
    }

    #[test]
    fn test_smooth() {
        let d = DriveshaftBal::new();
        assert!(d.smooth());
    }

    #[test]
    fn test_bad_ujoint() {
        let mut d = DriveshaftBal::new();
        d.u_joint_ok = false;
        assert!(d.needs_service());
    }

    #[test]
    fn test_health() {
        let d = DriveshaftBal::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
