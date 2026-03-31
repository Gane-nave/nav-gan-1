/// aurora-viz-sankey: viz sankey
/// Phase 2430

#[derive(Debug, Clone)]
pub struct VizSankey {
    pub render_ok: bool,
    pub node_ok: bool,
    pub link_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
}

impl Default for VizSankey {
    fn default() -> Self {
        Self::new()
    }
}

impl VizSankey {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            node_ok: true,
            link_ok: true,
            color_ok: true,
            label_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.node_ok && self.link_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.label_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.node_ok
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
        let c = VizSankey::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizSankey::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizSankey::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizSankey::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizSankey::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizSankey::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizSankey::default();
        assert!(c.all_ok());
    }
}
