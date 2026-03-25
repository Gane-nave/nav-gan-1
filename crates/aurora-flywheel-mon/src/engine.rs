/// Flywheel monitor: dual-mass, surface condition, ring gear
/// Phase 457

#[derive(Debug, Clone)]
pub struct FlywheelMon {
    pub surface_ok: bool,
    pub ring_gear_ok: bool,
    pub dual_mass_ok: bool,
    pub runout_mm: f64,
    pub max_runout_mm: f64,
}

impl Default for FlywheelMon {
    fn default() -> Self {
        Self::new()
    }
}

impl FlywheelMon {
    pub fn new() -> Self {
        Self {
            surface_ok: true,
            ring_gear_ok: true,
            dual_mass_ok: true,
            runout_mm: 0.05,
            max_runout_mm: 0.15,
        }
    }

    pub fn runout_ok(&self) -> bool {
        self.runout_mm < self.max_runout_mm
    }

    pub fn all_ok(&self) -> bool {
        self.surface_ok && self.ring_gear_ok && self.dual_mass_ok && self.runout_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.surface_ok || !self.dual_mass_ok
    }

    pub fn starter_ok(&self) -> bool {
        self.ring_gear_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.dual_mass_ok {
            return 10.0;
        }
        if !self.surface_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runout() {
        let f = FlywheelMon::new();
        assert!(f.runout_ok());
    }

    #[test]
    fn test_all_ok() {
        let f = FlywheelMon::new();
        assert!(f.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let f = FlywheelMon::new();
        assert!(!f.needs_replacement());
    }

    #[test]
    fn test_starter() {
        let f = FlywheelMon::new();
        assert!(f.starter_ok());
    }

    #[test]
    fn test_bad_dmf() {
        let mut f = FlywheelMon::new();
        f.dual_mass_ok = false;
        assert!(f.needs_replacement());
    }

    #[test]
    fn test_health() {
        let f = FlywheelMon::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
