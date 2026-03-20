/// Blind spot: radar, alert, cross-traffic, merge assist
/// Phase 928

#[derive(Debug, Clone)]
pub struct BlindSpot {
    pub radar_ok: bool,
    pub alert_ok: bool,
    pub cross_ok: bool,
    pub merge_ok: bool,
    pub display_ok: bool,
}

impl Default for BlindSpot {
    fn default() -> Self {
        Self::new()
    }
}

impl BlindSpot {
    pub fn new() -> Self {
        Self {
            radar_ok: true,
            alert_ok: true,
            cross_ok: true,
            merge_ok: true,
            display_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.radar_ok && self.cross_ok
    }

    pub fn warning_ok(&self) -> bool {
        self.alert_ok && self.merge_ok && self.display_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.warning_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.radar_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.radar_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = BlindSpot::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_warning() {
        let c = BlindSpot::new();
        assert!(c.warning_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BlindSpot::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = BlindSpot::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_radar() {
        let mut c = BlindSpot::new();
        c.radar_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = BlindSpot::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
