/// U-turn assist: space, traffic, steering, clearance
/// Phase 936

#[derive(Debug, Clone)]
pub struct UturnAssist {
    pub space_ok: bool,
    pub traffic_ok: bool,
    pub steering_ok: bool,
    pub clearance_ok: bool,
    pub sensor_ok: bool,
}

impl Default for UturnAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl UturnAssist {
    pub fn new() -> Self {
        Self {
            space_ok: true,
            traffic_ok: true,
            steering_ok: true,
            clearance_ok: true,
            sensor_ok: true,
        }
    }

    pub fn feasibility_ok(&self) -> bool {
        self.space_ok && self.clearance_ok && self.sensor_ok
    }

    pub fn execution_ok(&self) -> bool {
        self.traffic_ok && self.steering_ok
    }

    pub fn all_ok(&self) -> bool {
        self.feasibility_ok() && self.execution_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.sensor_ok || !self.space_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feasibility() {
        let c = UturnAssist::new();
        assert!(c.feasibility_ok());
    }

    #[test]
    fn test_execution() {
        let c = UturnAssist::new();
        assert!(c.execution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UturnAssist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = UturnAssist::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_sensor() {
        let mut c = UturnAssist::new();
        c.sensor_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = UturnAssist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
