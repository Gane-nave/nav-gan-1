/// Torque split: front/rear distribution, AWD bias, traction control
/// Phase 458

#[derive(Debug, Clone)]
pub struct TorqueSplit {
    pub front_pct: f64,
    pub rear_pct: f64,
    pub clutch_ok: bool,
    pub sensor_ok: bool,
    pub mode: u8,
}

impl Default for TorqueSplit {
    fn default() -> Self {
        Self::new()
    }
}

impl TorqueSplit {
    pub fn new() -> Self {
        Self {
            front_pct: 40.0,
            rear_pct: 60.0,
            clutch_ok: true,
            sensor_ok: true,
            mode: 1,
        }
    }

    pub fn balanced(&self) -> bool {
        (self.front_pct + self.rear_pct - 100.0).abs() < 1.0
    }

    pub fn all_ok(&self) -> bool {
        self.balanced() && self.clutch_ok && self.sensor_ok
    }

    pub fn rear_biased(&self) -> bool {
        self.rear_pct > 55.0
    }

    pub fn needs_service(&self) -> bool {
        !self.clutch_ok || !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.clutch_ok {
            return 20.0;
        }
        if !self.sensor_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        let t = TorqueSplit::new();
        assert!(t.balanced());
    }

    #[test]
    fn test_all_ok() {
        let t = TorqueSplit::new();
        assert!(t.all_ok());
    }

    #[test]
    fn test_rear_biased() {
        let t = TorqueSplit::new();
        assert!(t.rear_biased());
    }

    #[test]
    fn test_no_service() {
        let t = TorqueSplit::new();
        assert!(!t.needs_service());
    }

    #[test]
    fn test_bad_clutch() {
        let mut t = TorqueSplit::new();
        t.clutch_ok = false;
        assert!(t.needs_service());
    }

    #[test]
    fn test_health() {
        let t = TorqueSplit::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
