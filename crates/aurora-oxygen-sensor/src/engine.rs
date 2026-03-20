/// Oxygen sensor: lambda measurement, air-fuel ratio, heater control
/// Phase 307

#[derive(Debug, Clone)]
pub struct OxygenSensor {
    pub lambda: f64,
    pub voltage: f64,
    pub heater_ok: bool,
    pub ready: bool,
    pub bank: u8,
    pub position: u8,
}

impl Default for OxygenSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl OxygenSensor {
    pub fn new() -> Self {
        Self {
            lambda: 1.0,
            voltage: 0.45,
            heater_ok: true,
            ready: true,
            bank: 1,
            position: 1,
        }
    }

    pub fn stoichiometric(&self) -> bool {
        (self.lambda - 1.0).abs() < 0.05
    }

    pub fn lean(&self) -> bool {
        self.lambda > 1.05
    }

    pub fn rich(&self) -> bool {
        self.lambda < 0.95
    }

    pub fn is_ready(&self) -> bool {
        self.ready && self.heater_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.heater_ok {
            return 30.0;
        }
        if !self.ready {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stoich() {
        let o = OxygenSensor::new();
        assert!(o.stoichiometric());
    }

    #[test]
    fn test_not_lean() {
        let o = OxygenSensor::new();
        assert!(!o.lean());
    }

    #[test]
    fn test_not_rich() {
        let o = OxygenSensor::new();
        assert!(!o.rich());
    }

    #[test]
    fn test_ready() {
        let o = OxygenSensor::new();
        assert!(o.is_ready());
    }

    #[test]
    fn test_lean() {
        let mut o = OxygenSensor::new();
        o.lambda = 1.2;
        assert!(o.lean());
    }

    #[test]
    fn test_health() {
        let o = OxygenSensor::new();
        assert!((o.health_score() - 100.0).abs() < 0.1);
    }
}
