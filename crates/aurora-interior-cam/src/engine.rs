/// interior cam: monitor, detect, classify, alert, store
/// Phase 1290

#[derive(Debug, Clone)]
pub struct InteriorCam {
    pub monitor_ok: bool,
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub alert_ok: bool,
    pub store_ok: bool,
}

impl Default for InteriorCam {
    fn default() -> Self {
        Self::new()
    }
}

impl InteriorCam {
    pub fn new() -> Self {
        Self {
            monitor_ok: true,
            detect_ok: true,
            classify_ok: true,
            alert_ok: true,
            store_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.monitor_ok && self.detect_ok && self.classify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.store_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.monitor_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.monitor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = InteriorCam::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InteriorCam::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InteriorCam::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InteriorCam::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InteriorCam::new();
        c.monitor_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InteriorCam::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
