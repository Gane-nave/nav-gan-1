/// Tailgate: panel, power strut, handle, camera
/// Phase 791

#[derive(Debug, Clone)]
pub struct Tailgate {
    pub panel_ok: bool,
    pub power_strut_ok: bool,
    pub handle_ok: bool,
    pub camera_ok: bool,
    pub seal_ok: bool,
}

impl Default for Tailgate {
    fn default() -> Self {
        Self::new()
    }
}

impl Tailgate {
    pub fn new() -> Self {
        Self {
            panel_ok: true,
            power_strut_ok: true,
            handle_ok: true,
            camera_ok: true,
            seal_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.panel_ok && self.seal_ok
    }

    pub fn features_ok(&self) -> bool {
        self.power_strut_ok && self.handle_ok && self.camera_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.power_strut_ok || !self.handle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.power_strut_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = Tailgate::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_features() {
        let c = Tailgate::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Tailgate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Tailgate::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_strut() {
        let mut c = Tailgate::new();
        c.power_strut_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Tailgate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
