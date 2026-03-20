/// Traction control: wheel slip, throttle cut, brake apply
/// Phase 670

#[derive(Debug, Clone)]
pub struct TractionControl {
    pub slip_sensor_ok: bool,
    pub throttle_ok: bool,
    pub brake_apply_ok: bool,
    pub ecu_ok: bool,
    pub enabled: bool,
}

impl Default for TractionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl TractionControl {
    pub fn new() -> Self {
        Self {
            slip_sensor_ok: true,
            throttle_ok: true,
            brake_apply_ok: true,
            ecu_ok: true,
            enabled: true,
        }
    }

    pub fn sensors_ok(&self) -> bool {
        self.slip_sensor_ok && self.throttle_ok
    }

    pub fn intervention_ok(&self) -> bool {
        self.brake_apply_ok && self.ecu_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensors_ok() && self.intervention_ok() && self.enabled
    }

    pub fn needs_service(&self) -> bool {
        !self.ecu_ok || !self.slip_sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ecu_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensors() {
        let c = TractionControl::new();
        assert!(c.sensors_ok());
    }

    #[test]
    fn test_intervention() {
        let c = TractionControl::new();
        assert!(c.intervention_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TractionControl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TractionControl::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ecu() {
        let mut c = TractionControl::new();
        c.ecu_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TractionControl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
