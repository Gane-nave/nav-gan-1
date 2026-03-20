/// Washer fluid: level monitoring, pump control, nozzle heating
/// Phase 241

#[derive(Debug, Clone)]
pub struct WasherFluid {
    pub level_pct: f64,
    pub pump_ok: bool,
    pub nozzle_heater_on: bool,
    pub fluid_temp_c: f64,
    pub low_warning: bool,
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
            pump_ok: true,
            nozzle_heater_on: false,
            fluid_temp_c: 15.0,
            low_warning: false,
        }
    }

    pub fn level_ok(&self) -> bool {
        self.level_pct > 15.0
    }

    pub fn needs_refill(&self) -> bool {
        self.level_pct < 10.0
    }

    pub fn frozen_risk(&self) -> bool {
        self.fluid_temp_c < -5.0
    }

    pub fn heater_needed(&self) -> bool {
        self.fluid_temp_c < 2.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.pump_ok {
            return 20.0;
        }
        if self.needs_refill() {
            return 30.0;
        }
        if !self.level_ok() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_ok() {
        let w = WasherFluid::new();
        assert!(w.level_ok());
    }

    #[test]
    fn test_no_refill() {
        let w = WasherFluid::new();
        assert!(!w.needs_refill());
    }

    #[test]
    fn test_no_frozen() {
        let w = WasherFluid::new();
        assert!(!w.frozen_risk());
    }

    #[test]
    fn test_no_heater() {
        let w = WasherFluid::new();
        assert!(!w.heater_needed());
    }

    #[test]
    fn test_low() {
        let mut w = WasherFluid::new();
        w.level_pct = 5.0;
        assert!(w.needs_refill());
    }

    #[test]
    fn test_health() {
        let w = WasherFluid::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
