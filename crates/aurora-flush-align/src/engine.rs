/// Flush alignment: panel flushness, step measurement, surface continuity
/// Phase 394

#[derive(Debug, Clone)]
pub struct FlushAlign {
    pub step_mm: f64,
    pub max_step_mm: f64,
    pub flush: bool,
    pub adjusted: bool,
    pub panel_pair: u8,
}

impl Default for FlushAlign {
    fn default() -> Self {
        Self::new()
    }
}

impl FlushAlign {
    pub fn new() -> Self {
        Self {
            step_mm: 0.3,
            max_step_mm: 1.0,
            flush: true,
            adjusted: true,
            panel_pair: 8,
        }
    }

    pub fn in_spec(&self) -> bool {
        self.step_mm <= self.max_step_mm
    }

    pub fn premium_fit(&self) -> bool {
        self.step_mm < 0.5
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.in_spec()
    }

    pub fn step_pct(&self) -> f64 {
        if self.max_step_mm <= 0.0 {
            return 0.0;
        }
        (self.step_mm / self.max_step_mm * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.in_spec() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_spec() {
        let f = FlushAlign::new();
        assert!(f.in_spec());
    }

    #[test]
    fn test_premium() {
        let f = FlushAlign::new();
        assert!(f.premium_fit());
    }

    #[test]
    fn test_no_adjust() {
        let f = FlushAlign::new();
        assert!(!f.needs_adjustment());
    }

    #[test]
    fn test_step_pct() {
        let f = FlushAlign::new();
        assert!(f.step_pct() < 40.0);
    }

    #[test]
    fn test_out_spec() {
        let mut f = FlushAlign::new();
        f.step_mm = 2.0;
        assert!(f.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let f = FlushAlign::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
