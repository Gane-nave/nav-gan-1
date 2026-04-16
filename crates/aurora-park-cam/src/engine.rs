/// park cam: capture, stitch, overlay, guide, record
/// Phase 1291

#[derive(Debug, Clone)]
pub struct ParkCam {
    pub capture_ok: bool,
    pub stitch_ok: bool,
    pub overlay_ok: bool,
    pub guide_ok: bool,
    pub record_ok: bool,
}

impl Default for ParkCam {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkCam {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            stitch_ok: true,
            overlay_ok: true,
            guide_ok: true,
            record_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.stitch_ok && self.overlay_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.guide_ok && self.record_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.stitch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ParkCam::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ParkCam::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ParkCam::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ParkCam::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ParkCam::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ParkCam::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
