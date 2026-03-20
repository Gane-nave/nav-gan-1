/// TTS engine: text, phoneme, prosody, vocoder, stream
/// Phase 1025

#[derive(Debug, Clone)]
pub struct TtsEngine {
    pub text_ok: bool,
    pub phoneme_ok: bool,
    pub prosody_ok: bool,
    pub vocoder_ok: bool,
    pub stream_ok: bool,
}

impl Default for TtsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TtsEngine {
    pub fn new() -> Self {
        Self {
            text_ok: true,
            phoneme_ok: true,
            prosody_ok: true,
            vocoder_ok: true,
            stream_ok: true,
        }
    }

    pub fn synthesis_ok(&self) -> bool {
        self.text_ok && self.phoneme_ok && self.prosody_ok
    }

    pub fn output_ok(&self) -> bool {
        self.vocoder_ok && self.stream_ok
    }

    pub fn all_ok(&self) -> bool {
        self.synthesis_ok() && self.output_ok()
    }

    pub fn needs_voice(&self) -> bool {
        !self.vocoder_ok || !self.phoneme_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.text_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesis() {
        let c = TtsEngine::new();
        assert!(c.synthesis_ok());
    }

    #[test]
    fn test_output() {
        let c = TtsEngine::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TtsEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_voice() {
        let c = TtsEngine::new();
        assert!(!c.needs_voice());
    }

    #[test]
    fn test_vocoder() {
        let mut c = TtsEngine::new();
        c.vocoder_ok = false;
        assert!(c.needs_voice());
    }

    #[test]
    fn test_health() {
        let c = TtsEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
