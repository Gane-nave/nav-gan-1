/// Road surface: friction, wet, ice, gravel, quality
/// Phase 941

#[derive(Debug, Clone)]
pub struct RoadSurface {
    pub friction_ok: bool,
    pub wet_ok: bool,
    pub ice_ok: bool,
    pub gravel_ok: bool,
    pub quality_ok: bool,
}

impl Default for RoadSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadSurface {
    pub fn new() -> Self {
        Self {
            friction_ok: true,
            wet_ok: true,
            ice_ok: true,
            gravel_ok: true,
            quality_ok: true,
        }
    }

    pub fn condition_ok(&self) -> bool {
        self.friction_ok && self.wet_ok && self.ice_ok
    }

    pub fn classification_ok(&self) -> bool {
        self.gravel_ok && self.quality_ok
    }

    pub fn all_ok(&self) -> bool {
        self.condition_ok() && self.classification_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.friction_ok || !self.wet_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.friction_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition() {
        let c = RoadSurface::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_classification() {
        let c = RoadSurface::new();
        assert!(c.classification_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoadSurface::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = RoadSurface::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_friction() {
        let mut c = RoadSurface::new();
        c.friction_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = RoadSurface::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
