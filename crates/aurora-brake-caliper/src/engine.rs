/// Brake caliper: piston, seal, slide pin, bleeder
/// Phase 656

#[derive(Debug, Clone)]
pub struct BrakeCaliper {
    pub piston_ok: bool,
    pub seal_ok: bool,
    pub slide_pin_ok: bool,
    pub bleeder_ok: bool,
    pub leak_free: bool,
}

impl Default for BrakeCaliper {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeCaliper {
    pub fn new() -> Self {
        Self {
            piston_ok: true,
            seal_ok: true,
            slide_pin_ok: true,
            bleeder_ok: true,
            leak_free: true,
        }
    }

    pub fn hydraulic_ok(&self) -> bool {
        self.piston_ok && self.seal_ok && self.leak_free
    }

    pub fn mechanical_ok(&self) -> bool {
        self.slide_pin_ok && self.bleeder_ok
    }

    pub fn all_ok(&self) -> bool {
        self.hydraulic_ok() && self.mechanical_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.piston_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.piston_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydraulic() {
        let c = BrakeCaliper::new();
        assert!(c.hydraulic_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = BrakeCaliper::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeCaliper::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = BrakeCaliper::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_piston() {
        let mut c = BrakeCaliper::new();
        c.piston_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = BrakeCaliper::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
