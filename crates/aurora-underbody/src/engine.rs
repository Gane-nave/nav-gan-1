/// Underbody panel: flat floor, aerodynamic sealing, stone protection
/// Phase 358

#[derive(Debug, Clone)]
pub struct Underbody {
    pub panels_ok: bool,
    pub panel_count: u8,
    pub fasteners_ok: bool,
    pub flat_floor: bool,
    pub damage_detected: bool,
}

impl Default for Underbody {
    fn default() -> Self {
        Self::new()
    }
}

impl Underbody {
    pub fn new() -> Self {
        Self {
            panels_ok: true,
            panel_count: 6,
            fasteners_ok: true,
            flat_floor: true,
            damage_detected: false,
        }
    }

    pub fn sealed(&self) -> bool {
        self.panels_ok && self.fasteners_ok
    }

    pub fn aero_effective(&self) -> bool {
        self.flat_floor && self.sealed()
    }

    pub fn needs_repair(&self) -> bool {
        self.damage_detected || !self.panels_ok
    }

    pub fn all_ok(&self) -> bool {
        self.panels_ok && self.fasteners_ok && !self.damage_detected
    }

    pub fn health_score(&self) -> f64 {
        if self.damage_detected {
            return 20.0;
        }
        if !self.panels_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealed() {
        let u = Underbody::new();
        assert!(u.sealed());
    }

    #[test]
    fn test_aero() {
        let u = Underbody::new();
        assert!(u.aero_effective());
    }

    #[test]
    fn test_no_repair() {
        let u = Underbody::new();
        assert!(!u.needs_repair());
    }

    #[test]
    fn test_all_ok() {
        let u = Underbody::new();
        assert!(u.all_ok());
    }

    #[test]
    fn test_damaged() {
        let mut u = Underbody::new();
        u.damage_detected = true;
        assert!(u.needs_repair());
    }

    #[test]
    fn test_health() {
        let u = Underbody::new();
        assert!((u.health_score() - 100.0).abs() < 0.1);
    }
}
