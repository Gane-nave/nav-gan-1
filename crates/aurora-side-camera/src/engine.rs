/// Side camera: blind spot monitoring, surround view, lane change assist
/// Phase 181

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct SideCamera {
    pub side: Side,
    pub active: bool,
    pub blind_spot_occupied: bool,
    pub closest_vehicle_m: f64,
    pub fov_deg: f64,
}

impl SideCamera {
    pub fn new(side: Side) -> Self {
        Self {
            side,
            active: true,
            blind_spot_occupied: false,
            closest_vehicle_m: f64::MAX,
            fov_deg: 80.0,
        }
    }

    pub fn safe_to_change_lane(&self) -> bool {
        self.active && !self.blind_spot_occupied && self.closest_vehicle_m > 3.0
    }

    pub fn warning_active(&self) -> bool {
        self.blind_spot_occupied || self.closest_vehicle_m < 2.0
    }
}

#[derive(Debug, Clone)]
pub struct SideCameraSystem {
    pub left: SideCamera,
    pub right: SideCamera,
}

impl Default for SideCameraSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SideCameraSystem {
    pub fn new() -> Self {
        Self {
            left: SideCamera::new(Side::Left),
            right: SideCamera::new(Side::Right),
        }
    }

    pub fn any_warning(&self) -> bool {
        self.left.warning_active() || self.right.warning_active()
    }

    pub fn safe_left(&self) -> bool {
        self.left.safe_to_change_lane()
    }

    pub fn safe_right(&self) -> bool {
        self.right.safe_to_change_lane()
    }

    pub fn both_active(&self) -> bool {
        self.left.active && self.right.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_lane_change() {
        let c = SideCamera::new(Side::Left);
        assert!(c.safe_to_change_lane());
    }

    #[test]
    fn test_blind_spot() {
        let mut c = SideCamera::new(Side::Right);
        c.blind_spot_occupied = true;
        assert!(!c.safe_to_change_lane());
    }

    #[test]
    fn test_warning() {
        let mut c = SideCamera::new(Side::Left);
        c.closest_vehicle_m = 1.0;
        assert!(c.warning_active());
    }

    #[test]
    fn test_system_no_warning() {
        let s = SideCameraSystem::new();
        assert!(!s.any_warning());
    }

    #[test]
    fn test_both_active() {
        let s = SideCameraSystem::new();
        assert!(s.both_active());
    }

    #[test]
    fn test_safe_left() {
        let s = SideCameraSystem::new();
        assert!(s.safe_left());
    }

    #[test]
    fn test_safe_right() {
        let s = SideCameraSystem::new();
        assert!(s.safe_right());
    }
}
