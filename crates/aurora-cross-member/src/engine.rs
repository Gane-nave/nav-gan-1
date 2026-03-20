/// Cross member: front, rear, floor, bracket
/// Phase 796

#[derive(Debug, Clone)]
pub struct CrossMember {
    pub front_ok: bool,
    pub rear_ok: bool,
    pub floor_ok: bool,
    pub bracket_ok: bool,
    pub weld_ok: bool,
}

impl Default for CrossMember {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossMember {
    pub fn new() -> Self {
        Self {
            front_ok: true,
            rear_ok: true,
            floor_ok: true,
            bracket_ok: true,
            weld_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.front_ok && self.rear_ok && self.floor_ok
    }

    pub fn mounting_ok(&self) -> bool {
        self.bracket_ok && self.weld_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.mounting_ok()
    }

    pub fn needs_inspection(&self) -> bool {
        !self.front_ok || !self.weld_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.weld_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = CrossMember::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_mounting() {
        let c = CrossMember::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CrossMember::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_inspect() {
        let c = CrossMember::new();
        assert!(!c.needs_inspection());
    }

    #[test]
    fn test_weld() {
        let mut c = CrossMember::new();
        c.weld_ok = false;
        assert!(c.needs_inspection());
    }

    #[test]
    fn test_health() {
        let c = CrossMember::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
