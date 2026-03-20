/// Floor pan: structure, drain, insulation, reinforcement
/// Phase 793

#[derive(Debug, Clone)]
pub struct FloorPan {
    pub structure_ok: bool,
    pub drain_ok: bool,
    pub insulation_ok: bool,
    pub reinforcement_ok: bool,
    pub coating_ok: bool,
}

impl Default for FloorPan {
    fn default() -> Self {
        Self::new()
    }
}

impl FloorPan {
    pub fn new() -> Self {
        Self {
            structure_ok: true,
            drain_ok: true,
            insulation_ok: true,
            reinforcement_ok: true,
            coating_ok: true,
        }
    }

    pub fn body_ok(&self) -> bool {
        self.structure_ok && self.reinforcement_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.drain_ok && self.insulation_ok && self.coating_ok
    }

    pub fn all_ok(&self) -> bool {
        self.body_ok() && self.protection_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.structure_ok || !self.coating_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.structure_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body() {
        let c = FloorPan::new();
        assert!(c.body_ok());
    }

    #[test]
    fn test_protection() {
        let c = FloorPan::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FloorPan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = FloorPan::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_structure() {
        let mut c = FloorPan::new();
        c.structure_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = FloorPan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
