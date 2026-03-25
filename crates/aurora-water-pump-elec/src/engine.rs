/// Electric water pump: motor, impeller, control
/// Phase 618

#[derive(Debug, Clone)]
pub struct ElecWaterPump {
    pub motor_ok: bool,
    pub impeller_ok: bool,
    pub flow_lpm: f64,
    pub control_ok: bool,
    pub seal_ok: bool,
}

impl Default for ElecWaterPump {
    fn default() -> Self {
        Self::new()
    }
}

impl ElecWaterPump {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            impeller_ok: true,
            flow_lpm: 45.0,
            control_ok: true,
            seal_ok: true,
        }
    }

    pub fn pump_ok(&self) -> bool {
        self.motor_ok && self.impeller_ok
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_lpm > 20.0
    }

    pub fn all_ok(&self) -> bool {
        self.pump_ok() && self.flow_ok() && self.control_ok && self.seal_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pump() {
        let c = ElecWaterPump::new();
        assert!(c.pump_ok());
    }

    #[test]
    fn test_flow() {
        let c = ElecWaterPump::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ElecWaterPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ElecWaterPump::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = ElecWaterPump::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ElecWaterPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
