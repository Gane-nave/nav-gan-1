/// Sign reader: OCR, speed limit, warning, regulatory
/// Phase 932

#[derive(Debug, Clone)]
pub struct SignRead {
    pub ocr_ok: bool,
    pub speed_ok: bool,
    pub warning_ok: bool,
    pub regulatory_ok: bool,
    pub camera_ok: bool,
}

impl Default for SignRead {
    fn default() -> Self {
        Self::new()
    }
}

impl SignRead {
    pub fn new() -> Self {
        Self {
            ocr_ok: true,
            speed_ok: true,
            warning_ok: true,
            regulatory_ok: true,
            camera_ok: true,
        }
    }

    pub fn recognition_ok(&self) -> bool {
        self.ocr_ok && self.camera_ok
    }

    pub fn classification_ok(&self) -> bool {
        self.speed_ok && self.warning_ok && self.regulatory_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recognition_ok() && self.classification_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.camera_ok || !self.ocr_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognition() {
        let c = SignRead::new();
        assert!(c.recognition_ok());
    }

    #[test]
    fn test_classification() {
        let c = SignRead::new();
        assert!(c.classification_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SignRead::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = SignRead::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_camera() {
        let mut c = SignRead::new();
        c.camera_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = SignRead::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
