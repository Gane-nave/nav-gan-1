/// Night vision: infrared, pedestrian, animal, display
/// Phase 929

#[derive(Debug, Clone)]
pub struct NightVision {
    pub infrared_ok: bool,
    pub pedestrian_ok: bool,
    pub animal_ok: bool,
    pub display_ok: bool,
    pub thermal_ok: bool,
}

impl Default for NightVision {
    fn default() -> Self {
        Self::new()
    }
}

impl NightVision {
    pub fn new() -> Self {
        Self {
            infrared_ok: true,
            pedestrian_ok: true,
            animal_ok: true,
            display_ok: true,
            thermal_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.infrared_ok && self.pedestrian_ok && self.animal_ok
    }

    pub fn output_ok(&self) -> bool {
        self.display_ok && self.thermal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.output_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.infrared_ok || !self.thermal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.infrared_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = NightVision::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_output() {
        let c = NightVision::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NightVision::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = NightVision::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_infrared() {
        let mut c = NightVision::new();
        c.infrared_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = NightVision::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
