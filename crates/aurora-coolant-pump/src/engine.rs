/// Coolant pump: electric water pump, flow rate, pressure monitoring
/// Phase 300

#[derive(Debug, Clone)]
pub struct CoolantPump {
    pub flow_rate_lpm: f64,
    pub pressure_bar: f64,
    pub rpm: f64,
    pub current_a: f64,
    pub motor_ok: bool,
}

impl Default for CoolantPump {
    fn default() -> Self {
        Self::new()
    }
}

impl CoolantPump {
    pub fn new() -> Self {
        Self {
            flow_rate_lpm: 10.0,
            pressure_bar: 1.5,
            rpm: 3000.0,
            current_a: 2.0,
            motor_ok: true,
        }
    }

    pub fn is_running(&self) -> bool {
        self.rpm > 100.0
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_rate_lpm > 5.0
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_bar > 0.5 && self.pressure_bar < 3.0
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.flow_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if !self.flow_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running() {
        let c = CoolantPump::new();
        assert!(c.is_running());
    }

    #[test]
    fn test_flow_ok() {
        let c = CoolantPump::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_pressure_ok() {
        let c = CoolantPump::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CoolantPump::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor_fail() {
        let mut c = CoolantPump::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CoolantPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
