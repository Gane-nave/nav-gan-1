/// Preconditioning: battery thermal prep, cabin climate prep, departure time
/// Phase 293

#[derive(Debug, Clone)]
pub struct Preconditioning {
    pub active: bool,
    pub battery_heating: bool,
    pub battery_cooling: bool,
    pub cabin_heating: bool,
    pub cabin_cooling: bool,
    pub departure_set: bool,
    pub target_battery_temp_c: f64,
    pub current_battery_temp_c: f64,
}

impl Default for Preconditioning {
    fn default() -> Self {
        Self::new()
    }
}

impl Preconditioning {
    pub fn new() -> Self {
        Self {
            active: false,
            battery_heating: false,
            battery_cooling: false,
            cabin_heating: false,
            cabin_cooling: false,
            departure_set: false,
            target_battery_temp_c: 25.0,
            current_battery_temp_c: 20.0,
        }
    }

    pub fn any_active(&self) -> bool {
        self.battery_heating || self.battery_cooling || self.cabin_heating || self.cabin_cooling
    }

    pub fn battery_at_target(&self) -> bool {
        (self.current_battery_temp_c - self.target_battery_temp_c).abs() < 3.0
    }

    pub fn needs_heating(&self) -> bool {
        self.current_battery_temp_c < self.target_battery_temp_c - 5.0
    }

    pub fn needs_cooling(&self) -> bool {
        self.current_battery_temp_c > self.target_battery_temp_c + 5.0
    }

    pub fn health_score(&self) -> f64 {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_active() {
        let p = Preconditioning::new();
        assert!(!p.any_active());
    }

    #[test]
    fn test_at_target() {
        let p = Preconditioning::new();
        assert!(p.battery_at_target());
    }

    #[test]
    fn test_no_heating() {
        let p = Preconditioning::new();
        assert!(!p.needs_heating());
    }

    #[test]
    fn test_no_cooling() {
        let p = Preconditioning::new();
        assert!(!p.needs_cooling());
    }

    #[test]
    fn test_cold_battery() {
        let mut p = Preconditioning::new();
        p.current_battery_temp_c = -5.0;
        assert!(p.needs_heating());
    }

    #[test]
    fn test_health() {
        let p = Preconditioning::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
