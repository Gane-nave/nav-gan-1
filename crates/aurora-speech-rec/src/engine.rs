/// Speech recognition: audio, feature, decode, language, punct
/// Phase 1024

#[derive(Debug, Clone)]
pub struct SpeechRec {
    pub audio_ok: bool,
    pub feature_ok: bool,
    pub decode_ok: bool,
    pub language_ok: bool,
    pub punct_ok: bool,
}

impl Default for SpeechRec {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeechRec {
    pub fn new() -> Self {
        Self {
            audio_ok: true,
            feature_ok: true,
            decode_ok: true,
            language_ok: true,
            punct_ok: true,
        }
    }

    pub fn frontend_ok(&self) -> bool {
        self.audio_ok && self.feature_ok
    }

    pub fn backend_ok(&self) -> bool {
        self.decode_ok && self.language_ok && self.punct_ok
    }

    pub fn all_ok(&self) -> bool {
        self.frontend_ok() && self.backend_ok()
    }

    pub fn needs_model(&self) -> bool {
        !self.decode_ok || !self.language_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.audio_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontend() {
        let c = SpeechRec::new();
        assert!(c.frontend_ok());
    }

    #[test]
    fn test_backend() {
        let c = SpeechRec::new();
        assert!(c.backend_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpeechRec::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_model() {
        let c = SpeechRec::new();
        assert!(!c.needs_model());
    }

    #[test]
    fn test_decode() {
        let mut c = SpeechRec::new();
        c.decode_ok = false;
        assert!(c.needs_model());
    }

    #[test]
    fn test_health() {
        let c = SpeechRec::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
