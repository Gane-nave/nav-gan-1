/// Shift fork: gear engagement, synchronizer, detent spring
/// Phase 466

#[derive(Debug, Clone)]
pub struct ShiftFork {
    pub engagement_ok: bool,
    pub wear_mm: f64,
    pub max_wear_mm: f64,
    pub pad_ok: bool,
    pub spring_ok: bool,
}

impl Default for ShiftFork {
    fn default() -> Self {
        Self::new()
    }
}

impl ShiftFork {
    pub fn new() -> Self {
        Self {
            engagement_ok: true,
            wear_mm: 0.1,
            max_wear_mm: 0.5,
            pad_ok: true,
            spring_ok: true,
        }
    }

    pub fn wear_ok(&self) -> bool {
        self.wear_mm < self.max_wear_mm
    }

    pub fn all_ok(&self) -> bool {
        self.engagement_ok && self.wear_ok() && self.pad_ok && self.spring_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.wear_ok() || !self.pad_ok
    }

    pub fn shift_quality_ok(&self) -> bool {
        self.engagement_ok && self.spring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wear_ok() {
            return 10.0;
        }
        if !self.pad_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wear() {
        let s = ShiftFork::new();
        assert!(s.wear_ok());
    }

    #[test]
    fn test_all_ok() {
        let s = ShiftFork::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let s = ShiftFork::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_quality() {
        let s = ShiftFork::new();
        assert!(s.shift_quality_ok());
    }

    #[test]
    fn test_worn() {
        let mut s = ShiftFork::new();
        s.wear_mm = 0.8;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = ShiftFork::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
