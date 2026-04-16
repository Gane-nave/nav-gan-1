/// aurora-ui-tabs: ui tabs
/// Phase 2415

#[derive(Debug, Clone)]
pub struct UiTabs {
    pub select_ok: bool,
    pub add_ok: bool,
    pub close_ok: bool,
    pub scroll_ok: bool,
    pub focus_ok: bool,
}

impl Default for UiTabs {
    fn default() -> Self {
        Self::new()
    }
}

impl UiTabs {
    pub fn new() -> Self {
        Self {
            select_ok: true,
            add_ok: true,
            close_ok: true,
            scroll_ok: true,
            focus_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.select_ok && self.add_ok && self.close_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scroll_ok && self.focus_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.select_ok || !self.add_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.select_ok {
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
        let c = UiTabs::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiTabs::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiTabs::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiTabs::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiTabs::new();
        c.select_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiTabs::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiTabs::default();
        assert!(c.all_ok());
    }
}
