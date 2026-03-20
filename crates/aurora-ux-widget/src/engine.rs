/// ux widget: create, render, interact, update, log
/// Phase 1502

#[derive(Debug, Clone)]
pub struct UxWidget {
    pub create_ok: bool,
    pub render_ok: bool,
    pub interact_ok: bool,
    pub update_ok: bool,
    pub log_ok: bool,
}

impl Default for UxWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl UxWidget {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            render_ok: true,
            interact_ok: true,
            update_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.render_ok && self.interact_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.update_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.render_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = UxWidget::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxWidget::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxWidget::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxWidget::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxWidget::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxWidget::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
