/// ABS module: pump, valve, sensor, ECU
/// Phase 659

#[derive(Debug, Clone)]
pub struct AbsModule {
    pub pump_ok: bool,
    pub valve_ok: bool,
    pub sensor_ok: bool,
    pub ecu_ok: bool,
    pub calibrated: bool,
}

impl Default for AbsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsModule {
    pub fn new() -> Self {
        Self {
            pump_ok: true,
            valve_ok: true,
            sensor_ok: true,
            ecu_ok: true,
            calibrated: true,
        }
    }

    pub fn hydraulic_ok(&self) -> bool {
        self.pump_ok && self.valve_ok
    }

    pub fn electronic_ok(&self) -> bool {
        self.sensor_ok && self.ecu_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.hydraulic_ok() && self.electronic_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.pump_ok || !self.ecu_ok
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
    fn test_hydraulic() {
        let c = AbsModule::new();
        assert!(c.hydraulic_ok());
    }

    #[test]
    fn test_electronic() {
        let c = AbsModule::new();
        assert!(c.electronic_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AbsModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AbsModule::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_pump() {
        let mut c = AbsModule::new();
        c.pump_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AbsModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
