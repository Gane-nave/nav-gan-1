/// Digital mirror: camera, display, auto-dim, recording
/// Phase 743

#[derive(Debug, Clone)]
pub struct DigitalMirror {
    pub camera_ok: bool,
    pub display_ok: bool,
    pub auto_dim_ok: bool,
    pub recording_ok: bool,
    pub calibrated: bool,
}

impl Default for DigitalMirror {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalMirror {
    pub fn new() -> Self {
        Self {
            camera_ok: true,
            display_ok: true,
            auto_dim_ok: true,
            recording_ok: true,
            calibrated: true,
        }
    }

    pub fn viewing_ok(&self) -> bool {
        self.camera_ok && self.display_ok && self.calibrated
    }

    pub fn features_ok(&self) -> bool {
        self.auto_dim_ok && self.recording_ok
    }

    pub fn all_ok(&self) -> bool {
        self.viewing_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.camera_ok || !self.display_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewing() {
        let c = DigitalMirror::new();
        assert!(c.viewing_ok());
    }

    #[test]
    fn test_features() {
        let c = DigitalMirror::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DigitalMirror::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DigitalMirror::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_camera() {
        let mut c = DigitalMirror::new();
        c.camera_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DigitalMirror::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
