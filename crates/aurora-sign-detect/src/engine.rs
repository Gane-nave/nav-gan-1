/// Sign detection: detect, classify, read, validate, alert
/// Phase 1115

#[derive(Debug, Clone)]
pub struct SignDetect {
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub read_ok: bool,
    pub validate_ok: bool,
    pub alert_ok: bool,
}

impl Default for SignDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl SignDetect {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            classify_ok: true,
            read_ok: true,
            validate_ok: true,
            alert_ok: true,
        }
    }

    pub fn recognition_ok(&self) -> bool {
        self.detect_ok && self.classify_ok && self.read_ok
    }

    pub fn response_ok(&self) -> bool {
        self.validate_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recognition_ok() && self.response_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.detect_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognition() {
        let c = SignDetect::new();
        assert!(c.recognition_ok());
    }

    #[test]
    fn test_response() {
        let c = SignDetect::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SignDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = SignDetect::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_detect() {
        let mut c = SignDetect::new();
        c.detect_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = SignDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
