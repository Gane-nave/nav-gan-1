/// Rubber trim: flexibility, UV resist, color, seal
/// Phase 774

#[derive(Debug, Clone)]
pub struct RubberTrim {
    pub flexibility_ok: bool,
    pub uv_ok: bool,
    pub color_ok: bool,
    pub seal_ok: bool,
    pub adhesion_ok: bool,
}

impl Default for RubberTrim {
    fn default() -> Self {
        Self::new()
    }
}

impl RubberTrim {
    pub fn new() -> Self {
        Self {
            flexibility_ok: true,
            uv_ok: true,
            color_ok: true,
            seal_ok: true,
            adhesion_ok: true,
        }
    }

    pub fn material_ok(&self) -> bool {
        self.flexibility_ok && self.uv_ok
    }

    pub fn function_ok(&self) -> bool {
        self.color_ok && self.seal_ok && self.adhesion_ok
    }

    pub fn all_ok(&self) -> bool {
        self.material_ok() && self.function_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.flexibility_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flexibility_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material() {
        let c = RubberTrim::new();
        assert!(c.material_ok());
    }

    #[test]
    fn test_function() {
        let c = RubberTrim::new();
        assert!(c.function_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RubberTrim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = RubberTrim::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_flex() {
        let mut c = RubberTrim::new();
        c.flexibility_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = RubberTrim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
