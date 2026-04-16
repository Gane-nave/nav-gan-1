/// aurora-viz-force: viz force
/// Phase 2432

#[derive(Debug, Clone)]
pub struct VizForce {
    pub render_ok: bool,
    pub node_ok: bool,
    pub edge_ok: bool,
    pub color_ok: bool,
    pub drag_ok: bool,
}

impl Default for VizForce {
    fn default() -> Self {
        Self::new()
    }
}

impl VizForce {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            node_ok: true,
            edge_ok: true,
            color_ok: true,
            drag_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.node_ok && self.edge_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.drag_ok
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
        let c = VizForce::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizForce::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizForce::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizForce::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizForce::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizForce::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizForce::default();
        assert!(c.all_ok());
    }
}
