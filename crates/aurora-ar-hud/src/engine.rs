/// AR HUD: projection, overlay, depth, brightness, calibration
/// Phase 898

#[derive(Debug, Clone)]
pub struct ArHud {
    pub projection_ok: bool,
    pub overlay_ok: bool,
    pub depth_ok: bool,
    pub brightness_ok: bool,
    pub calibration_ok: bool,
}

impl Default for ArHud {
    fn default() -> Self {
        Self::new()
    }
}

impl ArHud {
    pub fn new() -> Self {
        Self {
            projection_ok: true,
            overlay_ok: true,
            depth_ok: true,
            brightness_ok: true,
            calibration_ok: true,
        }
    }

    pub fn display_ok(&self) -> bool {
        self.projection_ok && self.overlay_ok && self.depth_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.brightness_ok && self.calibration_ok
    }

    pub fn all_ok(&self) -> bool {
        self.display_ok() && self.quality_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibration_ok || !self.projection_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.projection_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let c = ArHud::new();
        assert!(c.display_ok());
    }

    #[test]
    fn test_quality() {
        let c = ArHud::new();
        assert!(c.quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ArHud::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = ArHud::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = ArHud::new();
        c.calibration_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = ArHud::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
