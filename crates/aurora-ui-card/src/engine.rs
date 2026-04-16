/// aurora-ui-card: ui card
/// Phase 2405

#[derive(Debug, Clone)]
pub struct UiCard {
    pub render_ok: bool,
    pub flip_ok: bool,
    pub expand_ok: bool,
    pub collapse_ok: bool,
    pub focus_ok: bool,
}

impl Default for UiCard {
    fn default() -> Self {
        Self::new()
    }
}

impl UiCard {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            flip_ok: true,
            expand_ok: true,
            collapse_ok: true,
            focus_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.flip_ok && self.expand_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.collapse_ok && self.focus_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.flip_ok
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
        let c = UiCard::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiCard::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiCard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiCard::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiCard::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiCard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiCard::default();
        assert!(c.all_ok());
    }
}
