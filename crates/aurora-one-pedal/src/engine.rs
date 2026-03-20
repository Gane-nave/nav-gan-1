/// One-pedal driving: regen level, hold, creep, transition
/// Phase 874

#[derive(Debug, Clone)]
pub struct OnePedal {
    pub regen_ok: bool,
    pub hold_ok: bool,
    pub creep_ok: bool,
    pub transition_ok: bool,
    pub smooth_ok: bool,
}

impl Default for OnePedal {
    fn default() -> Self {
        Self::new()
    }
}

impl OnePedal {
    pub fn new() -> Self {
        Self {
            regen_ok: true,
            hold_ok: true,
            creep_ok: true,
            transition_ok: true,
            smooth_ok: true,
        }
    }

    pub fn braking_ok(&self) -> bool {
        self.regen_ok && self.hold_ok && self.smooth_ok
    }

    pub fn features_ok(&self) -> bool {
        self.creep_ok && self.transition_ok
    }

    pub fn all_ok(&self) -> bool {
        self.braking_ok() && self.features_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.regen_ok || !self.smooth_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.regen_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braking() {
        let c = OnePedal::new();
        assert!(c.braking_ok());
    }

    #[test]
    fn test_features() {
        let c = OnePedal::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OnePedal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = OnePedal::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_regen() {
        let mut c = OnePedal::new();
        c.regen_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = OnePedal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
