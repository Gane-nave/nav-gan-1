/// aurora-ui-header: ui header
/// Phase 2408

#[derive(Debug, Clone)]
pub struct UiHeader {
    pub render_ok: bool,
    pub sticky_ok: bool,
    pub shrink_ok: bool,
    pub expand_ok: bool,
    pub theme_ok: bool,
}

impl Default for UiHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl UiHeader {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            sticky_ok: true,
            shrink_ok: true,
            expand_ok: true,
            theme_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.sticky_ok && self.shrink_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.expand_ok && self.theme_ok
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
        let c = UiHeader::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiHeader::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiHeader::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiHeader::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiHeader::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiHeader::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiHeader::default();
        assert!(c.all_ok());
    }
}
