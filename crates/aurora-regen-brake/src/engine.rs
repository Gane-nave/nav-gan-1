/// Regenerative braking: energy recovery, blending, control
/// Phase 721

#[derive(Debug, Clone)]
pub struct RegenBrake {
    pub recovery_ok: bool,
    pub blending_ok: bool,
    pub control_ok: bool,
    pub efficiency_ok: bool,
    pub smooth_ok: bool,
}

impl Default for RegenBrake {
    fn default() -> Self {
        Self::new()
    }
}

impl RegenBrake {
    pub fn new() -> Self {
        Self {
            recovery_ok: true,
            blending_ok: true,
            control_ok: true,
            efficiency_ok: true,
            smooth_ok: true,
        }
    }

    pub fn energy_ok(&self) -> bool {
        self.recovery_ok && self.efficiency_ok
    }

    pub fn feel_ok(&self) -> bool {
        self.blending_ok && self.control_ok && self.smooth_ok
    }

    pub fn all_ok(&self) -> bool {
        self.energy_ok() && self.feel_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.blending_ok || !self.control_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.recovery_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy() {
        let c = RegenBrake::new();
        assert!(c.energy_ok());
    }

    #[test]
    fn test_feel() {
        let c = RegenBrake::new();
        assert!(c.feel_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RegenBrake::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = RegenBrake::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_blending() {
        let mut c = RegenBrake::new();
        c.blending_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = RegenBrake::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
