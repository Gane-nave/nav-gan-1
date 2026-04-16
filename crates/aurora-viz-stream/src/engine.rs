/// aurora-viz-stream: viz stream
/// Phase 2437

#[derive(Debug, Clone)]
pub struct VizStream {
    pub render_ok: bool,
    pub layer_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
    pub zoom_ok: bool,
}

impl Default for VizStream {
    fn default() -> Self {
        Self::new()
    }
}

impl VizStream {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            layer_ok: true,
            color_ok: true,
            label_ok: true,
            zoom_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.layer_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.zoom_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.layer_ok
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
        let c = VizStream::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizStream::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizStream::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizStream::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizStream::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizStream::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizStream::default();
        assert!(c.all_ok());
    }
}
