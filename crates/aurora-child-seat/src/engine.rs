/// Child seat anchor: ISOFIX, top tether, latch, indicator
/// Phase 837

#[derive(Debug, Clone)]
pub struct ChildSeat {
    pub isofix_ok: bool,
    pub tether_ok: bool,
    pub latch_ok: bool,
    pub indicator_ok: bool,
    pub torque_ok: bool,
}

impl Default for ChildSeat {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildSeat {
    pub fn new() -> Self {
        Self {
            isofix_ok: true,
            tether_ok: true,
            latch_ok: true,
            indicator_ok: true,
            torque_ok: true,
        }
    }

    pub fn mounting_ok(&self) -> bool {
        self.isofix_ok && self.tether_ok && self.latch_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.indicator_ok && self.torque_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mounting_ok() && self.safety_ok()
    }

    pub fn needs_inspection(&self) -> bool {
        !self.isofix_ok || !self.latch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.isofix_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mounting() {
        let c = ChildSeat::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_safety() {
        let c = ChildSeat::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChildSeat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_inspect() {
        let c = ChildSeat::new();
        assert!(!c.needs_inspection());
    }

    #[test]
    fn test_isofix() {
        let mut c = ChildSeat::new();
        c.isofix_ok = false;
        assert!(c.needs_inspection());
    }

    #[test]
    fn test_health() {
        let c = ChildSeat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
