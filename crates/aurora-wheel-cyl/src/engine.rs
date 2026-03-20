/// Wheel cylinder: bore, piston, cup, spring
/// Phase 665

#[derive(Debug, Clone)]
pub struct WheelCylinder {
    pub bore_ok: bool,
    pub piston_ok: bool,
    pub cup_ok: bool,
    pub spring_ok: bool,
    pub leak_free: bool,
}

impl Default for WheelCylinder {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelCylinder {
    pub fn new() -> Self {
        Self {
            bore_ok: true,
            piston_ok: true,
            cup_ok: true,
            spring_ok: true,
            leak_free: true,
        }
    }

    pub fn hydraulic_ok(&self) -> bool {
        self.bore_ok && self.piston_ok && self.cup_ok
    }

    pub fn mechanical_ok(&self) -> bool {
        self.spring_ok && self.leak_free
    }

    pub fn all_ok(&self) -> bool {
        self.hydraulic_ok() && self.mechanical_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.cup_ok || !self.bore_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bore_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydraulic() {
        let c = WheelCylinder::new();
        assert!(c.hydraulic_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = WheelCylinder::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelCylinder::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = WheelCylinder::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_cup() {
        let mut c = WheelCylinder::new();
        c.cup_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = WheelCylinder::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
