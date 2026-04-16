/// Free space detection: grid, project, classify, fuse, track
/// Phase 1117

#[derive(Debug, Clone)]
pub struct FreeSpace {
    pub grid_ok: bool,
    pub project_ok: bool,
    pub classify_ok: bool,
    pub fuse_ok: bool,
    pub track_ok: bool,
}

impl Default for FreeSpace {
    fn default() -> Self {
        Self::new()
    }
}

impl FreeSpace {
    pub fn new() -> Self {
        Self {
            grid_ok: true,
            project_ok: true,
            classify_ok: true,
            fuse_ok: true,
            track_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.grid_ok && self.project_ok && self.classify_ok
    }

    pub fn tracking_ok(&self) -> bool {
        self.fuse_ok && self.track_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.tracking_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.grid_ok || !self.project_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.grid_ok {
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
        let c = FreeSpace::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_tracking() {
        let c = FreeSpace::new();
        assert!(c.tracking_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FreeSpace::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = FreeSpace::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_grid() {
        let mut c = FreeSpace::new();
        c.grid_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = FreeSpace::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
