/// Fuel level sensor: sender, float, resistor, ground
/// Phase 695

#[derive(Debug, Clone)]
pub struct FuelLevel {
    pub sender_ok: bool,
    pub float_ok: bool,
    pub resistor_ok: bool,
    pub ground_ok: bool,
    pub reading_ok: bool,
}

impl Default for FuelLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelLevel {
    pub fn new() -> Self {
        Self {
            sender_ok: true,
            float_ok: true,
            resistor_ok: true,
            ground_ok: true,
            reading_ok: true,
        }
    }

    pub fn sensor_ok(&self) -> bool {
        self.sender_ok && self.float_ok && self.resistor_ok
    }

    pub fn circuit_ok(&self) -> bool {
        self.ground_ok && self.reading_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok() && self.circuit_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.sender_ok || !self.float_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sender_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor() {
        let c = FuelLevel::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_circuit() {
        let c = FuelLevel::new();
        assert!(c.circuit_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelLevel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = FuelLevel::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_sender() {
        let mut c = FuelLevel::new();
        c.sender_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = FuelLevel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
