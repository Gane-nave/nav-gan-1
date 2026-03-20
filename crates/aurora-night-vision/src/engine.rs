/// Night vision: infrared camera, thermal imaging, pedestrian highlight
/// Phase 230

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NvMode {
    Off,
    NearInfrared,
    FarInfrared,
    Thermal,
}

#[derive(Debug, Clone)]
pub struct NightVisionSystem {
    pub mode: NvMode,
    pub active: bool,
    pub range_m: f64,
    pub objects_detected: u32,
    pub ambient_light_lux: f64,
    pub camera_temp_c: f64,
}

impl Default for NightVisionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl NightVisionSystem {
    pub fn new() -> Self {
        Self {
            mode: NvMode::FarInfrared,
            active: false,
            range_m: 300.0,
            objects_detected: 0,
            ambient_light_lux: 50.0,
            camera_temp_c: 35.0,
        }
    }

    pub fn should_activate(&self) -> bool {
        self.ambient_light_lux < 10.0
    }

    pub fn is_active(&self) -> bool {
        self.active && self.mode != NvMode::Off
    }

    pub fn has_detections(&self) -> bool {
        self.objects_detected > 0
    }

    pub fn camera_overheated(&self) -> bool {
        self.camera_temp_c > 80.0
    }

    pub fn health_score(&self) -> f64 {
        if self.camera_overheated() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_active() {
        let n = NightVisionSystem::new();
        assert!(!n.is_active());
    }

    #[test]
    fn test_should_not_activate() {
        let n = NightVisionSystem::new();
        assert!(!n.should_activate());
    }

    #[test]
    fn test_no_detections() {
        let n = NightVisionSystem::new();
        assert!(!n.has_detections());
    }

    #[test]
    fn test_not_overheated() {
        let n = NightVisionSystem::new();
        assert!(!n.camera_overheated());
    }

    #[test]
    fn test_dark_activate() {
        let mut n = NightVisionSystem::new();
        n.ambient_light_lux = 2.0;
        assert!(n.should_activate());
    }

    #[test]
    fn test_health() {
        let n = NightVisionSystem::new();
        assert!((n.health_score() - 100.0).abs() < 0.1);
    }
}
