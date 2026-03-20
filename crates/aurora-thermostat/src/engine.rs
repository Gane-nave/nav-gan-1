/// Thermostat: coolant temperature regulation, opening/closing, wax element
/// Phase 321

#[derive(Debug, Clone)]
pub struct Thermostat {
    pub coolant_temp_c: f64,
    pub opening_temp_c: f64,
    pub position_pct: f64,
    pub stuck_open: bool,
    pub stuck_closed: bool,
}

impl Default for Thermostat {
    fn default() -> Self {
        Self::new()
    }
}

impl Thermostat {
    pub fn new() -> Self {
        Self {
            coolant_temp_c: 85.0,
            opening_temp_c: 82.0,
            position_pct: 50.0,
            stuck_open: false,
            stuck_closed: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.position_pct > 10.0
    }

    pub fn should_be_open(&self) -> bool {
        self.coolant_temp_c >= self.opening_temp_c
    }

    pub fn functioning(&self) -> bool {
        !self.stuck_open && !self.stuck_closed
    }

    pub fn temp_ok(&self) -> bool {
        self.coolant_temp_c > 70.0 && self.coolant_temp_c < 110.0
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck_closed {
            return 0.0;
        }
        if self.stuck_open {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let t = Thermostat::new();
        assert!(t.is_open());
    }

    #[test]
    fn test_should_open() {
        let t = Thermostat::new();
        assert!(t.should_be_open());
    }

    #[test]
    fn test_functioning() {
        let t = Thermostat::new();
        assert!(t.functioning());
    }

    #[test]
    fn test_temp() {
        let t = Thermostat::new();
        assert!(t.temp_ok());
    }

    #[test]
    fn test_stuck() {
        let mut t = Thermostat::new();
        t.stuck_closed = true;
        assert!(!t.functioning());
    }

    #[test]
    fn test_health() {
        let t = Thermostat::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
