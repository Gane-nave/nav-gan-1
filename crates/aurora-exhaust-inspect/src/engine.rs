/// Exhaust inspection: leak, hanger, catalyst, muffler
/// Phase 828

#[derive(Debug, Clone)]
pub struct ExhaustInspect {
    pub leak_free: bool,
    pub hanger_ok: bool,
    pub catalyst_ok: bool,
    pub muffler_ok: bool,
    pub pipe_ok: bool,
}

impl Default for ExhaustInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustInspect {
    pub fn new() -> Self {
        Self {
            leak_free: true,
            hanger_ok: true,
            catalyst_ok: true,
            muffler_ok: true,
            pipe_ok: true,
        }
    }

    pub fn integrity_ok(&self) -> bool {
        self.leak_free && self.pipe_ok
    }

    pub fn components_ok(&self) -> bool {
        self.hanger_ok && self.catalyst_ok && self.muffler_ok
    }

    pub fn all_ok(&self) -> bool {
        self.integrity_ok() && self.components_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.leak_free || !self.catalyst_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrity() {
        let c = ExhaustInspect::new();
        assert!(c.integrity_ok());
    }

    #[test]
    fn test_components() {
        let c = ExhaustInspect::new();
        assert!(c.components_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ExhaustInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = ExhaustInspect::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_leak() {
        let mut c = ExhaustInspect::new();
        c.leak_free = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = ExhaustInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
