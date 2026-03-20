/// map 3d: model, texture, render, animate, log
/// Phase 1439

#[derive(Debug, Clone)]
pub struct Map3d {
    pub model_ok: bool,
    pub texture_ok: bool,
    pub render_ok: bool,
    pub animate_ok: bool,
    pub log_ok: bool,
}

impl Default for Map3d {
    fn default() -> Self {
        Self::new()
    }
}

impl Map3d {
    pub fn new() -> Self {
        Self {
            model_ok: true,
            texture_ok: true,
            render_ok: true,
            animate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.model_ok && self.texture_ok && self.render_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.animate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.model_ok || !self.texture_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.model_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = Map3d::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Map3d::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Map3d::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Map3d::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Map3d::new();
        c.model_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Map3d::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
