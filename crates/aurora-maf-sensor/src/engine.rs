/// MAF sensor: air mass flow, hot wire, contamination
/// Phase 585

#[derive(Debug, Clone)]
pub struct MafSensor {
    pub flow_gs: f64,
    pub hot_wire_ok: bool,
    pub clean: bool,
    pub signal_ok: bool,
    pub calibrated: bool,
}

impl Default for MafSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl MafSensor {
    pub fn new() -> Self {
        Self {
            flow_gs: 15.0,
            hot_wire_ok: true,
            clean: true,
            signal_ok: true,
            calibrated: true,
        }
    }

    pub fn flow_valid(&self) -> bool {
        self.signal_ok && self.flow_gs > 0.0
    }

    pub fn sensor_clean(&self) -> bool {
        self.clean && self.hot_wire_ok
    }

    pub fn all_ok(&self) -> bool {
        self.flow_valid() && self.sensor_clean() && self.calibrated
    }

    pub fn needs_cleaning(&self) -> bool {
        !self.clean || !self.hot_wire_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hot_wire_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow() {
        let c = MafSensor::new();
        assert!(c.flow_valid());
    }

    #[test]
    fn test_clean() {
        let c = MafSensor::new();
        assert!(c.sensor_clean());
    }

    #[test]
    fn test_all_ok() {
        let c = MafSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_clean() {
        let c = MafSensor::new();
        assert!(!c.needs_cleaning());
    }

    #[test]
    fn test_dirty() {
        let mut c = MafSensor::new();
        c.clean = false;
        assert!(c.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let c = MafSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
