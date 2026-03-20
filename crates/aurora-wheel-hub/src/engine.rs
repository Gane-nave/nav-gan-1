/// Wheel hub: bearing, seal, ABS tone ring
/// Phase 483

#[derive(Debug, Clone)]
pub struct WheelHub {
    pub bearing_play_mm: f64,
    pub max_play_mm: f64,
    pub seal_ok: bool,
    pub tone_ring_ok: bool,
    pub noise_detected: bool,
}

impl Default for WheelHub {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelHub {
    pub fn new() -> Self {
        Self {
            bearing_play_mm: 0.02,
            max_play_mm: 0.1,
            seal_ok: true,
            tone_ring_ok: true,
            noise_detected: false,
        }
    }

    pub fn play_pct(&self) -> f64 {
        (self.bearing_play_mm / self.max_play_mm) * 100.0
    }

    pub fn excessive_play(&self) -> bool {
        self.bearing_play_mm > self.max_play_mm * 0.8
    }

    pub fn all_ok(&self) -> bool {
        self.seal_ok && self.tone_ring_ok && !self.noise_detected && !self.excessive_play()
    }

    pub fn needs_replacement(&self) -> bool {
        self.noise_detected || self.excessive_play()
    }

    pub fn health_score(&self) -> f64 {
        if self.noise_detected { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let c = WheelHub::new();
        assert!(c.play_pct() < 30.0);
    }

    #[test]
    fn test_no_excessive() {
        let c = WheelHub::new();
        assert!(!c.excessive_play());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelHub::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WheelHub::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_noise() {
        let mut c = WheelHub::new();
        c.noise_detected = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WheelHub::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
