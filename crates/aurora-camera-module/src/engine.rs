/// Camera module: CMOS sensor, lens, heater, washer
/// Phase 700

#[derive(Debug, Clone)]
pub struct CameraModule {
    pub cmos_ok: bool,
    pub lens_ok: bool,
    pub heater_ok: bool,
    pub washer_ok: bool,
    pub calibrated: bool,
}

impl Default for CameraModule {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraModule {
    pub fn new() -> Self {
        Self {
            cmos_ok: true,
            lens_ok: true,
            heater_ok: true,
            washer_ok: true,
            calibrated: true,
        }
    }

    pub fn imaging_ok(&self) -> bool {
        self.cmos_ok && self.lens_ok
    }

    pub fn maintenance_ok(&self) -> bool {
        self.heater_ok && self.washer_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.imaging_ok() && self.maintenance_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.cmos_ok || !self.lens_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cmos_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imaging() {
        let c = CameraModule::new();
        assert!(c.imaging_ok());
    }

    #[test]
    fn test_maintenance() {
        let c = CameraModule::new();
        assert!(c.maintenance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CameraModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CameraModule::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_cmos() {
        let mut c = CameraModule::new();
        c.cmos_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CameraModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
