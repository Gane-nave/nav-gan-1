/// LiDAR processing: scan, filter, cluster, classify, fuse
/// Phase 1105

#[derive(Debug, Clone)]
pub struct LidarProc {
    pub scan_ok: bool,
    pub filter_ok: bool,
    pub cluster_ok: bool,
    pub classify_ok: bool,
    pub fuse_ok: bool,
}

impl Default for LidarProc {
    fn default() -> Self {
        Self::new()
    }
}

impl LidarProc {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            filter_ok: true,
            cluster_ok: true,
            classify_ok: true,
            fuse_ok: true,
        }
    }

    pub fn processing_ok(&self) -> bool {
        self.scan_ok && self.filter_ok && self.cluster_ok
    }

    pub fn understanding_ok(&self) -> bool {
        self.classify_ok && self.fuse_ok
    }

    pub fn all_ok(&self) -> bool {
        self.processing_ok() && self.understanding_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.scan_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing() {
        let c = LidarProc::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_understanding() {
        let c = LidarProc::new();
        assert!(c.understanding_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LidarProc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = LidarProc::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_scan() {
        let mut c = LidarProc::new();
        c.scan_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = LidarProc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
