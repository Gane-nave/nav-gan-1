/// Thermostat valve: coolant temperature regulation, wax element
/// Phase 443

#[derive(Debug, Clone)]
pub struct ThermostatVal {
    pub open_pct: f64,
    pub temp_c: f64,
    pub target_c: f64,
    pub stuck: bool,
    pub responsive: bool,
}

impl Default for ThermostatVal {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermostatVal {
    pub fn new() -> Self {
        Self {
            open_pct: 50.0,
            temp_c: 90.0,
            target_c: 90.0,
            stuck: false,
            responsive: true,
        }
    }

    pub fn regulating(&self) -> bool {
        !self.stuck && self.responsive
    }

    pub fn temp_ok(&self) -> bool {
        (self.temp_c - self.target_c).abs() < 5.0
    }

    pub fn all_ok(&self) -> bool {
        self.regulating() && self.temp_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.stuck
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck {
            return 0.0;
        }
        if !self.temp_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regulating() {
        let t = ThermostatVal::new();
        assert!(t.regulating());
    }

    #[test]
    fn test_temp() {
        let t = ThermostatVal::new();
        assert!(t.temp_ok());
    }

    #[test]
    fn test_all_ok() {
        let t = ThermostatVal::new();
        assert!(t.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let t = ThermostatVal::new();
        assert!(!t.needs_replacement());
    }

    #[test]
    fn test_stuck() {
        let mut t = ThermostatVal::new();
        t.stuck = true;
        assert!(t.needs_replacement());
    }

    #[test]
    fn test_health() {
        let t = ThermostatVal::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
