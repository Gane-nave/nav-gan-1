/// aurora-dash-map: dash map
/// Phase 2452

#[derive(Debug, Clone)]
pub struct DashMap {
    pub render_ok: bool,
    pub layer_ok: bool,
    pub zoom_ok: bool,
    pub pan_ok: bool,
    pub select_ok: bool,
}

impl Default for DashMap {
    fn default() -> Self {
        Self::new()
    }
}

impl DashMap {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            layer_ok: true,
            zoom_ok: true,
            pan_ok: true,
            select_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.layer_ok && self.zoom_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.pan_ok && self.select_ok
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
        let c = DashMap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashMap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashMap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashMap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashMap::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashMap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashMap::default();
        assert!(c.all_ok());
    }
}
