/// Adaptive lighting: matrix LED, cornering, leveling
/// Phase 747

#[derive(Debug, Clone)]
pub struct AdaptiveLight {
    pub matrix_ok: bool,
    pub cornering_ok: bool,
    pub leveling_ok: bool,
    pub sensor_ok: bool,
    pub calibrated: bool,
}

impl Default for AdaptiveLight {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveLight {
    pub fn new() -> Self {
        Self {
            matrix_ok: true,
            cornering_ok: true,
            leveling_ok: true,
            sensor_ok: true,
            calibrated: true,
        }
    }

    pub fn beam_ok(&self) -> bool {
        self.matrix_ok && self.cornering_ok
    }

    pub fn adjustment_ok(&self) -> bool {
        self.leveling_ok && self.sensor_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.beam_ok() && self.adjustment_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.leveling_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.matrix_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam() {
        let c = AdaptiveLight::new();
        assert!(c.beam_ok());
    }

    #[test]
    fn test_adjustment() {
        let c = AdaptiveLight::new();
        assert!(c.adjustment_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AdaptiveLight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = AdaptiveLight::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = AdaptiveLight::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = AdaptiveLight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
