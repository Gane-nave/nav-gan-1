/// hud display: render, overlay, brightness, color, refresh
/// Phase 1302

#[derive(Debug, Clone)]
pub struct HudDisplay {
    pub render_ok: bool,
    pub overlay_ok: bool,
    pub brightness_ok: bool,
    pub color_ok: bool,
    pub refresh_ok: bool,
}

impl Default for HudDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl HudDisplay {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            overlay_ok: true,
            brightness_ok: true,
            color_ok: true,
            refresh_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.overlay_ok && self.brightness_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.refresh_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.overlay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.render_ok {
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
        let c = HudDisplay::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HudDisplay::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HudDisplay::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HudDisplay::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HudDisplay::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HudDisplay::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
