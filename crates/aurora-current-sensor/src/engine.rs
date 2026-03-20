/// Current sensor: hall effect, shunt, range, offset
/// Phase 866

#[derive(Debug, Clone)]
pub struct CurrentSensor {
    pub hall_ok: bool,
    pub shunt_ok: bool,
    pub range_ok: bool,
    pub offset_ok: bool,
    pub calibration_ok: bool,
}

impl Default for CurrentSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentSensor {
    pub fn new() -> Self {
        Self {
            hall_ok: true,
            shunt_ok: true,
            range_ok: true,
            offset_ok: true,
            calibration_ok: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.hall_ok && self.shunt_ok && self.range_ok
    }

    pub fn accuracy_ok(&self) -> bool {
        self.offset_ok && self.calibration_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.accuracy_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibration_ok || !self.offset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hall_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = CurrentSensor::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_accuracy() {
        let c = CurrentSensor::new();
        assert!(c.accuracy_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CurrentSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = CurrentSensor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = CurrentSensor::new();
        c.calibration_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = CurrentSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
