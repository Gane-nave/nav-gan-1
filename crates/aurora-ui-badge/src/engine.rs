/// aurora-ui-badge: ui badge
/// Phase 2417

#[derive(Debug, Clone)]
pub struct UiBadge {
    pub show_ok: bool,
    pub hide_ok: bool,
    pub count_ok: bool,
    pub color_ok: bool,
    pub animate_ok: bool,
}

impl Default for UiBadge {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBadge {
    pub fn new() -> Self {
        Self {
            show_ok: true,
            hide_ok: true,
            count_ok: true,
            color_ok: true,
            animate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.show_ok && self.hide_ok && self.count_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.animate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.show_ok || !self.hide_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.show_ok {
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
        let c = UiBadge::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiBadge::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiBadge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiBadge::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiBadge::new();
        c.show_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiBadge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiBadge::default();
        assert!(c.all_ok());
    }
}
