/// Fuel injection control: injector pulse width, fuel trim, spray pattern
/// Phase 190

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InjectorType {
    PortFuel,
    DirectGasoline,
    DirectDiesel,
}

#[derive(Debug, Clone)]
pub struct FuelInjector {
    pub injector_type: InjectorType,
    pub pulse_width_ms: f64,
    pub fuel_pressure_bar: f64,
    pub short_term_trim_pct: f64,
    pub long_term_trim_pct: f64,
}

impl Default for FuelInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelInjector {
    pub fn new() -> Self {
        Self {
            injector_type: InjectorType::PortFuel,
            pulse_width_ms: 3.0,
            fuel_pressure_bar: 3.5,
            short_term_trim_pct: 0.0,
            long_term_trim_pct: 0.0,
        }
    }

    pub fn total_trim_pct(&self) -> f64 {
        self.short_term_trim_pct + self.long_term_trim_pct
    }

    pub fn trim_in_range(&self) -> bool {
        self.total_trim_pct().abs() < 25.0
    }

    pub fn pressure_ok(&self) -> bool {
        match self.injector_type {
            InjectorType::PortFuel => {
                (2.5..=4.5).contains(&self.fuel_pressure_bar)
            }
            InjectorType::DirectGasoline => {
                (50.0..=200.0).contains(&self.fuel_pressure_bar)
            }
            InjectorType::DirectDiesel => {
                (200.0..=2500.0).contains(&self.fuel_pressure_bar)
            }
        }
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.trim_in_range() {
            score -= 40.0;
        }
        if !self.pressure_ok() {
            score -= 30.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_healthy() {
        let f = FuelInjector::new();
        assert!(f.trim_in_range());
    }

    #[test]
    fn test_pressure_ok() {
        let f = FuelInjector::new();
        assert!(f.pressure_ok());
    }

    #[test]
    fn test_total_trim() {
        let mut f = FuelInjector::new();
        f.short_term_trim_pct = 5.0;
        f.long_term_trim_pct = 3.0;
        assert!((f.total_trim_pct() - 8.0).abs() < 0.1);
    }

    #[test]
    fn test_trim_out_of_range() {
        let mut f = FuelInjector::new();
        f.short_term_trim_pct = 20.0;
        f.long_term_trim_pct = 10.0;
        assert!(!f.trim_in_range());
    }

    #[test]
    fn test_health_score() {
        let f = FuelInjector::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_direct_diesel_pressure() {
        let mut f = FuelInjector::new();
        f.injector_type = InjectorType::DirectDiesel;
        f.fuel_pressure_bar = 1500.0;
        assert!(f.pressure_ok());
    }
}
