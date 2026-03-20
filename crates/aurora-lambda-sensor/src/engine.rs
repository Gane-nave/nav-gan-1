/// Lambda/O2 sensor: air-fuel ratio, wideband reading, heater circuit
/// Phase 213

#[derive(Debug, Clone)]
pub struct LambdaSensor {
    pub position: String,
    pub lambda_value: f64,
    pub voltage: f64,
    pub heater_on: bool,
    pub heater_current_a: f64,
    pub ready: bool,
}

impl Default for LambdaSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaSensor {
    pub fn new() -> Self {
        Self {
            position: "upstream".into(),
            lambda_value: 1.0,
            voltage: 0.45,
            heater_on: true,
            heater_current_a: 1.5,
            ready: true,
        }
    }

    pub fn stoichiometric(&self) -> bool {
        (self.lambda_value - 1.0).abs() < 0.03
    }

    pub fn running_rich(&self) -> bool {
        self.lambda_value < 0.97
    }

    pub fn running_lean(&self) -> bool {
        self.lambda_value > 1.03
    }

    pub fn heater_ok(&self) -> bool {
        self.heater_on && self.heater_current_a > 0.5 && self.heater_current_a < 3.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.ready {
            score -= 30.0;
        }
        if !self.heater_ok() {
            score -= 25.0;
        }
        if !self.stoichiometric() {
            score -= 15.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stoich() {
        let l = LambdaSensor::new();
        assert!(l.stoichiometric());
    }

    #[test]
    fn test_not_rich() {
        let l = LambdaSensor::new();
        assert!(!l.running_rich());
    }

    #[test]
    fn test_not_lean() {
        let l = LambdaSensor::new();
        assert!(!l.running_lean());
    }

    #[test]
    fn test_heater_ok() {
        let l = LambdaSensor::new();
        assert!(l.heater_ok());
    }

    #[test]
    fn test_rich() {
        let mut l = LambdaSensor::new();
        l.lambda_value = 0.85;
        assert!(l.running_rich());
    }

    #[test]
    fn test_health() {
        let l = LambdaSensor::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
