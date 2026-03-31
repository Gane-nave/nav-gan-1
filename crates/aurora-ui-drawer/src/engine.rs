/// aurora-ui-drawer: ui drawer
/// Phase 2404

#[derive(Debug, Clone)]
pub struct UiDrawer {
    pub open_ok: bool,
    pub close_ok: bool,
    pub pin_ok: bool,
    pub resize_ok: bool,
    pub scroll_ok: bool,
}

impl Default for UiDrawer {
    fn default() -> Self {
        Self::new()
    }
}

impl UiDrawer {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            pin_ok: true,
            resize_ok: true,
            scroll_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.pin_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resize_ok && self.scroll_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.close_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok {
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
        let c = UiDrawer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiDrawer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiDrawer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiDrawer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiDrawer::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiDrawer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiDrawer::default();
        assert!(c.all_ok());
    }
}
