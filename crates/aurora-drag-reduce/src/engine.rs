/// Drag reduction system: active aero, DRS, movable elements
/// Phase 349

#[derive(Debug, Clone)]
pub struct DragReduce {
    pub active: bool,
    pub flap_open: bool,
    pub reduction_pct: f64,
    pub speed_threshold_kmh: f64,
    pub current_speed_kmh: f64,
}

impl Default for DragReduce {
    fn default() -> Self {
        Self::new()
    }
}

impl DragReduce {
    pub fn new() -> Self {
        Self {
            active: false,
            flap_open: false,
            reduction_pct: 0.0,
            speed_threshold_kmh: 80.0,
            current_speed_kmh: 60.0,
        }
    }

    pub fn can_activate(&self) -> bool {
        self.current_speed_kmh >= self.speed_threshold_kmh
    }

    pub fn is_reducing(&self) -> bool {
        self.active && self.flap_open
    }

    pub fn savings_pct(&self) -> f64 {
        if self.is_reducing() {
            self.reduction_pct
        } else {
            0.0
        }
    }

    pub fn effective(&self) -> bool {
        self.reduction_pct > 5.0
    }

    pub fn health_score(&self) -> f64 {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cannot_activate() {
        let d = DragReduce::new();
        assert!(!d.can_activate());
    }

    #[test]
    fn test_not_reducing() {
        let d = DragReduce::new();
        assert!(!d.is_reducing());
    }

    #[test]
    fn test_no_savings() {
        let d = DragReduce::new();
        assert!(d.savings_pct() < 0.1);
    }

    #[test]
    fn test_not_effective() {
        let d = DragReduce::new();
        assert!(!d.effective());
    }

    #[test]
    fn test_can_activate() {
        let mut d = DragReduce::new();
        d.current_speed_kmh = 100.0;
        assert!(d.can_activate());
    }

    #[test]
    fn test_health() {
        let d = DragReduce::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
