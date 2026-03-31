/// aurora-dash-widget: dash widget
/// Phase 2440

#[derive(Debug, Clone)]
pub struct DashWidget {
    pub render_ok: bool,
    pub config_ok: bool,
    pub refresh_ok: bool,
    pub resize_ok: bool,
    pub theme_ok: bool,
}

impl Default for DashWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl DashWidget {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            config_ok: true,
            refresh_ok: true,
            resize_ok: true,
            theme_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.config_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resize_ok && self.theme_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.config_ok
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
        let c = DashWidget::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashWidget::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashWidget::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashWidget::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashWidget::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashWidget::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashWidget::default();
        assert!(c.all_ok());
    }
}
