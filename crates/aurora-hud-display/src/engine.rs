/// HUD display: head-up display, combiner, projection, brightness
/// Phase 428

#[derive(Debug, Clone)]
pub struct HudDisplay {
    pub brightness_pct: f64,
    pub focus_ok: bool,
    pub combiner_ok: bool,
    pub projector_ok: bool,
    pub auto_brightness: bool,
}

impl Default for HudDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl HudDisplay {
    pub fn new() -> Self {
        Self {
            brightness_pct: 75.0,
            focus_ok: true,
            combiner_ok: true,
            projector_ok: true,
            auto_brightness: true,
        }
    }

    pub fn visible(&self) -> bool {
        self.brightness_pct > 30.0 && self.projector_ok
    }

    pub fn all_ok(&self) -> bool {
        self.visible() && self.focus_ok && self.combiner_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.projector_ok || !self.combiner_ok
    }

    pub fn readable(&self) -> bool {
        self.visible() && self.focus_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.projector_ok {
            return 0.0;
        }
        if !self.focus_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible() {
        let h = HudDisplay::new();
        assert!(h.visible());
    }

    #[test]
    fn test_all_ok() {
        let h = HudDisplay::new();
        assert!(h.all_ok());
    }

    #[test]
    fn test_no_service() {
        let h = HudDisplay::new();
        assert!(!h.needs_service());
    }

    #[test]
    fn test_readable() {
        let h = HudDisplay::new();
        assert!(h.readable());
    }

    #[test]
    fn test_bad_projector() {
        let mut h = HudDisplay::new();
        h.projector_ok = false;
        assert!(h.needs_service());
    }

    #[test]
    fn test_health() {
        let h = HudDisplay::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
