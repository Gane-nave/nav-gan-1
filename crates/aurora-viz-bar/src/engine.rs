/// aurora-viz-bar: viz bar
/// Phase 2421

#[derive(Debug, Clone)]
pub struct VizBar {
    pub render_ok: bool,
    pub group_ok: bool,
    pub stack_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
}

impl Default for VizBar {
    fn default() -> Self {
        Self::new()
    }
}

impl VizBar {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            group_ok: true,
            stack_ok: true,
            color_ok: true,
            label_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.group_ok && self.stack_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.label_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.group_ok
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
        let c = VizBar::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizBar::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizBar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizBar::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizBar::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizBar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizBar::default();
        assert!(c.all_ok());
    }
}
