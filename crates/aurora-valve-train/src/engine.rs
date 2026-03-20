/// Valve train: valve lift, duration, overlap, hydraulic lifters
/// Phase 312

#[derive(Debug, Clone)]
pub struct ValveTrain {
    pub intake_lift_mm: f64,
    pub exhaust_lift_mm: f64,
    pub overlap_deg: f64,
    pub lifters_ok: bool,
    pub springs_ok: bool,
}

impl Default for ValveTrain {
    fn default() -> Self {
        Self::new()
    }
}

impl ValveTrain {
    pub fn new() -> Self {
        Self {
            intake_lift_mm: 10.0,
            exhaust_lift_mm: 9.5,
            overlap_deg: 20.0,
            lifters_ok: true,
            springs_ok: true,
        }
    }

    pub fn lift_ok(&self) -> bool {
        self.intake_lift_mm > 5.0 && self.exhaust_lift_mm > 5.0
    }

    pub fn all_ok(&self) -> bool {
        self.lifters_ok && self.springs_ok && self.lift_ok()
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.lifters_ok || !self.springs_ok
    }

    pub fn high_performance(&self) -> bool {
        self.intake_lift_mm > 12.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.springs_ok {
            return 0.0;
        }
        if !self.lifters_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lift_ok() {
        let v = ValveTrain::new();
        assert!(v.lift_ok());
    }

    #[test]
    fn test_all_ok() {
        let v = ValveTrain::new();
        assert!(v.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let v = ValveTrain::new();
        assert!(!v.needs_adjustment());
    }

    #[test]
    fn test_not_high_perf() {
        let v = ValveTrain::new();
        assert!(!v.high_performance());
    }

    #[test]
    fn test_bad_lifter() {
        let mut v = ValveTrain::new();
        v.lifters_ok = false;
        assert!(v.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let v = ValveTrain::new();
        assert!((v.health_score() - 100.0).abs() < 0.1);
    }
}
