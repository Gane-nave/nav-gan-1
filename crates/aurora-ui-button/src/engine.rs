/// aurora-ui-button: ui button
/// Phase 2400

#[derive(Debug, Clone)]
pub struct UiButton {
    pub click_ok: bool,
    pub press_ok: bool,
    pub focus_ok: bool,
    pub hover_ok: bool,
    pub disable_ok: bool,
}

impl Default for UiButton {
    fn default() -> Self {
        Self::new()
    }
}

impl UiButton {
    pub fn new() -> Self {
        Self {
            click_ok: true,
            press_ok: true,
            focus_ok: true,
            hover_ok: true,
            disable_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.click_ok && self.press_ok && self.focus_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.hover_ok && self.disable_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.click_ok || !self.press_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.click_ok {
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
        let c = UiButton::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiButton::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiButton::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiButton::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiButton::new();
        c.click_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiButton::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiButton::default();
        assert!(c.all_ok());
    }
}
