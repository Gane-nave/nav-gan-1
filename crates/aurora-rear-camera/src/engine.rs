/// Rear camera: image quality, lens, night vision, gridlines
/// Phase 545

#[derive(Debug, Clone)]
pub struct RearCamera {
    pub resolution_ok: bool,
    pub lens_clean: bool,
    pub night_vision_ok: bool,
    pub gridlines_ok: bool,
    pub wiring_ok: bool,
}

impl Default for RearCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl RearCamera {
    pub fn new() -> Self {
        Self {
            resolution_ok: true,
            lens_clean: true,
            night_vision_ok: true,
            gridlines_ok: true,
            wiring_ok: true,
        }
    }

    pub fn image_ok(&self) -> bool {
        self.resolution_ok && self.lens_clean
    }

    pub fn features_ok(&self) -> bool {
        self.night_vision_ok && self.gridlines_ok
    }

    pub fn all_ok(&self) -> bool {
        self.image_ok() && self.features_ok() && self.wiring_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.resolution_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.resolution_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let c = RearCamera::new();
        assert!(c.image_ok());
    }

    #[test]
    fn test_features() {
        let c = RearCamera::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RearCamera::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RearCamera::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_resolution() {
        let mut c = RearCamera::new();
        c.resolution_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RearCamera::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
