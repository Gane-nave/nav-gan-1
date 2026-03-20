/// Voice assistant: wake word, NLU, TTS, command, context
/// Phase 896

#[derive(Debug, Clone)]
pub struct VoiceAssist {
    pub wake_ok: bool,
    pub nlu_ok: bool,
    pub tts_ok: bool,
    pub command_ok: bool,
    pub context_ok: bool,
}

impl Default for VoiceAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceAssist {
    pub fn new() -> Self {
        Self {
            wake_ok: true,
            nlu_ok: true,
            tts_ok: true,
            command_ok: true,
            context_ok: true,
        }
    }

    pub fn recognition_ok(&self) -> bool {
        self.wake_ok && self.nlu_ok && self.context_ok
    }

    pub fn output_ok(&self) -> bool {
        self.tts_ok && self.command_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recognition_ok() && self.output_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.nlu_ok || !self.wake_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.nlu_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognition() {
        let c = VoiceAssist::new();
        assert!(c.recognition_ok());
    }

    #[test]
    fn test_output() {
        let c = VoiceAssist::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VoiceAssist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = VoiceAssist::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_nlu() {
        let mut c = VoiceAssist::new();
        c.nlu_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = VoiceAssist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
