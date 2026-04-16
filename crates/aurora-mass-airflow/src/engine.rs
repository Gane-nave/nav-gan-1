/// MAF sensor: mass air flow, hot wire, intake temp
/// Phase 509

#[derive(Debug, Clone)]
pub struct MassAirflow {
    pub flow_gs: f64,
    pub voltage_v: f64,
    pub intake_temp_c: f64,
    pub sensor_ok: bool,
    pub contaminated: bool,
}

impl Default for MassAirflow {
    fn default() -> Self {
        Self::new()
    }
}

impl MassAirflow {
    pub fn new() -> Self {
        Self {
            flow_gs: 25.0,
            voltage_v: 2.5,
            intake_temp_c: 30.0,
            sensor_ok: true,
            contaminated: false,
        }
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_gs > 5.0 && self.flow_gs < 200.0
    }

    pub fn voltage_ok(&self) -> bool {
        self.voltage_v > 0.5 && self.voltage_v < 4.5
    }

    pub fn all_ok(&self) -> bool {
        self.flow_ok() && self.voltage_ok() && self.sensor_ok && !self.contaminated
    }

    pub fn needs_cleaning(&self) -> bool {
        self.contaminated
    }

    pub fn health_score(&self) -> f64 {
        if self.contaminated {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow() {
        let c = MassAirflow::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_voltage() {
        let c = MassAirflow::new();
        assert!(c.voltage_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MassAirflow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_clean() {
        let c = MassAirflow::new();
        assert!(!c.needs_cleaning());
    }

    #[test]
    fn test_contaminated() {
        let mut c = MassAirflow::new();
        c.contaminated = true;
        assert!(c.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let c = MassAirflow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
