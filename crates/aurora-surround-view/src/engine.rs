/// Surround view: camera, stitch, 3D, overlay, record
/// Phase 930

#[derive(Debug, Clone)]
pub struct SurroundView {
    pub camera_ok: bool,
    pub stitch_ok: bool,
    pub view_3d_ok: bool,
    pub overlay_ok: bool,
    pub record_ok: bool,
}

impl Default for SurroundView {
    fn default() -> Self {
        Self::new()
    }
}

impl SurroundView {
    pub fn new() -> Self {
        Self {
            camera_ok: true,
            stitch_ok: true,
            view_3d_ok: true,
            overlay_ok: true,
            record_ok: true,
        }
    }

    pub fn capture_ok(&self) -> bool {
        self.camera_ok && self.stitch_ok
    }

    pub fn rendering_ok(&self) -> bool {
        self.view_3d_ok && self.overlay_ok && self.record_ok
    }

    pub fn all_ok(&self) -> bool {
        self.capture_ok() && self.rendering_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.camera_ok || !self.stitch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture() {
        let c = SurroundView::new();
        assert!(c.capture_ok());
    }

    #[test]
    fn test_rendering() {
        let c = SurroundView::new();
        assert!(c.rendering_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SurroundView::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = SurroundView::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_camera() {
        let mut c = SurroundView::new();
        c.camera_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = SurroundView::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
