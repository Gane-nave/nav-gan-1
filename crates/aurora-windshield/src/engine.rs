/// Windshield: laminated glass, PVB, antenna, HUD zone
/// Phase 780

#[derive(Debug, Clone)]
pub struct Windshield {
    pub laminated_ok: bool,
    pub pvb_ok: bool,
    pub antenna_ok: bool,
    pub hud_zone_ok: bool,
    pub seal_ok: bool,
}

impl Default for Windshield {
    fn default() -> Self {
        Self::new()
    }
}

impl Windshield {
    pub fn new() -> Self {
        Self {
            laminated_ok: true,
            pvb_ok: true,
            antenna_ok: true,
            hud_zone_ok: true,
            seal_ok: true,
        }
    }

    pub fn glass_ok(&self) -> bool {
        self.laminated_ok && self.pvb_ok
    }

    pub fn features_ok(&self) -> bool {
        self.antenna_ok && self.hud_zone_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.glass_ok() && self.features_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.laminated_ok || !self.pvb_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.laminated_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glass() {
        let c = Windshield::new();
        assert!(c.glass_ok());
    }

    #[test]
    fn test_features() {
        let c = Windshield::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Windshield::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Windshield::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_lam() {
        let mut c = Windshield::new();
        c.laminated_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Windshield::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
