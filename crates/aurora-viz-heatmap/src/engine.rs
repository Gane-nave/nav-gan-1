/// aurora-viz-heatmap: viz heatmap
/// Phase 2425

#[derive(Debug, Clone)]
pub struct VizHeatmap {
    pub render_ok: bool,
    pub scale_ok: bool,
    pub color_ok: bool,
    pub legend_ok: bool,
    pub zoom_ok: bool,
}

impl Default for VizHeatmap {
    fn default() -> Self {
        Self::new()
    }
}

impl VizHeatmap {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            scale_ok: true,
            color_ok: true,
            legend_ok: true,
            zoom_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.scale_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.legend_ok && self.zoom_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.scale_ok
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
        let c = VizHeatmap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizHeatmap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizHeatmap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizHeatmap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizHeatmap::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizHeatmap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizHeatmap::default();
        assert!(c.all_ok());
    }
}
