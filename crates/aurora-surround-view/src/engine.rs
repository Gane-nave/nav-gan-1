/// Surround view: four cameras, stitching, calibration
/// Phase 740

#[derive(Debug, Clone)]
pub struct SurroundView {
    pub front_cam_ok: bool,
    pub rear_cam_ok: bool,
    pub left_cam_ok: bool,
    pub right_cam_ok: bool,
    pub calibrated: bool,
}

impl Default for SurroundView {
    fn default() -> Self {
        Self::new()
    }
}

impl SurroundView {
    pub fn new() -> Self {
        Self {
            front_cam_ok: true,
            rear_cam_ok: true,
            left_cam_ok: true,
            right_cam_ok: true,
            calibrated: true,
        }
    }

    pub fn cameras_ok(&self) -> bool {
        self.front_cam_ok && self.rear_cam_ok && self.left_cam_ok && self.right_cam_ok
    }

    pub fn stitching_ok(&self) -> bool {
        self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.cameras_ok() && self.stitching_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.front_cam_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.front_cam_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cameras() {
        let c = SurroundView::new();
        assert!(c.cameras_ok());
    }

    #[test]
    fn test_stitching() {
        let c = SurroundView::new();
        assert!(c.stitching_ok());
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
    fn test_cal() {
        let mut c = SurroundView::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = SurroundView::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
