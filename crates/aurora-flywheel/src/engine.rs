/// Flywheel: dual-mass, vibration damping, ring gear condition
/// Phase 324

#[derive(Debug, Clone)]
pub struct Flywheel {
    pub dual_mass: bool,
    pub vibration_ok: bool,
    pub ring_gear_ok: bool,
    pub surface_ok: bool,
    pub wear_pct: f64,
}

impl Default for Flywheel {
    fn default() -> Self {
        Self::new()
    }
}

impl Flywheel {
    pub fn new() -> Self {
        Self {
            dual_mass: true,
            vibration_ok: true,
            ring_gear_ok: true,
            surface_ok: true,
            wear_pct: 15.0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.vibration_ok && self.ring_gear_ok && self.surface_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.surface_ok || self.wear_pct > 80.0
    }

    pub fn needs_resurfacing(&self) -> bool {
        self.wear_pct > 50.0 && self.wear_pct <= 80.0
    }

    pub fn remaining_life_pct(&self) -> f64 {
        (100.0 - self.wear_pct).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.ring_gear_ok {
            return 0.0;
        }
        if !self.surface_ok {
            return 20.0;
        }
        if !self.vibration_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let f = Flywheel::new();
        assert!(f.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let f = Flywheel::new();
        assert!(!f.needs_replacement());
    }

    #[test]
    fn test_no_resurface() {
        let f = Flywheel::new();
        assert!(!f.needs_resurfacing());
    }

    #[test]
    fn test_life() {
        let f = Flywheel::new();
        assert!(f.remaining_life_pct() > 80.0);
    }

    #[test]
    fn test_worn() {
        let mut f = Flywheel::new();
        f.wear_pct = 90.0;
        assert!(f.needs_replacement());
    }

    #[test]
    fn test_health() {
        let f = Flywheel::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
