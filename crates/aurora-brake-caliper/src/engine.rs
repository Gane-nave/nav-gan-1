/// Brake caliper: piston, seal, slide pins
/// Phase 486

#[derive(Debug, Clone)]
pub struct BrakeCaliper {
    pub piston_ok: bool,
    pub seal_ok: bool,
    pub slide_pins_ok: bool,
    pub sticking: bool,
    pub leak_detected: bool,
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
            slide_pins_ok: true,
            sticking: false,
            leak_detected: false,
        }
    }

    pub fn functional(&self) -> bool {
        self.piston_ok && self.seal_ok && !self.sticking
    }

    pub fn slides_ok(&self) -> bool {
        self.slide_pins_ok && !self.sticking
    }

    pub fn all_ok(&self) -> bool {
        self.functional() && self.slides_ok() && !self.leak_detected
    }

    pub fn needs_rebuild(&self) -> bool {
        self.sticking || self.leak_detected
    }

    pub fn health_score(&self) -> f64 {
        if self.leak_detected { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional() {
        let c = BrakeCaliper::new();
        assert!(c.functional());
    }

    #[test]
    fn test_slides() {
        let c = BrakeCaliper::new();
        assert!(c.slides_ok());
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
    fn test_sticking() {
        let mut c = BrakeCaliper::new();
        c.sticking = true;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = BrakeCaliper::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
