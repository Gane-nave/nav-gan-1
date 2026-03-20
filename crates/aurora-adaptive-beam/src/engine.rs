/// Adaptive beam: matrix LED, glare-free high beam, oncoming detection
/// Phase 249

#[derive(Debug, Clone)]
pub struct AdaptiveBeam {
    pub segments_total: u8,
    pub segments_active: u8,
    pub oncoming_detected: bool,
    pub preceding_detected: bool,
    pub auto_active: bool,
    pub camera_ok: bool,
}

impl Default for AdaptiveBeam {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveBeam {
    pub fn new() -> Self {
        Self {
            segments_total: 16,
            segments_active: 16,
            oncoming_detected: false,
            preceding_detected: false,
            auto_active: true,
            camera_ok: true,
        }
    }

    pub fn full_beam(&self) -> bool {
        self.segments_active == self.segments_total
    }

    pub fn partial_beam(&self) -> bool {
        self.segments_active > 0 && self.segments_active < self.segments_total
    }

    pub fn should_dim(&self) -> bool {
        self.oncoming_detected || self.preceding_detected
    }

    pub fn active_pct(&self) -> f64 {
        if self.segments_total == 0 {
            return 0.0;
        }
        self.segments_active as f64 / self.segments_total as f64 * 100.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_beam() {
        let a = AdaptiveBeam::new();
        assert!(a.full_beam());
    }

    #[test]
    fn test_not_partial() {
        let a = AdaptiveBeam::new();
        assert!(!a.partial_beam());
    }

    #[test]
    fn test_no_dim() {
        let a = AdaptiveBeam::new();
        assert!(!a.should_dim());
    }

    #[test]
    fn test_active_pct() {
        let a = AdaptiveBeam::new();
        assert!((a.active_pct() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_oncoming() {
        let mut a = AdaptiveBeam::new();
        a.oncoming_detected = true;
        assert!(a.should_dim());
    }

    #[test]
    fn test_health() {
        let a = AdaptiveBeam::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
