/// aurora-ui-modal: ui modal
/// Phase 2403

#[derive(Debug, Clone)]
pub struct UiModal {
    pub open_ok: bool,
    pub close_ok: bool,
    pub confirm_ok: bool,
    pub cancel_ok: bool,
    pub animate_ok: bool,
}

impl Default for UiModal {
    fn default() -> Self {
        Self::new()
    }
}

impl UiModal {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            confirm_ok: true,
            cancel_ok: true,
            animate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.confirm_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.animate_ok
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
        let c = UiModal::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiModal::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiModal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiModal::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiModal::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiModal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiModal::default();
        assert!(c.all_ok());
    }
}
