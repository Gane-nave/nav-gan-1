/// Clutch controller: engagement point, wear, hydraulic pressure
/// Phase 325

#[derive(Debug, Clone)]
pub struct ClutchController {
    pub engagement_pct: f64,
    pub wear_pct: f64,
    pub hydraulic_ok: bool,
    pub slip_detected: bool,
    pub pedal_position_pct: f64,
}

impl Default for ClutchController {
    fn default() -> Self {
        Self::new()
    }
}

impl ClutchController {
    pub fn new() -> Self {
        Self {
            engagement_pct: 100.0,
            wear_pct: 20.0,
            hydraulic_ok: true,
            slip_detected: false,
            pedal_position_pct: 0.0,
        }
    }

    pub fn fully_engaged(&self) -> bool {
        self.engagement_pct > 95.0
    }

    pub fn slipping(&self) -> bool {
        self.slip_detected
    }

    pub fn needs_replacement(&self) -> bool {
        self.wear_pct > 80.0 || self.slip_detected
    }

    pub fn remaining_life_pct(&self) -> f64 {
        (100.0 - self.wear_pct).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.slip_detected {
            return 10.0;
        }
        if !self.hydraulic_ok {
            return 30.0;
        }
        if self.wear_pct > 70.0 {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engaged() {
        let c = ClutchController::new();
        assert!(c.fully_engaged());
    }

    #[test]
    fn test_no_slip() {
        let c = ClutchController::new();
        assert!(!c.slipping());
    }

    #[test]
    fn test_no_replace() {
        let c = ClutchController::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_life() {
        let c = ClutchController::new();
        assert!(c.remaining_life_pct() > 70.0);
    }

    #[test]
    fn test_slip() {
        let mut c = ClutchController::new();
        c.slip_detected = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ClutchController::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
