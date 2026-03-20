/// Intercooler monitoring: charge air temp, efficiency, pressure drop
/// Phase 192

#[derive(Debug, Clone)]
pub struct Intercooler {
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,
    pub inlet_pressure_bar: f64,
    pub outlet_pressure_bar: f64,
    pub ambient_temp_c: f64,
}

impl Default for Intercooler {
    fn default() -> Self {
        Self::new()
    }
}

impl Intercooler {
    pub fn new() -> Self {
        Self {
            inlet_temp_c: 150.0,
            outlet_temp_c: 45.0,
            inlet_pressure_bar: 2.5,
            outlet_pressure_bar: 2.4,
            ambient_temp_c: 25.0,
        }
    }

    pub fn temp_drop_c(&self) -> f64 {
        self.inlet_temp_c - self.outlet_temp_c
    }

    pub fn efficiency_pct(&self) -> f64 {
        let max_drop = self.inlet_temp_c - self.ambient_temp_c;
        if max_drop <= 0.0 {
            return 0.0;
        }
        ((self.temp_drop_c() / max_drop) * 100.0).clamp(0.0, 100.0)
    }

    pub fn pressure_drop_bar(&self) -> f64 {
        self.inlet_pressure_bar - self.outlet_pressure_bar
    }

    pub fn pressure_drop_ok(&self) -> bool {
        self.pressure_drop_bar() < 0.3
    }

    pub fn needs_cleaning(&self) -> bool {
        self.efficiency_pct() < 60.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if self.efficiency_pct() < 70.0 {
            score -= 30.0;
        }
        if !self.pressure_drop_ok() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_drop() {
        let ic = Intercooler::new();
        assert!((ic.temp_drop_c() - 105.0).abs() < 0.1);
    }

    #[test]
    fn test_efficiency() {
        let ic = Intercooler::new();
        assert!(ic.efficiency_pct() > 80.0);
    }

    #[test]
    fn test_pressure_drop_ok() {
        let ic = Intercooler::new();
        assert!(ic.pressure_drop_ok());
    }

    #[test]
    fn test_no_cleaning() {
        let ic = Intercooler::new();
        assert!(!ic.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let ic = Intercooler::new();
        assert!((ic.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_poor_efficiency() {
        let mut ic = Intercooler::new();
        ic.outlet_temp_c = 120.0;
        assert!(ic.needs_cleaning());
    }
}
