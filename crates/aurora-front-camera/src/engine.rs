/// Front camera: ADAS vision, lane detection, sign recognition
/// Phase 180

#[derive(Debug, Clone)]
pub struct FrontCamera {
    pub active: bool,
    pub resolution_w: u32,
    pub resolution_h: u32,
    pub fov_deg: f64,
    pub fps: u32,
    pub lane_detected: bool,
    pub forward_distance_m: f64,
}

impl Default for FrontCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl FrontCamera {
    pub fn new() -> Self {
        Self {
            active: true,
            resolution_w: 1920,
            resolution_h: 1080,
            fov_deg: 120.0,
            fps: 30,
            lane_detected: true,
            forward_distance_m: 100.0,
        }
    }

    pub fn megapixels(&self) -> f64 {
        (self.resolution_w as f64 * self.resolution_h as f64) / 1_000_000.0
    }

    pub fn collision_risk(&self) -> bool {
        self.active && self.forward_distance_m < 5.0
    }

    pub fn lane_departure(&self) -> bool {
        self.active && !self.lane_detected
    }

    pub fn visibility_score(&self) -> f64 {
        let res_s = (self.megapixels() / 2.0).min(1.0) * 40.0;
        let fov_s = (self.fov_deg / 180.0) * 30.0;
        let fps_s = (self.fps as f64 / 60.0).min(1.0) * 30.0;
        res_s + fov_s + fps_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active() {
        let c = FrontCamera::new();
        assert!(c.active);
    }

    #[test]
    fn test_megapixels() {
        let c = FrontCamera::new();
        assert!(c.megapixels() > 2.0);
    }

    #[test]
    fn test_no_collision() {
        let c = FrontCamera::new();
        assert!(!c.collision_risk());
    }

    #[test]
    fn test_collision() {
        let mut c = FrontCamera::new();
        c.forward_distance_m = 3.0;
        assert!(c.collision_risk());
    }

    #[test]
    fn test_lane_ok() {
        let c = FrontCamera::new();
        assert!(!c.lane_departure());
    }

    #[test]
    fn test_lane_departure() {
        let mut c = FrontCamera::new();
        c.lane_detected = false;
        assert!(c.lane_departure());
    }

    #[test]
    fn test_visibility() {
        let c = FrontCamera::new();
        assert!(c.visibility_score() > 60.0);
    }
}
