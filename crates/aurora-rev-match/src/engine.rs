/// Rev matching: downshift, blip, smooth, heel-toe
/// Phase 945

#[derive(Debug, Clone)]
pub struct RevMatch {
    pub downshift_ok: bool,
    pub blip_ok: bool,
    pub smooth_ok: bool,
    pub heel_toe_ok: bool,
    pub sensor_ok: bool,
}

impl Default for RevMatch {
    fn default() -> Self {
        Self::new()
    }
}

impl RevMatch {
    pub fn new() -> Self {
        Self {
            downshift_ok: true,
            blip_ok: true,
            smooth_ok: true,
            heel_toe_ok: true,
            sensor_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.downshift_ok && self.sensor_ok
    }

    pub fn execution_ok(&self) -> bool {
        self.blip_ok && self.smooth_ok && self.heel_toe_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.execution_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.sensor_ok || !self.downshift_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = RevMatch::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_execution() {
        let c = RevMatch::new();
        assert!(c.execution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RevMatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = RevMatch::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_sensor() {
        let mut c = RevMatch::new();
        c.sensor_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = RevMatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
