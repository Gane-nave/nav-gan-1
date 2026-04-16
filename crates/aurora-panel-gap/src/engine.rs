/// Panel gap: gap measurement, consistency, alignment tolerance
/// Phase 393

#[derive(Debug, Clone)]
pub struct PanelGap {
    pub gap_mm: f64,
    pub target_mm: f64,
    pub tolerance_mm: f64,
    pub consistent: bool,
    pub panel_count: u8,
}

impl Default for PanelGap {
    fn default() -> Self {
        Self::new()
    }
}

impl PanelGap {
    pub fn new() -> Self {
        Self {
            gap_mm: 4.0,
            target_mm: 4.0,
            tolerance_mm: 0.5,
            consistent: true,
            panel_count: 12,
        }
    }

    pub fn in_spec(&self) -> bool {
        (self.gap_mm - self.target_mm).abs() <= self.tolerance_mm
    }

    pub fn premium_fit(&self) -> bool {
        (self.gap_mm - self.target_mm).abs() < 0.3
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.in_spec()
    }

    pub fn deviation_mm(&self) -> f64 {
        (self.gap_mm - self.target_mm).abs()
    }

    pub fn health_score(&self) -> f64 {
        if !self.in_spec() {
            return 30.0;
        }
        if !self.consistent {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_spec() {
        let p = PanelGap::new();
        assert!(p.in_spec());
    }

    #[test]
    fn test_premium() {
        let p = PanelGap::new();
        assert!(p.premium_fit());
    }

    #[test]
    fn test_no_adjust() {
        let p = PanelGap::new();
        assert!(!p.needs_adjustment());
    }

    #[test]
    fn test_deviation() {
        let p = PanelGap::new();
        assert!(p.deviation_mm() < 0.1);
    }

    #[test]
    fn test_out_spec() {
        let mut p = PanelGap::new();
        p.gap_mm = 6.0;
        assert!(p.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let p = PanelGap::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
