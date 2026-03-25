/// Cyclist detection: lidar, classifier, tracking, warning
/// Phase 839

#[derive(Debug, Clone)]
pub struct CyclistDet {
    pub lidar_ok: bool,
    pub classifier_ok: bool,
    pub tracking_ok: bool,
    pub warning_ok: bool,
    pub brake_ok: bool,
}

impl Default for CyclistDet {
    fn default() -> Self {
        Self::new()
    }
}

impl CyclistDet {
    pub fn new() -> Self {
        Self {
            lidar_ok: true,
            classifier_ok: true,
            tracking_ok: true,
            warning_ok: true,
            brake_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.lidar_ok && self.classifier_ok && self.tracking_ok
    }

    pub fn response_ok(&self) -> bool {
        self.warning_ok && self.brake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.response_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.classifier_ok || !self.lidar_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lidar_ok {
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
        let c = CyclistDet::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_response() {
        let c = CyclistDet::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CyclistDet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = CyclistDet::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_lidar() {
        let mut c = CyclistDet::new();
        c.lidar_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = CyclistDet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
