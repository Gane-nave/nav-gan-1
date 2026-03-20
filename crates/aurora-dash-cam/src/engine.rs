/// Dash cam: front camera, rear camera, loop record, G-sensor
/// Phase 844

#[derive(Debug, Clone)]
pub struct DashCam {
    pub front_ok: bool,
    pub rear_ok: bool,
    pub loop_ok: bool,
    pub g_sensor_ok: bool,
    pub storage_ok: bool,
}

impl Default for DashCam {
    fn default() -> Self {
        Self::new()
    }
}

impl DashCam {
    pub fn new() -> Self {
        Self {
            front_ok: true,
            rear_ok: true,
            loop_ok: true,
            g_sensor_ok: true,
            storage_ok: true,
        }
    }

    pub fn recording_ok(&self) -> bool {
        self.front_ok && self.rear_ok && self.loop_ok
    }

    pub fn features_ok(&self) -> bool {
        self.g_sensor_ok && self.storage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recording_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.front_ok || !self.storage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.front_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording() {
        let c = DashCam::new();
        assert!(c.recording_ok());
    }

    #[test]
    fn test_features() {
        let c = DashCam::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashCam::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DashCam::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_front() {
        let mut c = DashCam::new();
        c.front_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DashCam::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
