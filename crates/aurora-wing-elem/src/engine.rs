/// Wing element: adjustable angle, multi-element, Gurney flap
/// Phase 353

#[derive(Debug, Clone)]
pub struct WingElement {
    pub angle_deg: f64,
    pub max_angle_deg: f64,
    pub elements: u8,
    pub gurney_flap: bool,
    pub motorized: bool,
}

impl Default for WingElement {
    fn default() -> Self {
        Self::new()
    }
}

impl WingElement {
    pub fn new() -> Self {
        Self {
            angle_deg: 8.0,
            max_angle_deg: 25.0,
            elements: 2,
            gurney_flap: false,
            motorized: false,
        }
    }

    pub fn angle_pct(&self) -> f64 {
        if self.max_angle_deg <= 0.0 {
            return 0.0;
        }
        (self.angle_deg / self.max_angle_deg * 100.0).clamp(0.0, 100.0)
    }

    pub fn high_downforce(&self) -> bool {
        self.angle_deg > 15.0
    }

    pub fn low_drag(&self) -> bool {
        self.angle_deg < 5.0
    }

    pub fn multi_element(&self) -> bool {
        self.elements > 1
    }

    pub fn health_score(&self) -> f64 {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_pct() {
        let w = WingElement::new();
        assert!(w.angle_pct() < 40.0);
    }

    #[test]
    fn test_not_high_df() {
        let w = WingElement::new();
        assert!(!w.high_downforce());
    }

    #[test]
    fn test_not_low_drag() {
        let w = WingElement::new();
        assert!(!w.low_drag());
    }

    #[test]
    fn test_multi() {
        let w = WingElement::new();
        assert!(w.multi_element());
    }

    #[test]
    fn test_high_df() {
        let mut w = WingElement::new();
        w.angle_deg = 20.0;
        assert!(w.high_downforce());
    }

    #[test]
    fn test_health() {
        let w = WingElement::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
