/// speech synth: load, generate, queue, play, stop
/// Phase 1181

#[derive(Debug, Clone)]
pub struct SpeechSynth {
    pub load_ok: bool,
    pub generate_ok: bool,
    pub queue_ok: bool,
    pub play_ok: bool,
    pub stop_ok: bool,
}

impl Default for SpeechSynth {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeechSynth {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            generate_ok: true,
            queue_ok: true,
            play_ok: true,
            stop_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.generate_ok && self.queue_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.play_ok && self.stop_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.generate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SpeechSynth::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SpeechSynth::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpeechSynth::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SpeechSynth::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SpeechSynth::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SpeechSynth::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
