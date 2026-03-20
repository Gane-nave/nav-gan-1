/// Interior camera: occupant monitoring, child presence, theft detection
/// Phase 271

#[derive(Debug, Clone)]
pub struct InteriorCamera {
    pub active: bool,
    pub occupant_count: u8,
    pub child_detected: bool,
    pub movement_detected: bool,
    pub camera_ok: bool,
    pub ir_mode: bool,
}

impl Default for InteriorCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl InteriorCamera {
    pub fn new() -> Self {
        Self {
            active: true,
            occupant_count: 1,
            child_detected: false,
            movement_detected: false,
            camera_ok: true,
            ir_mode: false,
        }
    }

    pub fn has_occupants(&self) -> bool {
        self.occupant_count > 0
    }

    pub fn child_left_behind(&self, engine_off: bool) -> bool {
        engine_off && self.child_detected
    }

    pub fn intrusion_detected(&self, vehicle_locked: bool) -> bool {
        vehicle_locked && self.movement_detected
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_occupants() {
        let i = InteriorCamera::new();
        assert!(i.has_occupants());
    }

    #[test]
    fn test_no_child_left() {
        let i = InteriorCamera::new();
        assert!(!i.child_left_behind(true));
    }

    #[test]
    fn test_no_intrusion() {
        let i = InteriorCamera::new();
        assert!(!i.intrusion_detected(true));
    }

    #[test]
    fn test_camera_ok() {
        let i = InteriorCamera::new();
        assert!(i.camera_ok);
    }

    #[test]
    fn test_child_alert() {
        let mut i = InteriorCamera::new();
        i.child_detected = true;
        assert!(i.child_left_behind(true));
    }

    #[test]
    fn test_health() {
        let i = InteriorCamera::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
