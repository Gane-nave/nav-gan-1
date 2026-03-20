/// Thermostat valve: coolant flow control, warm-up optimization
/// Phase 220

#[derive(Debug, Clone)]
pub struct ThermostatValve {
    pub position_pct: f64,
    pub coolant_temp_c: f64,
    pub opening_temp_c: f64,
    pub full_open_temp_c: f64,
    pub stuck: bool,
}

impl Default for ThermostatValve {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermostatValve {
    pub fn new() -> Self {
        Self {
            position_pct: 80.0,
            coolant_temp_c: 90.0,
            opening_temp_c: 82.0,
            full_open_temp_c: 95.0,
            stuck: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.position_pct > 5.0
    }

    pub fn is_fully_open(&self) -> bool {
        self.position_pct > 95.0
    }

    pub fn expected_position(&self) -> f64 {
        if self.coolant_temp_c < self.opening_temp_c {
            return 0.0;
        }
        let range = self.full_open_temp_c - self.opening_temp_c;
        if range <= 0.0 {
            return 100.0;
        }
        ((self.coolant_temp_c - self.opening_temp_c) / range * 100.0).clamp(0.0, 100.0)
    }

    pub fn position_error(&self) -> f64 {
        (self.position_pct - self.expected_position()).abs()
    }

    pub fn needs_replacement(&self) -> bool {
        self.stuck || self.position_error() > 30.0
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck {
            return 10.0;
        }
        let error = self.position_error();
        if error > 30.0 {
            return 30.0;
        }
        if error > 15.0 {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let t = ThermostatValve::new();
        assert!(t.is_open());
    }

    #[test]
    fn test_not_fully_open() {
        let t = ThermostatValve::new();
        assert!(!t.is_fully_open());
    }

    #[test]
    fn test_expected_position() {
        let t = ThermostatValve::new();
        assert!(t.expected_position() > 50.0);
    }

    #[test]
    fn test_no_replacement() {
        let t = ThermostatValve::new();
        assert!(!t.needs_replacement());
    }

    #[test]
    fn test_stuck() {
        let mut t = ThermostatValve::new();
        t.stuck = true;
        assert!(t.needs_replacement());
    }

    #[test]
    fn test_health() {
        let t = ThermostatValve::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
