/// Striker plate: door striker, alignment, wear, adjustment
/// Phase 411

#[derive(Debug, Clone)]
pub struct StrikerPlate {
    pub aligned: bool,
    pub wear_mm: f64,
    pub max_wear_mm: f64,
    pub bolts_ok: bool,
    pub lubricated: bool,
}

impl Default for StrikerPlate {
    fn default() -> Self {
        Self::new()
    }
}

impl StrikerPlate {
    pub fn new() -> Self {
        Self {
            aligned: true,
            wear_mm: 0.1,
            max_wear_mm: 1.0,
            bolts_ok: true,
            lubricated: true,
        }
    }

    pub fn wear_ok(&self) -> bool {
        self.wear_mm < self.max_wear_mm
    }

    pub fn all_ok(&self) -> bool {
        self.aligned && self.wear_ok() && self.bolts_ok
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.aligned
    }

    pub fn needs_replacement(&self) -> bool {
        !self.wear_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.wear_ok() {
            return 20.0;
        }
        if !self.aligned {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wear() {
        let s = StrikerPlate::new();
        assert!(s.wear_ok());
    }

    #[test]
    fn test_all_ok() {
        let s = StrikerPlate::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let s = StrikerPlate::new();
        assert!(!s.needs_adjustment());
    }

    #[test]
    fn test_no_replace() {
        let s = StrikerPlate::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_misaligned() {
        let mut s = StrikerPlate::new();
        s.aligned = false;
        assert!(s.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let s = StrikerPlate::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
