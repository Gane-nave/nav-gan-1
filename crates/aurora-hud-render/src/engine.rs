/// hud render: project, overlay, adjust, dim, clear
/// Phase 1183

#[derive(Debug, Clone)]
pub struct HudRender {
    pub project_ok: bool,
    pub overlay_ok: bool,
    pub adjust_ok: bool,
    pub dim_ok: bool,
    pub clear_ok: bool,
}

impl Default for HudRender {
    fn default() -> Self {
        Self::new()
    }
}

impl HudRender {
    pub fn new() -> Self {
        Self {
            project_ok: true,
            overlay_ok: true,
            adjust_ok: true,
            dim_ok: true,
            clear_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.project_ok && self.overlay_ok && self.adjust_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dim_ok && self.clear_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.project_ok || !self.overlay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.project_ok {
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
        let c = HudRender::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HudRender::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HudRender::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HudRender::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HudRender::new();
        c.project_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HudRender::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
