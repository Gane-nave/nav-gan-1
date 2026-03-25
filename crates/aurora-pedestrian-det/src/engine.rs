/// Pedestrian detection: camera, radar, algorithm, alert
/// Phase 838

#[derive(Debug, Clone)]
pub struct PedestrianDet {
    pub camera_ok: bool,
    pub radar_ok: bool,
    pub algo_ok: bool,
    pub alert_ok: bool,
    pub brake_ok: bool,
}

impl Default for PedestrianDet {
    fn default() -> Self {
        Self::new()
    }
}

impl PedestrianDet {
    pub fn new() -> Self {
        Self {
            camera_ok: true,
            radar_ok: true,
            algo_ok: true,
            alert_ok: true,
            brake_ok: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.camera_ok && self.radar_ok
    }

    pub fn response_ok(&self) -> bool {
        self.algo_ok && self.alert_ok && self.brake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.response_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.camera_ok || !self.algo_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = PedestrianDet::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_response() {
        let c = PedestrianDet::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PedestrianDet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibration() {
        let c = PedestrianDet::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cam() {
        let mut c = PedestrianDet::new();
        c.camera_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = PedestrianDet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
