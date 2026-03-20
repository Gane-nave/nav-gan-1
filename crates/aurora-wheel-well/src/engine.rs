/// Wheel well: liner, turbulence management, splash guard
/// Phase 357

#[derive(Debug, Clone)]
pub struct WheelWell {
    pub liner_ok: bool,
    pub splash_guard_ok: bool,
    pub clearance_mm: f64,
    pub debris_buildup: bool,
    pub aero_sealed: bool,
}

impl Default for WheelWell {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelWell {
    pub fn new() -> Self {
        Self {
            liner_ok: true,
            splash_guard_ok: true,
            clearance_mm: 40.0,
            debris_buildup: false,
            aero_sealed: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.liner_ok && self.splash_guard_ok && !self.debris_buildup
    }

    pub fn clearance_ok(&self) -> bool {
        self.clearance_mm > 20.0
    }

    pub fn needs_cleaning(&self) -> bool {
        self.debris_buildup
    }

    pub fn needs_repair(&self) -> bool {
        !self.liner_ok || !self.splash_guard_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.liner_ok {
            return 30.0;
        }
        if !self.splash_guard_ok {
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
        let w = WheelWell::new();
        assert!(w.all_ok());
    }

    #[test]
    fn test_clearance() {
        let w = WheelWell::new();
        assert!(w.clearance_ok());
    }

    #[test]
    fn test_no_cleaning() {
        let w = WheelWell::new();
        assert!(!w.needs_cleaning());
    }

    #[test]
    fn test_no_repair() {
        let w = WheelWell::new();
        assert!(!w.needs_repair());
    }

    #[test]
    fn test_debris() {
        let mut w = WheelWell::new();
        w.debris_buildup = true;
        assert!(w.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let w = WheelWell::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
