/// aurora-ui-footer: ui footer
/// Phase 2409

#[derive(Debug, Clone)]
pub struct UiFooter {
    pub render_ok: bool,
    pub sticky_ok: bool,
    pub links_ok: bool,
    pub theme_ok: bool,
    pub scroll_ok: bool,
}

impl Default for UiFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl UiFooter {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            sticky_ok: true,
            links_ok: true,
            theme_ok: true,
            scroll_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.sticky_ok && self.links_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.theme_ok && self.scroll_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.sticky_ok
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
        let c = UiFooter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiFooter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiFooter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiFooter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiFooter::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiFooter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiFooter::default();
        assert!(c.all_ok());
    }
}
