/// mirror disp: camera, overlay, dim, signal, blind
/// Phase 1306

#[derive(Debug, Clone)]
pub struct MirrorDisp {
    pub camera_ok: bool,
    pub overlay_ok: bool,
    pub dim_ok: bool,
    pub signal_ok: bool,
    pub blind_ok: bool,
}

impl Default for MirrorDisp {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorDisp {
    pub fn new() -> Self {
        Self {
            camera_ok: true,
            overlay_ok: true,
            dim_ok: true,
            signal_ok: true,
            blind_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.camera_ok && self.overlay_ok && self.dim_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.signal_ok && self.blind_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.camera_ok || !self.overlay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
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
        let c = MirrorDisp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MirrorDisp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MirrorDisp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MirrorDisp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MirrorDisp::new();
        c.camera_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MirrorDisp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
