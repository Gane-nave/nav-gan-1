/// aurora-ui-panel: ui panel
/// Phase 2416

#[derive(Debug, Clone)]
pub struct UiPanel {
    pub toggle_ok: bool,
    pub resize_ok: bool,
    pub pin_ok: bool,
    pub theme_ok: bool,
    pub scroll_ok: bool,
}

impl Default for UiPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl UiPanel {
    pub fn new() -> Self {
        Self {
            toggle_ok: true,
            resize_ok: true,
            pin_ok: true,
            theme_ok: true,
            scroll_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.toggle_ok && self.resize_ok && self.pin_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.theme_ok && self.scroll_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.toggle_ok || !self.resize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.toggle_ok {
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
        let c = UiPanel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiPanel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiPanel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiPanel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiPanel::new();
        c.toggle_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiPanel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiPanel::default();
        assert!(c.all_ok());
    }
}
