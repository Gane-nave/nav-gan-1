/// aurora-viz-scatter: viz scatter
/// Phase 2423

#[derive(Debug, Clone)]
pub struct VizScatter {
    pub render_ok: bool,
    pub scale_ok: bool,
    pub color_ok: bool,
    pub cluster_ok: bool,
    pub zoom_ok: bool,
}

impl Default for VizScatter {
    fn default() -> Self {
        Self::new()
    }
}

impl VizScatter {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            scale_ok: true,
            color_ok: true,
            cluster_ok: true,
            zoom_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.scale_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cluster_ok && self.zoom_ok
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
        let c = VizScatter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizScatter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizScatter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizScatter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizScatter::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizScatter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizScatter::default();
        assert!(c.all_ok());
    }
}
