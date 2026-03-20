/// Lumbar adjustment: support level, position, pneumatic/electric control
/// Phase 257

#[derive(Debug, Clone)]
pub struct LumbarAdjustment {
    pub support_level: u8,
    pub max_level: u8,
    pub position_mm: f64,
    pub electric: bool,
    pub motor_ok: bool,
}

impl Default for LumbarAdjustment {
    fn default() -> Self {
        Self::new()
    }
}

impl LumbarAdjustment {
    pub fn new() -> Self {
        Self {
            support_level: 2,
            max_level: 5,
            position_mm: 10.0,
            electric: true,
            motor_ok: true,
        }
    }

    pub fn support_pct(&self) -> f64 {
        if self.max_level == 0 {
            return 0.0;
        }
        self.support_level as f64 / self.max_level as f64 * 100.0
    }

    pub fn can_adjust(&self) -> bool {
        self.electric && self.motor_ok
    }

    pub fn at_max(&self) -> bool {
        self.support_level >= self.max_level
    }

    pub fn at_min(&self) -> bool {
        self.support_level == 0
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_support_pct() {
        let l = LumbarAdjustment::new();
        assert!((l.support_pct() - 40.0).abs() < 0.1);
    }

    #[test]
    fn test_can_adjust() {
        let l = LumbarAdjustment::new();
        assert!(l.can_adjust());
    }

    #[test]
    fn test_not_at_max() {
        let l = LumbarAdjustment::new();
        assert!(!l.at_max());
    }

    #[test]
    fn test_not_at_min() {
        let l = LumbarAdjustment::new();
        assert!(!l.at_min());
    }

    #[test]
    fn test_motor_fail() {
        let mut l = LumbarAdjustment::new();
        l.motor_ok = false;
        assert!(!l.can_adjust());
    }

    #[test]
    fn test_health() {
        let l = LumbarAdjustment::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
