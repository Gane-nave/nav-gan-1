/// Dashboard pad: material, color, texture, defroster
/// Phase 777

#[derive(Debug, Clone)]
pub struct DashPad {
    pub material_ok: bool,
    pub color_ok: bool,
    pub texture_ok: bool,
    pub defroster_ok: bool,
    pub fit_ok: bool,
}

impl Default for DashPad {
    fn default() -> Self {
        Self::new()
    }
}

impl DashPad {
    pub fn new() -> Self {
        Self {
            material_ok: true,
            color_ok: true,
            texture_ok: true,
            defroster_ok: true,
            fit_ok: true,
        }
    }

    pub fn appearance_ok(&self) -> bool {
        self.material_ok && self.color_ok && self.texture_ok
    }

    pub fn function_ok(&self) -> bool {
        self.defroster_ok && self.fit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.appearance_ok() && self.function_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.material_ok || !self.defroster_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.material_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance() {
        let c = DashPad::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_function() {
        let c = DashPad::new();
        assert!(c.function_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashPad::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = DashPad::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_material() {
        let mut c = DashPad::new();
        c.material_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = DashPad::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
