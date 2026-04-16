/// aurora-viz-polar: viz polar
/// Phase 2427

#[derive(Debug, Clone)]
pub struct VizPolar {
    pub render_ok: bool,
    pub axis_ok: bool,
    pub grid_ok: bool,
    pub label_ok: bool,
    pub color_ok: bool,
}

impl Default for VizPolar {
    fn default() -> Self {
        Self::new()
    }
}

impl VizPolar {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            axis_ok: true,
            grid_ok: true,
            label_ok: true,
            color_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.axis_ok && self.grid_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.color_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.axis_ok
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
        let c = VizPolar::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizPolar::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizPolar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizPolar::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizPolar::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizPolar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizPolar::default();
        assert!(c.all_ok());
    }
}
