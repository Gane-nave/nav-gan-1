/// Rocker panel: structure, coating, guard, drain
/// Phase 788

#[derive(Debug, Clone)]
pub struct RockerPanel {
    pub structure_ok: bool,
    pub coating_ok: bool,
    pub guard_ok: bool,
    pub drain_ok: bool,
    pub weld_ok: bool,
}

impl Default for RockerPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RockerPanel {
    pub fn new() -> Self {
        Self {
            structure_ok: true,
            coating_ok: true,
            guard_ok: true,
            drain_ok: true,
            weld_ok: true,
        }
    }

    pub fn body_ok(&self) -> bool {
        self.structure_ok && self.weld_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.coating_ok && self.guard_ok && self.drain_ok
    }

    pub fn all_ok(&self) -> bool {
        self.body_ok() && self.protection_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.structure_ok || !self.weld_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.structure_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body() {
        let c = RockerPanel::new();
        assert!(c.body_ok());
    }

    #[test]
    fn test_protection() {
        let c = RockerPanel::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RockerPanel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = RockerPanel::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_structure() {
        let mut c = RockerPanel::new();
        c.structure_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = RockerPanel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
