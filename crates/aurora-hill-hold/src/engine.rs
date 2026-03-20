/// Hill hold assist: incline detection, brake hold, rollback prevention
/// Phase 207

#[derive(Debug, Clone)]
pub struct HillHoldAssist {
    pub incline_deg: f64,
    pub brake_hold_active: bool,
    pub hold_pressure_bar: f64,
    pub rollback_detected: bool,
    pub enabled: bool,
}

impl Default for HillHoldAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl HillHoldAssist {
    pub fn new() -> Self {
        Self {
            incline_deg: 0.0,
            brake_hold_active: false,
            hold_pressure_bar: 0.0,
            rollback_detected: false,
            enabled: true,
        }
    }

    pub fn on_hill(&self) -> bool {
        self.incline_deg.abs() > 3.0
    }

    pub fn should_activate(&self) -> bool {
        self.enabled && self.on_hill() && !self.brake_hold_active
    }

    pub fn holding(&self) -> bool {
        self.brake_hold_active && self.hold_pressure_bar > 5.0
    }

    pub fn steep_hill(&self) -> bool {
        self.incline_deg.abs() > 15.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.enabled {
            return 50.0;
        }
        if self.rollback_detected {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat() {
        let h = HillHoldAssist::new();
        assert!(!h.on_hill());
    }

    #[test]
    fn test_on_hill() {
        let mut h = HillHoldAssist::new();
        h.incline_deg = 10.0;
        assert!(h.on_hill());
    }

    #[test]
    fn test_should_activate() {
        let mut h = HillHoldAssist::new();
        h.incline_deg = 10.0;
        assert!(h.should_activate());
    }

    #[test]
    fn test_not_holding() {
        let h = HillHoldAssist::new();
        assert!(!h.holding());
    }

    #[test]
    fn test_steep() {
        let mut h = HillHoldAssist::new();
        h.incline_deg = 20.0;
        assert!(h.steep_hill());
    }

    #[test]
    fn test_health() {
        let h = HillHoldAssist::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
