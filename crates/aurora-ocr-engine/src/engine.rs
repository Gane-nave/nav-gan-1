/// OCR engine: preprocess, segment, recognize, correct, layout
/// Phase 1026

#[derive(Debug, Clone)]
pub struct OcrEngine {
    pub preprocess_ok: bool,
    pub segment_ok: bool,
    pub recognize_ok: bool,
    pub correct_ok: bool,
    pub layout_ok: bool,
}

impl Default for OcrEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl OcrEngine {
    pub fn new() -> Self {
        Self {
            preprocess_ok: true,
            segment_ok: true,
            recognize_ok: true,
            correct_ok: true,
            layout_ok: true,
        }
    }

    pub fn extraction_ok(&self) -> bool {
        self.preprocess_ok && self.segment_ok && self.recognize_ok
    }

    pub fn refinement_ok(&self) -> bool {
        self.correct_ok && self.layout_ok
    }

    pub fn all_ok(&self) -> bool {
        self.extraction_ok() && self.refinement_ok()
    }

    pub fn needs_model(&self) -> bool {
        !self.recognize_ok || !self.segment_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.preprocess_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction() {
        let c = OcrEngine::new();
        assert!(c.extraction_ok());
    }

    #[test]
    fn test_refinement() {
        let c = OcrEngine::new();
        assert!(c.refinement_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OcrEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_model() {
        let c = OcrEngine::new();
        assert!(!c.needs_model());
    }

    #[test]
    fn test_recognize() {
        let mut c = OcrEngine::new();
        c.recognize_ok = false;
        assert!(c.needs_model());
    }

    #[test]
    fn test_health() {
        let c = OcrEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
