/// Camera fusion: capture, rectify, stereo, depth, fuse
/// Phase 1106

#[derive(Debug, Clone)]
pub struct CameraFusion {
    pub capture_ok: bool,
    pub rectify_ok: bool,
    pub stereo_ok: bool,
    pub depth_ok: bool,
    pub fuse_ok: bool,
}

impl Default for CameraFusion {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraFusion {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            rectify_ok: true,
            stereo_ok: true,
            depth_ok: true,
            fuse_ok: true,
        }
    }

    pub fn imaging_ok(&self) -> bool {
        self.capture_ok && self.rectify_ok && self.stereo_ok
    }

    pub fn perception_ok(&self) -> bool {
        self.depth_ok && self.fuse_ok
    }

    pub fn all_ok(&self) -> bool {
        self.imaging_ok() && self.perception_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.capture_ok || !self.rectify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imaging() {
        let c = CameraFusion::new();
        assert!(c.imaging_ok());
    }

    #[test]
    fn test_perception() {
        let c = CameraFusion::new();
        assert!(c.perception_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CameraFusion::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = CameraFusion::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_capture() {
        let mut c = CameraFusion::new();
        c.capture_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = CameraFusion::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
