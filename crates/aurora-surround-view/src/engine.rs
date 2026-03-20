/// Surround view: 360° camera stitching, bird's eye view, obstacle overlay
/// Phase 269

#[derive(Debug, Clone)]
pub struct SurroundView {
    pub front_cam_ok: bool,
    pub rear_cam_ok: bool,
    pub left_cam_ok: bool,
    pub right_cam_ok: bool,
    pub stitching_ok: bool,
    pub active: bool,
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
            stitching_ok: true,
            active: false,
        }
    }

    pub fn all_cameras_ok(&self) -> bool {
        self.front_cam_ok && self.rear_cam_ok && self.left_cam_ok && self.right_cam_ok
    }

    pub fn working_cameras(&self) -> u8 {
        let mut count: u8 = 0;
        if self.front_cam_ok {
            count += 1;
        }
        if self.rear_cam_ok {
            count += 1;
        }
        if self.left_cam_ok {
            count += 1;
        }
        if self.right_cam_ok {
            count += 1;
        }
        count
    }

    pub fn can_stitch(&self) -> bool {
        self.working_cameras() >= 3 && self.stitching_ok
    }

    pub fn full_view(&self) -> bool {
        self.all_cameras_ok() && self.stitching_ok
    }

    pub fn health_score(&self) -> f64 {
        self.working_cameras() as f64 / 4.0 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_cameras() {
        let s = SurroundView::new();
        assert!(s.all_cameras_ok());
    }

    #[test]
    fn test_count() {
        let s = SurroundView::new();
        assert_eq!(s.working_cameras(), 4);
    }

    #[test]
    fn test_can_stitch() {
        let s = SurroundView::new();
        assert!(s.can_stitch());
    }

    #[test]
    fn test_full_view() {
        let s = SurroundView::new();
        assert!(s.full_view());
    }

    #[test]
    fn test_camera_fail() {
        let mut s = SurroundView::new();
        s.front_cam_ok = false;
        assert!(!s.all_cameras_ok());
    }

    #[test]
    fn test_health() {
        let s = SurroundView::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
