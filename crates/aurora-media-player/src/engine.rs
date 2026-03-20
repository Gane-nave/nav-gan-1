/// media player: play, pause, skip, shuffle, repeat
/// Phase 1176

#[derive(Debug, Clone)]
pub struct MediaPlayer {
    pub play_ok: bool,
    pub pause_ok: bool,
    pub skip_ok: bool,
    pub shuffle_ok: bool,
    pub repeat_ok: bool,
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            play_ok: true,
            pause_ok: true,
            skip_ok: true,
            shuffle_ok: true,
            repeat_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.play_ok && self.pause_ok && self.skip_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.shuffle_ok && self.repeat_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.play_ok || !self.pause_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.play_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MediaPlayer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MediaPlayer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MediaPlayer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MediaPlayer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MediaPlayer::new();
        c.play_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MediaPlayer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
