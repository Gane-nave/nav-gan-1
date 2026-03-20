/// Rear camera: backup view, obstacle detection, trajectory overlay
/// Phase 179

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraState {
    Off,
    Active,
    Recording,
    Error,
}

#[derive(Debug, Clone)]
pub struct RearCamera {
    pub state: CameraState,
    pub resolution_w: u32,
    pub resolution_h: u32,
    pub fov_deg: f64,
    pub night_vision: bool,
    pub distance_lines: bool,
    pub closest_obstacle_m: f64,
}

impl Default for RearCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl RearCamera {
    pub fn new() -> Self {
        Self {
            state: CameraState::Off,
            resolution_w: 1280,
            resolution_h: 720,
            fov_deg: 170.0,
            night_vision: true,
            distance_lines: true,
            closest_obstacle_m: f64::MAX,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, CameraState::Active | CameraState::Recording)
    }

    pub fn obstacle_warning(&self) -> bool {
        self.is_active() && self.closest_obstacle_m < 1.0
    }

    pub fn obstacle_critical(&self) -> bool {
        self.is_active() && self.closest_obstacle_m < 0.3
    }

    pub fn megapixels(&self) -> f64 {
        (self.resolution_w as f64 * self.resolution_h as f64) / 1_000_000.0
    }

    pub fn has_error(&self) -> bool {
        matches!(self.state, CameraState::Error)
    }

    pub fn wide_angle(&self) -> bool {
        self.fov_deg > 150.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_off_by_default() {
        let c = RearCamera::new();
        assert!(!c.is_active());
    }

    #[test]
    fn test_active() {
        let mut c = RearCamera::new();
        c.state = CameraState::Active;
        assert!(c.is_active());
    }

    #[test]
    fn test_obstacle_warning() {
        let mut c = RearCamera::new();
        c.state = CameraState::Active;
        c.closest_obstacle_m = 0.5;
        assert!(c.obstacle_warning());
    }

    #[test]
    fn test_no_warning_far() {
        let mut c = RearCamera::new();
        c.state = CameraState::Active;
        c.closest_obstacle_m = 5.0;
        assert!(!c.obstacle_warning());
    }

    #[test]
    fn test_critical() {
        let mut c = RearCamera::new();
        c.state = CameraState::Active;
        c.closest_obstacle_m = 0.1;
        assert!(c.obstacle_critical());
    }

    #[test]
    fn test_megapixels() {
        let c = RearCamera::new();
        assert!(c.megapixels() > 0.9);
    }

    #[test]
    fn test_wide_angle() {
        let c = RearCamera::new();
        assert!(c.wide_angle());
    }
}
