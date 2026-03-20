/// Roof rack: load capacity, crossbar position, aerodynamic drag
/// Phase 336

#[derive(Debug, Clone)]
pub struct RoofRack {
    pub installed: bool,
    pub load_kg: f64,
    pub max_load_kg: f64,
    pub crossbars_locked: bool,
    pub aero_fairing: bool,
}

impl Default for RoofRack {
    fn default() -> Self {
        Self::new()
    }
}

impl RoofRack {
    pub fn new() -> Self {
        Self {
            installed: false,
            load_kg: 0.0,
            max_load_kg: 75.0,
            crossbars_locked: true,
            aero_fairing: false,
        }
    }

    pub fn loaded(&self) -> bool {
        self.load_kg > 1.0
    }

    pub fn overloaded(&self) -> bool {
        self.load_kg > self.max_load_kg
    }

    pub fn secure(&self) -> bool {
        self.crossbars_locked && !self.overloaded()
    }

    pub fn load_pct(&self) -> f64 {
        if self.max_load_kg <= 0.0 {
            return 0.0;
        }
        (self.load_kg / self.max_load_kg * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.overloaded() {
            return 20.0;
        }
        if !self.crossbars_locked {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_loaded() {
        let r = RoofRack::new();
        assert!(!r.loaded());
    }

    #[test]
    fn test_not_overloaded() {
        let r = RoofRack::new();
        assert!(!r.overloaded());
    }

    #[test]
    fn test_secure() {
        let r = RoofRack::new();
        assert!(r.secure());
    }

    #[test]
    fn test_load_pct() {
        let r = RoofRack::new();
        assert!(r.load_pct() < 1.0);
    }

    #[test]
    fn test_overloaded() {
        let mut r = RoofRack::new();
        r.load_kg = 100.0;
        assert!(r.overloaded());
    }

    #[test]
    fn test_health() {
        let r = RoofRack::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
