/// Blind spot monitoring: radar, indicator, cross traffic
/// Phase 737

#[derive(Debug, Clone)]
pub struct BlindSpot {
    pub radar_ok: bool,
    pub indicator_ok: bool,
    pub cross_traffic_ok: bool,
    pub range_ok: bool,
    pub calibrated: bool,
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
            indicator_ok: true,
            cross_traffic_ok: true,
            range_ok: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.radar_ok && self.range_ok && self.calibrated
    }

    pub fn alert_ok(&self) -> bool {
        self.indicator_ok && self.cross_traffic_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.alert_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.radar_ok
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
    fn test_alert() {
        let c = BlindSpot::new();
        assert!(c.alert_ok());
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
    fn test_cal() {
        let mut c = BlindSpot::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = BlindSpot::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
