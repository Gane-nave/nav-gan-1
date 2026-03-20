/// Energy regeneration: brake regen, coast regen, level, map
/// Phase 872

#[derive(Debug, Clone)]
pub struct EnergyRegen {
    pub brake_ok: bool,
    pub coast_ok: bool,
    pub level_ok: bool,
    pub map_ok: bool,
    pub efficiency_ok: bool,
}

impl Default for EnergyRegen {
    fn default() -> Self {
        Self::new()
    }
}

impl EnergyRegen {
    pub fn new() -> Self {
        Self {
            brake_ok: true,
            coast_ok: true,
            level_ok: true,
            map_ok: true,
            efficiency_ok: true,
        }
    }

    pub fn recovery_ok(&self) -> bool {
        self.brake_ok && self.coast_ok && self.level_ok
    }

    pub fn optimization_ok(&self) -> bool {
        self.map_ok && self.efficiency_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recovery_ok() && self.optimization_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.level_ok || !self.efficiency_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.brake_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery() {
        let c = EnergyRegen::new();
        assert!(c.recovery_ok());
    }

    #[test]
    fn test_optimization() {
        let c = EnergyRegen::new();
        assert!(c.optimization_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EnergyRegen::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = EnergyRegen::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_level() {
        let mut c = EnergyRegen::new();
        c.level_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = EnergyRegen::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
