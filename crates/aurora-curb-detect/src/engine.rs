/// Curb detection: height, distance, warning, avoidance
/// Phase 938

#[derive(Debug, Clone)]
pub struct CurbDetect {
    pub height_ok: bool,
    pub distance_ok: bool,
    pub warning_ok: bool,
    pub avoidance_ok: bool,
    pub lidar_ok: bool,
}

impl Default for CurbDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl CurbDetect {
    pub fn new() -> Self {
        Self {
            height_ok: true,
            distance_ok: true,
            warning_ok: true,
            avoidance_ok: true,
            lidar_ok: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.height_ok && self.distance_ok && self.lidar_ok
    }

    pub fn response_ok(&self) -> bool {
        self.warning_ok && self.avoidance_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.response_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.lidar_ok || !self.height_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lidar_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = CurbDetect::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_response() {
        let c = CurbDetect::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CurbDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = CurbDetect::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_lidar() {
        let mut c = CurbDetect::new();
        c.lidar_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = CurbDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
