/// ABS module: hydraulic unit, speed sensors, pump
/// Phase 488

#[derive(Debug, Clone)]
pub struct AbsModule {
    pub pump_ok: bool,
    pub valve_ok: bool,
    pub sensors_ok: bool,
    pub ecu_ok: bool,
    pub warning_light: bool,
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
            sensors_ok: true,
            ecu_ok: true,
            warning_light: false,
        }
    }

    pub fn system_ok(&self) -> bool {
        self.pump_ok && self.valve_ok && self.sensors_ok && self.ecu_ok
    }

    pub fn is_active(&self) -> bool {
        self.system_ok() && !self.warning_light
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && !self.warning_light
    }

    pub fn needs_service(&self) -> bool {
        self.warning_light || !self.system_ok()
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
    fn test_system() {
        let c = AbsModule::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_active() {
        let c = AbsModule::new();
        assert!(c.is_active());
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
    fn test_warning() {
        let mut c = AbsModule::new();
        c.warning_light = true;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AbsModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
