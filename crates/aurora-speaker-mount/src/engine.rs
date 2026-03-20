/// Speaker mount: door speaker, dash speaker, rear deck, vibration isolation
/// Phase 430

#[derive(Debug, Clone)]
pub struct SpeakerMount {
    pub secure: bool,
    pub sealed: bool,
    pub rattle_free: bool,
    pub impedance_ok: bool,
    pub count: u8,
}

impl Default for SpeakerMount {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeakerMount {
    pub fn new() -> Self {
        Self {
            secure: true,
            sealed: true,
            rattle_free: true,
            impedance_ok: true,
            count: 8,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.secure && self.sealed && self.rattle_free && self.impedance_ok
    }

    pub fn sound_quality_ok(&self) -> bool {
        self.sealed && self.rattle_free
    }

    pub fn needs_service(&self) -> bool {
        !self.secure || !self.impedance_ok
    }

    pub fn resonance_free(&self) -> bool {
        self.rattle_free && self.sealed
    }

    pub fn health_score(&self) -> f64 {
        if !self.secure {
            return 20.0;
        }
        if !self.rattle_free {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let s = SpeakerMount::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_quality() {
        let s = SpeakerMount::new();
        assert!(s.sound_quality_ok());
    }

    #[test]
    fn test_no_service() {
        let s = SpeakerMount::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_resonance() {
        let s = SpeakerMount::new();
        assert!(s.resonance_free());
    }

    #[test]
    fn test_loose() {
        let mut s = SpeakerMount::new();
        s.secure = false;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = SpeakerMount::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
