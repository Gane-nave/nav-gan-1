/// auto park: scan, select, maneuver, correct, complete
/// Phase 1322

#[derive(Debug, Clone)]
pub struct AutoPark2 {
    pub scan_ok: bool,
    pub select_ok: bool,
    pub maneuver_ok: bool,
    pub correct_ok: bool,
    pub complete_ok: bool,
}

impl Default for AutoPark2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoPark2 {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            select_ok: true,
            maneuver_ok: true,
            correct_ok: true,
            complete_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.select_ok && self.maneuver_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.correct_ok && self.complete_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.select_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AutoPark2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AutoPark2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AutoPark2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AutoPark2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AutoPark2::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AutoPark2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
