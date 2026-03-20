/// Gear shifter: selector, indicator, interlock, cable
/// Phase 752

#[derive(Debug, Clone)]
pub struct GearShifter {
    pub selector_ok: bool,
    pub indicator_ok: bool,
    pub interlock_ok: bool,
    pub cable_ok: bool,
    pub position_ok: bool,
}

impl Default for GearShifter {
    fn default() -> Self {
        Self::new()
    }
}

impl GearShifter {
    pub fn new() -> Self {
        Self {
            selector_ok: true,
            indicator_ok: true,
            interlock_ok: true,
            cable_ok: true,
            position_ok: true,
        }
    }

    pub fn mechanism_ok(&self) -> bool {
        self.selector_ok && self.cable_ok && self.position_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.interlock_ok && self.indicator_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanism_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.selector_ok || !self.interlock_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.interlock_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanism() {
        let c = GearShifter::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_safety() {
        let c = GearShifter::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GearShifter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = GearShifter::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_selector() {
        let mut c = GearShifter::new();
        c.selector_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = GearShifter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
