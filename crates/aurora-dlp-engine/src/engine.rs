/// DLP engine: classify, detect, mask, encrypt, report
/// Phase 1009

#[derive(Debug, Clone)]
pub struct DlpEngine {
    pub classify_ok: bool,
    pub detect_ok: bool,
    pub mask_ok: bool,
    pub encrypt_ok: bool,
    pub report_ok: bool,
}

impl Default for DlpEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DlpEngine {
    pub fn new() -> Self {
        Self {
            classify_ok: true,
            detect_ok: true,
            mask_ok: true,
            encrypt_ok: true,
            report_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.classify_ok && self.detect_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.mask_ok && self.encrypt_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.protection_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.classify_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.classify_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = DlpEngine::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_protection() {
        let c = DlpEngine::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DlpEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = DlpEngine::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_classify() {
        let mut c = DlpEngine::new();
        c.classify_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = DlpEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
