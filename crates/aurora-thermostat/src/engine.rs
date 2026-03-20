/// Thermostat: wax pellet, opening temp, bypass
/// Phase 512

#[derive(Debug, Clone)]
pub struct Thermostat {
    pub opening_temp_c: f64,
    pub current_temp_c: f64,
    pub is_open: bool,
    pub stuck: bool,
    pub bypass_ok: bool,
}

impl Default for Thermostat {
    fn default() -> Self {
        Self::new()
    }
}

impl Thermostat {
    pub fn new() -> Self {
        Self {
            opening_temp_c: 82.0,
            current_temp_c: 90.0,
            is_open: true,
            stuck: false,
            bypass_ok: true,
        }
    }

    pub fn should_be_open(&self) -> bool {
        self.current_temp_c > self.opening_temp_c
    }

    pub fn position_correct(&self) -> bool {
        self.should_be_open() == self.is_open
    }

    pub fn all_ok(&self) -> bool {
        self.position_correct() && !self.stuck && self.bypass_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.stuck
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_open() {
        let c = Thermostat::new();
        assert!(c.should_be_open());
    }

    #[test]
    fn test_position() {
        let c = Thermostat::new();
        assert!(c.position_correct());
    }

    #[test]
    fn test_all_ok() {
        let c = Thermostat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Thermostat::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_stuck() {
        let mut c = Thermostat::new();
        c.stuck = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Thermostat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
