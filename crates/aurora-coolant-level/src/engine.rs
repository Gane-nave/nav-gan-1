/// Coolant level sensor: float, switch, connector
/// Phase 693

#[derive(Debug, Clone)]
pub struct CoolantLevel {
    pub float_ok: bool,
    pub switch_ok: bool,
    pub connector_ok: bool,
    pub wiring_ok: bool,
    pub reading_ok: bool,
}

impl Default for CoolantLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl CoolantLevel {
    pub fn new() -> Self {
        Self {
            float_ok: true,
            switch_ok: true,
            connector_ok: true,
            wiring_ok: true,
            reading_ok: true,
        }
    }

    pub fn sensor_ok(&self) -> bool {
        self.float_ok && self.switch_ok
    }

    pub fn circuit_ok(&self) -> bool {
        self.connector_ok && self.wiring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok() && self.circuit_ok() && self.reading_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.float_ok || !self.switch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.float_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor() {
        let c = CoolantLevel::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_circuit() {
        let c = CoolantLevel::new();
        assert!(c.circuit_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CoolantLevel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CoolantLevel::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_float() {
        let mut c = CoolantLevel::new();
        c.float_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CoolantLevel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
