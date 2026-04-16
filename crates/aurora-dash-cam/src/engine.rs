/// dash cam: record, store, detect, upload, loop
/// Phase 1289

#[derive(Debug, Clone)]
pub struct DashCam {
    pub record_ok: bool,
    pub store_ok: bool,
    pub detect_ok: bool,
    pub upload_ok: bool,
    pub loop_ok: bool,
}

impl Default for DashCam {
    fn default() -> Self {
        Self::new()
    }
}

impl DashCam {
    pub fn new() -> Self {
        Self {
            record_ok: true,
            store_ok: true,
            detect_ok: true,
            upload_ok: true,
            loop_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.record_ok && self.store_ok && self.detect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.upload_ok && self.loop_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.record_ok || !self.store_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.record_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DashCam::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashCam::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashCam::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashCam::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashCam::new();
        c.record_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashCam::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
