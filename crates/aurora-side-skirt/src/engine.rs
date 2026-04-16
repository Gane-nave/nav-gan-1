/// Side skirt: rocker panel aero, ground effect seal, side airflow
/// Phase 356

#[derive(Debug, Clone)]
pub struct SideSkirt {
    pub length_mm: f64,
    pub ground_gap_mm: f64,
    pub intact_left: bool,
    pub intact_right: bool,
    pub aero_optimized: bool,
}

impl Default for SideSkirt {
    fn default() -> Self {
        Self::new()
    }
}

impl SideSkirt {
    pub fn new() -> Self {
        Self {
            length_mm: 2000.0,
            ground_gap_mm: 80.0,
            intact_left: true,
            intact_right: true,
            aero_optimized: true,
        }
    }

    pub fn both_intact(&self) -> bool {
        self.intact_left && self.intact_right
    }

    pub fn gap_ok(&self) -> bool {
        self.ground_gap_mm > 40.0 && self.ground_gap_mm < 150.0
    }

    pub fn effective(&self) -> bool {
        self.both_intact() && self.gap_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.intact_left || !self.intact_right
    }

    pub fn health_score(&self) -> f64 {
        if !self.both_intact() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intact() {
        let s = SideSkirt::new();
        assert!(s.both_intact());
    }

    #[test]
    fn test_gap() {
        let s = SideSkirt::new();
        assert!(s.gap_ok());
    }

    #[test]
    fn test_effective() {
        let s = SideSkirt::new();
        assert!(s.effective());
    }

    #[test]
    fn test_no_repair() {
        let s = SideSkirt::new();
        assert!(!s.needs_repair());
    }

    #[test]
    fn test_damaged() {
        let mut s = SideSkirt::new();
        s.intact_left = false;
        assert!(s.needs_repair());
    }

    #[test]
    fn test_health() {
        let s = SideSkirt::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
