/// Washer fluid: level, nozzle, pump, heating
/// Phase 570

#[derive(Debug, Clone)]
pub struct WasherFluid {
    pub level_pct: f64,
    pub nozzle_ok: bool,
    pub pump_ok: bool,
    pub heated: bool,
    pub fluid_ok: bool,
}

impl Default for WasherFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl WasherFluid {
    pub fn new() -> Self {
        Self {
            level_pct: 80.0,
            nozzle_ok: true,
            pump_ok: true,
            heated: true,
            fluid_ok: true,
        }
    }

    pub fn level_ok(&self) -> bool {
        self.level_pct > 10.0
    }

    pub fn system_ok(&self) -> bool {
        self.nozzle_ok && self.pump_ok
    }

    pub fn all_ok(&self) -> bool {
        self.level_ok() && self.system_ok() && self.fluid_ok
    }

    pub fn needs_refill(&self) -> bool {
        self.level_pct < 10.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.pump_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level() {
        let c = WasherFluid::new();
        assert!(c.level_ok());
    }

    #[test]
    fn test_system() {
        let c = WasherFluid::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WasherFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refill() {
        let c = WasherFluid::new();
        assert!(!c.needs_refill());
    }

    #[test]
    fn test_low() {
        let mut c = WasherFluid::new();
        c.level_pct = 5.0;
        assert!(c.needs_refill());
    }

    #[test]
    fn test_health() {
        let c = WasherFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
