/// Engine oil: viscosity, level, contamination, life
/// Phase 564

#[derive(Debug, Clone)]
pub struct EngineOil {
    pub viscosity_cst: f64,
    pub level_pct: f64,
    pub contamination_ppm: f64,
    pub life_pct: f64,
    pub temp_c: f64,
}

impl Default for EngineOil {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineOil {
    pub fn new() -> Self {
        Self {
            viscosity_cst: 10.0,
            level_pct: 90.0,
            contamination_ppm: 50.0,
            life_pct: 75.0,
            temp_c: 95.0,
        }
    }

    pub fn viscosity_ok(&self) -> bool {
        self.viscosity_cst > 5.0 && self.viscosity_cst < 20.0
    }

    pub fn level_ok(&self) -> bool {
        self.level_pct > 30.0
    }

    pub fn all_ok(&self) -> bool {
        self.viscosity_ok() && self.level_ok() && self.life_pct > 10.0
    }

    pub fn needs_change(&self) -> bool {
        self.life_pct < 10.0 || self.contamination_ppm > 200.0
    }

    pub fn health_score(&self) -> f64 {
        if self.life_pct < 10.0 { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viscosity() {
        let c = EngineOil::new();
        assert!(c.viscosity_ok());
    }

    #[test]
    fn test_level() {
        let c = EngineOil::new();
        assert!(c.level_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EngineOil::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_change() {
        let c = EngineOil::new();
        assert!(!c.needs_change());
    }

    #[test]
    fn test_low_life() {
        let mut c = EngineOil::new();
        c.life_pct = 5.0;
        assert!(c.needs_change());
    }

    #[test]
    fn test_health() {
        let c = EngineOil::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
