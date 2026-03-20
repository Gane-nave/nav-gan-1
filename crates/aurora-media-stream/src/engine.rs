/// Media streaming: audio, video, casting, bluetooth, wifi
/// Phase 905

#[derive(Debug, Clone)]
pub struct MediaStream {
    pub audio_ok: bool,
    pub video_ok: bool,
    pub casting_ok: bool,
    pub bt_ok: bool,
    pub wifi_ok: bool,
}

impl Default for MediaStream {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaStream {
    pub fn new() -> Self {
        Self {
            audio_ok: true,
            video_ok: true,
            casting_ok: true,
            bt_ok: true,
            wifi_ok: true,
        }
    }

    pub fn playback_ok(&self) -> bool {
        self.audio_ok && self.video_ok
    }

    pub fn connectivity_ok(&self) -> bool {
        self.casting_ok && self.bt_ok && self.wifi_ok
    }

    pub fn all_ok(&self) -> bool {
        self.playback_ok() && self.connectivity_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.bt_ok || !self.wifi_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bt_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback() {
        let c = MediaStream::new();
        assert!(c.playback_ok());
    }

    #[test]
    fn test_connectivity() {
        let c = MediaStream::new();
        assert!(c.connectivity_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MediaStream::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = MediaStream::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_bt() {
        let mut c = MediaStream::new();
        c.bt_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = MediaStream::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
