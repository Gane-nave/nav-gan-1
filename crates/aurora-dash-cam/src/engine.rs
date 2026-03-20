/// Dash cam: continuous recording, event detection, loop recording
/// Phase 270

#[derive(Debug, Clone)]
pub struct DashCam {
    pub recording: bool,
    pub storage_used_pct: f64,
    pub event_detected: bool,
    pub resolution: u16,
    pub fps: u8,
    pub camera_ok: bool,
}

impl Default for DashCam {
    fn default() -> Self {
        Self::new()
    }
}

impl DashCam {
    pub fn new() -> Self {
        Self {
            recording: true,
            storage_used_pct: 30.0,
            event_detected: false,
            resolution: 1080,
            fps: 30,
            camera_ok: true,
        }
    }

    pub fn storage_ok(&self) -> bool {
        self.storage_used_pct < 90.0
    }

    pub fn needs_cleanup(&self) -> bool {
        self.storage_used_pct > 80.0
    }

    pub fn is_recording(&self) -> bool {
        self.recording && self.camera_ok
    }

    pub fn high_quality(&self) -> bool {
        self.resolution >= 1080 && self.fps >= 30
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 0.0;
        }
        if !self.storage_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_ok() {
        let d = DashCam::new();
        assert!(d.storage_ok());
    }

    #[test]
    fn test_no_cleanup() {
        let d = DashCam::new();
        assert!(!d.needs_cleanup());
    }

    #[test]
    fn test_recording() {
        let d = DashCam::new();
        assert!(d.is_recording());
    }

    #[test]
    fn test_high_quality() {
        let d = DashCam::new();
        assert!(d.high_quality());
    }

    #[test]
    fn test_full_storage() {
        let mut d = DashCam::new();
        d.storage_used_pct = 95.0;
        assert!(!d.storage_ok());
    }

    #[test]
    fn test_health() {
        let d = DashCam::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
