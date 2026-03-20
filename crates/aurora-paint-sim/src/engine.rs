/// Paint simulation: spray, cure, thickness, color match
/// Phase 965

#[derive(Debug, Clone)]
pub struct PaintSim {
    pub spray_ok: bool,
    pub cure_ok: bool,
    pub thickness_ok: bool,
    pub color_ok: bool,
    pub validate_ok: bool,
}

impl Default for PaintSim {
    fn default() -> Self {
        Self::new()
    }
}

impl PaintSim {
    pub fn new() -> Self {
        Self {
            spray_ok: true,
            cure_ok: true,
            thickness_ok: true,
            color_ok: true,
            validate_ok: true,
        }
    }

    pub fn application_ok(&self) -> bool {
        self.spray_ok && self.cure_ok && self.thickness_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.color_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.application_ok() && self.quality_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.color_ok || !self.thickness_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spray_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application() {
        let c = PaintSim::new();
        assert!(c.application_ok());
    }

    #[test]
    fn test_quality() {
        let c = PaintSim::new();
        assert!(c.quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PaintSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = PaintSim::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_color() {
        let mut c = PaintSim::new();
        c.color_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = PaintSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
