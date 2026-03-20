/// LiDAR unit: laser, detector, mirror, processor
/// Phase 701

#[derive(Debug, Clone)]
pub struct LidarUnit {
    pub laser_ok: bool,
    pub detector_ok: bool,
    pub mirror_ok: bool,
    pub processor_ok: bool,
    pub calibrated: bool,
}

impl Default for LidarUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl LidarUnit {
    pub fn new() -> Self {
        Self {
            laser_ok: true,
            detector_ok: true,
            mirror_ok: true,
            processor_ok: true,
            calibrated: true,
        }
    }

    pub fn optics_ok(&self) -> bool {
        self.laser_ok && self.detector_ok && self.mirror_ok
    }

    pub fn compute_ok(&self) -> bool {
        self.processor_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.optics_ok() && self.compute_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.laser_ok || !self.processor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.laser_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optics() {
        let c = LidarUnit::new();
        assert!(c.optics_ok());
    }

    #[test]
    fn test_compute() {
        let c = LidarUnit::new();
        assert!(c.compute_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LidarUnit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = LidarUnit::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_laser() {
        let mut c = LidarUnit::new();
        c.laser_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = LidarUnit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
