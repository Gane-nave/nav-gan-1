/// Body fit: overall fit and finish, dimensional accuracy, assembly quality
/// Phase 395

#[derive(Debug, Clone)]
pub struct BodyFit {
    pub overall_score: f64,
    pub gap_score: f64,
    pub flush_score: f64,
    pub visual_score: f64,
    pub defect_count: u32,
}

impl Default for BodyFit {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyFit {
    pub fn new() -> Self {
        Self {
            overall_score: 92.0,
            gap_score: 95.0,
            flush_score: 90.0,
            visual_score: 91.0,
            defect_count: 0,
        }
    }

    pub fn premium_quality(&self) -> bool {
        self.overall_score > 90.0 && self.defect_count == 0
    }

    pub fn acceptable(&self) -> bool {
        self.overall_score > 75.0
    }

    pub fn needs_rework(&self) -> bool {
        self.overall_score < 70.0 || self.defect_count > 3
    }

    pub fn composite_score(&self) -> f64 {
        (self.gap_score + self.flush_score + self.visual_score) / 3.0
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_rework() {
            return 20.0;
        }
        if !self.acceptable() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_premium() {
        let b = BodyFit::new();
        assert!(b.premium_quality());
    }

    #[test]
    fn test_acceptable() {
        let b = BodyFit::new();
        assert!(b.acceptable());
    }

    #[test]
    fn test_no_rework() {
        let b = BodyFit::new();
        assert!(!b.needs_rework());
    }

    #[test]
    fn test_composite() {
        let b = BodyFit::new();
        assert!(b.composite_score() > 90.0);
    }

    #[test]
    fn test_poor() {
        let mut b = BodyFit::new();
        b.overall_score = 60.0;
        assert!(b.needs_rework());
    }

    #[test]
    fn test_health() {
        let b = BodyFit::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
