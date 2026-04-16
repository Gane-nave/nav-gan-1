/// Idle speed control: target RPM, idle air valve, cold start enrichment
/// Phase 210

#[derive(Debug, Clone)]
pub struct IdleController {
    pub target_rpm: f64,
    pub current_rpm: f64,
    pub air_valve_pct: f64,
    pub cold_start: bool,
    pub ac_on: bool,
    pub load_compensation: f64,
}

impl Default for IdleController {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleController {
    pub fn new() -> Self {
        Self {
            target_rpm: 750.0,
            current_rpm: 750.0,
            air_valve_pct: 30.0,
            cold_start: false,
            ac_on: false,
            load_compensation: 0.0,
        }
    }

    pub fn rpm_error(&self) -> f64 {
        (self.current_rpm - self.target_rpm).abs()
    }

    pub fn stable(&self) -> bool {
        self.rpm_error() < 50.0
    }

    pub fn stalling_risk(&self) -> bool {
        self.current_rpm < 500.0
    }

    pub fn effective_target(&self) -> f64 {
        let mut target = self.target_rpm;
        if self.cold_start {
            target += 200.0;
        }
        if self.ac_on {
            target += 100.0;
        }
        target + self.load_compensation
    }

    pub fn health_score(&self) -> f64 {
        if self.stalling_risk() {
            return 20.0;
        }
        if !self.stable() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        let i = IdleController::new();
        assert!(i.stable());
    }

    #[test]
    fn test_no_stall_risk() {
        let i = IdleController::new();
        assert!(!i.stalling_risk());
    }

    #[test]
    fn test_rpm_error() {
        let i = IdleController::new();
        assert!(i.rpm_error() < 1.0);
    }

    #[test]
    fn test_cold_start_target() {
        let mut i = IdleController::new();
        i.cold_start = true;
        assert!((i.effective_target() - 950.0).abs() < 0.1);
    }

    #[test]
    fn test_ac_target() {
        let mut i = IdleController::new();
        i.ac_on = true;
        assert!((i.effective_target() - 850.0).abs() < 0.1);
    }

    #[test]
    fn test_health() {
        let i = IdleController::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
