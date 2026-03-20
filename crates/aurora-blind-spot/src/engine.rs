/// Blind spot monitoring: radar detection, mirror indicator, audible alert
/// Phase 265

#[derive(Debug, Clone)]
pub struct BlindSpotMonitor {
    pub active: bool,
    pub left_detected: bool,
    pub right_detected: bool,
    pub left_radar_ok: bool,
    pub right_radar_ok: bool,
    pub alert_volume: u8,
}

impl Default for BlindSpotMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl BlindSpotMonitor {
    pub fn new() -> Self {
        Self {
            active: true,
            left_detected: false,
            right_detected: false,
            left_radar_ok: true,
            right_radar_ok: true,
            alert_volume: 5,
        }
    }

    pub fn any_detected(&self) -> bool {
        self.left_detected || self.right_detected
    }

    pub fn all_clear(&self) -> bool {
        !self.left_detected && !self.right_detected
    }

    pub fn sensors_ok(&self) -> bool {
        self.left_radar_ok && self.right_radar_ok
    }

    pub fn safe_to_change_left(&self) -> bool {
        !self.left_detected
    }

    pub fn safe_to_change_right(&self) -> bool {
        !self.right_detected
    }

    pub fn health_score(&self) -> f64 {
        if !self.left_radar_ok && !self.right_radar_ok {
            return 0.0;
        }
        if !self.sensors_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_detected() {
        let b = BlindSpotMonitor::new();
        assert!(!b.any_detected());
    }

    #[test]
    fn test_all_clear() {
        let b = BlindSpotMonitor::new();
        assert!(b.all_clear());
    }

    #[test]
    fn test_sensors_ok() {
        let b = BlindSpotMonitor::new();
        assert!(b.sensors_ok());
    }

    #[test]
    fn test_safe_left() {
        let b = BlindSpotMonitor::new();
        assert!(b.safe_to_change_left());
    }

    #[test]
    fn test_vehicle_left() {
        let mut b = BlindSpotMonitor::new();
        b.left_detected = true;
        assert!(!b.safe_to_change_left());
    }

    #[test]
    fn test_health() {
        let b = BlindSpotMonitor::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
