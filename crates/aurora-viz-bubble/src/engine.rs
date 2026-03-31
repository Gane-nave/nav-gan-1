/// aurora-viz-bubble: viz bubble
/// Phase 2436

#[derive(Debug, Clone)]
pub struct VizBubble {
    pub render_ok: bool,
    pub size_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
    pub zoom_ok: bool,
}

impl Default for VizBubble {
    fn default() -> Self {
        Self::new()
    }
}

impl VizBubble {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            size_ok: true,
            color_ok: true,
            label_ok: true,
            zoom_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.size_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.zoom_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.size_ok
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
        let c = VizBubble::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizBubble::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizBubble::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizBubble::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizBubble::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizBubble::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizBubble::default();
        assert!(c.all_ok());
    }
}
