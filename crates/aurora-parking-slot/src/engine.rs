/// Parking slot detection: scan, classify, measure, guide, confirm
/// Phase 1118

#[derive(Debug, Clone)]
pub struct ParkingSlot {
    pub scan_ok: bool,
    pub classify_ok: bool,
    pub measure_ok: bool,
    pub guide_ok: bool,
    pub confirm_ok: bool,
}

impl Default for ParkingSlot {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkingSlot {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            classify_ok: true,
            measure_ok: true,
            guide_ok: true,
            confirm_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.scan_ok && self.classify_ok && self.measure_ok
    }

    pub fn assistance_ok(&self) -> bool {
        self.guide_ok && self.confirm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.assistance_ok()
    }

    pub fn needs_rescan(&self) -> bool {
        !self.scan_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = ParkingSlot::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_assistance() {
        let c = ParkingSlot::new();
        assert!(c.assistance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ParkingSlot::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rescan() {
        let c = ParkingSlot::new();
        assert!(!c.needs_rescan());
    }

    #[test]
    fn test_scan() {
        let mut c = ParkingSlot::new();
        c.scan_ok = false;
        assert!(c.needs_rescan());
    }

    #[test]
    fn test_health() {
        let c = ParkingSlot::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
