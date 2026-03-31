/// aurora-dash-panel: dash panel
/// Phase 2441

#[derive(Debug, Clone)]
pub struct DashPanel {
    pub render_ok: bool,
    pub header_ok: bool,
    pub body_ok: bool,
    pub footer_ok: bool,
    pub resize_ok: bool,
}

impl Default for DashPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl DashPanel {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            header_ok: true,
            body_ok: true,
            footer_ok: true,
            resize_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.header_ok && self.body_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.footer_ok && self.resize_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.header_ok
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
        let c = DashPanel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashPanel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashPanel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashPanel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashPanel::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashPanel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashPanel::default();
        assert!(c.all_ok());
    }
}
