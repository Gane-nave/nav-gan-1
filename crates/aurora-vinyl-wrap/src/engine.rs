/// Vinyl wrap: conformability, air release, adhesive, color
/// Phase 772

#[derive(Debug, Clone)]
pub struct VinylWrap {
    pub conform_ok: bool,
    pub air_release_ok: bool,
    pub adhesive_ok: bool,
    pub color_ok: bool,
    pub edge_ok: bool,
}

impl Default for VinylWrap {
    fn default() -> Self {
        Self::new()
    }
}

impl VinylWrap {
    pub fn new() -> Self {
        Self {
            conform_ok: true,
            air_release_ok: true,
            adhesive_ok: true,
            color_ok: true,
            edge_ok: true,
        }
    }

    pub fn application_ok(&self) -> bool {
        self.conform_ok && self.air_release_ok && self.edge_ok
    }

    pub fn appearance_ok(&self) -> bool {
        self.adhesive_ok && self.color_ok
    }

    pub fn all_ok(&self) -> bool {
        self.application_ok() && self.appearance_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.adhesive_ok || !self.conform_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.adhesive_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application() {
        let c = VinylWrap::new();
        assert!(c.application_ok());
    }

    #[test]
    fn test_appearance() {
        let c = VinylWrap::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VinylWrap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = VinylWrap::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_adhesive() {
        let mut c = VinylWrap::new();
        c.adhesive_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = VinylWrap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
