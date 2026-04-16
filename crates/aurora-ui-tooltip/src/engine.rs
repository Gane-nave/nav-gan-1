/// aurora-ui-tooltip: ui tooltip
/// Phase 2419

#[derive(Debug, Clone)]
pub struct UiTooltip {
    pub show_ok: bool,
    pub hide_ok: bool,
    pub position_ok: bool,
    pub delay_ok: bool,
    pub theme_ok: bool,
}

impl Default for UiTooltip {
    fn default() -> Self {
        Self::new()
    }
}

impl UiTooltip {
    pub fn new() -> Self {
        Self {
            show_ok: true,
            hide_ok: true,
            position_ok: true,
            delay_ok: true,
            theme_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.show_ok && self.hide_ok && self.position_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delay_ok && self.theme_ok
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
        let c = UiTooltip::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiTooltip::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiTooltip::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiTooltip::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiTooltip::new();
        c.show_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiTooltip::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiTooltip::default();
        assert!(c.all_ok());
    }
}
