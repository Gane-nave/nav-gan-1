/// aurora-ui-nav: ui nav
/// Phase 2406

#[derive(Debug, Clone)]
pub struct UiNav {
    pub route_ok: bool,
    pub active_ok: bool,
    pub collapse_ok: bool,
    pub scroll_ok: bool,
    pub focus_ok: bool,
}

impl Default for UiNav {
    fn default() -> Self {
        Self::new()
    }
}

impl UiNav {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            active_ok: true,
            collapse_ok: true,
            scroll_ok: true,
            focus_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.active_ok && self.collapse_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scroll_ok && self.focus_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.active_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
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
        let c = UiNav::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiNav::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiNav::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiNav::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiNav::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiNav::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiNav::default();
        assert!(c.all_ok());
    }
}
