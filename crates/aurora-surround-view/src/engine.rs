/// surround view: capture, stitch, render, overlay, stream
/// Phase 1175

#[derive(Debug, Clone)]
pub struct SurroundView {
    pub capture_ok: bool,
    pub stitch_ok: bool,
    pub render_ok: bool,
    pub overlay_ok: bool,
    pub stream_ok: bool,
}

impl Default for SurroundView {
    fn default() -> Self {
        Self::new()
    }
}

impl SurroundView {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            stitch_ok: true,
            render_ok: true,
            overlay_ok: true,
            stream_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.stitch_ok && self.render_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.overlay_ok && self.stream_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.stitch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
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
        let c = SurroundView::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SurroundView::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SurroundView::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SurroundView::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SurroundView::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SurroundView::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
