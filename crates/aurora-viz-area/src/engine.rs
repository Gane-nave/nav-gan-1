/// aurora-viz-area: viz area
/// Phase 2424

#[derive(Debug, Clone)]
pub struct VizArea {
    pub render_ok: bool,
    pub fill_ok: bool,
    pub stroke_ok: bool,
    pub scale_ok: bool,
    pub tooltip_ok: bool,
}

impl Default for VizArea {
    fn default() -> Self {
        Self::new()
    }
}

impl VizArea {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            fill_ok: true,
            stroke_ok: true,
            scale_ok: true,
            tooltip_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.fill_ok && self.stroke_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scale_ok && self.tooltip_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.fill_ok
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
        let c = VizArea::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizArea::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizArea::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizArea::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizArea::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizArea::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizArea::default();
        assert!(c.all_ok());
    }
}
