/// FlexRay bus: sync, cycle, slot, guardian
/// Phase 729

#[derive(Debug, Clone)]
pub struct FlexRay {
    pub sync_ok: bool,
    pub cycle_ok: bool,
    pub slot_ok: bool,
    pub guardian_ok: bool,
    pub redundancy_ok: bool,
}

impl Default for FlexRay {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexRay {
    pub fn new() -> Self {
        Self {
            sync_ok: true,
            cycle_ok: true,
            slot_ok: true,
            guardian_ok: true,
            redundancy_ok: true,
        }
    }

    pub fn timing_ok(&self) -> bool {
        self.sync_ok && self.cycle_ok && self.slot_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.guardian_ok && self.redundancy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.timing_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.sync_ok || !self.guardian_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sync_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing() {
        let c = FlexRay::new();
        assert!(c.timing_ok());
    }

    #[test]
    fn test_safety() {
        let c = FlexRay::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FlexRay::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FlexRay::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_sync() {
        let mut c = FlexRay::new();
        c.sync_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FlexRay::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
