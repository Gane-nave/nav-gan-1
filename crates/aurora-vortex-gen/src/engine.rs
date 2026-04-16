/// Vortex generator: boundary layer control, flow attachment
/// Phase 354

#[derive(Debug, Clone)]
pub struct VortexGen {
    pub count: u8,
    pub height_mm: f64,
    pub spacing_mm: f64,
    pub installed: bool,
    pub effective: bool,
}

impl Default for VortexGen {
    fn default() -> Self {
        Self::new()
    }
}

impl VortexGen {
    pub fn new() -> Self {
        Self {
            count: 12,
            height_mm: 8.0,
            spacing_mm: 25.0,
            installed: true,
            effective: true,
        }
    }

    pub fn active(&self) -> bool {
        self.installed && self.count > 0
    }

    pub fn properly_spaced(&self) -> bool {
        self.spacing_mm > 15.0 && self.spacing_mm < 50.0
    }

    pub fn height_ok(&self) -> bool {
        self.height_mm > 3.0 && self.height_mm < 15.0
    }

    pub fn all_ok(&self) -> bool {
        self.active() && self.properly_spaced() && self.height_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.installed {
            return 0.0;
        }
        if !self.effective {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active() {
        let v = VortexGen::new();
        assert!(v.active());
    }

    #[test]
    fn test_spaced() {
        let v = VortexGen::new();
        assert!(v.properly_spaced());
    }

    #[test]
    fn test_height() {
        let v = VortexGen::new();
        assert!(v.height_ok());
    }

    #[test]
    fn test_all_ok() {
        let v = VortexGen::new();
        assert!(v.all_ok());
    }

    #[test]
    fn test_not_installed() {
        let mut v = VortexGen::new();
        v.installed = false;
        assert!(!v.active());
    }

    #[test]
    fn test_health() {
        let v = VortexGen::new();
        assert!((v.health_score() - 100.0).abs() < 0.1);
    }
}
