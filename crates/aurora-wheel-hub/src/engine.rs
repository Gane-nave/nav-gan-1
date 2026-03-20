/// Wheel hub: bearing condition, ABS tone ring, hub assembly
/// Phase 328

#[derive(Debug, Clone)]
pub struct WheelHub {
    pub bearing_ok: bool,
    pub tone_ring_ok: bool,
    pub play_mm: f64,
    pub max_play_mm: f64,
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
            bearing_ok: true,
            tone_ring_ok: true,
            play_mm: 0.02,
            max_play_mm: 0.1,
            noise_detected: false,
        }
    }

    pub fn play_ok(&self) -> bool {
        self.play_mm < self.max_play_mm
    }

    pub fn all_ok(&self) -> bool {
        self.bearing_ok && self.tone_ring_ok && self.play_ok() && !self.noise_detected
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bearing_ok || self.play_mm >= self.max_play_mm
    }

    pub fn abs_compatible(&self) -> bool {
        self.tone_ring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok {
            return 0.0;
        }
        if !self.tone_ring_ok {
            return 30.0;
        }
        if self.noise_detected {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_ok() {
        let w = WheelHub::new();
        assert!(w.play_ok());
    }

    #[test]
    fn test_all_ok() {
        let w = WheelHub::new();
        assert!(w.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let w = WheelHub::new();
        assert!(!w.needs_replacement());
    }

    #[test]
    fn test_abs() {
        let w = WheelHub::new();
        assert!(w.abs_compatible());
    }

    #[test]
    fn test_bad_bearing() {
        let mut w = WheelHub::new();
        w.bearing_ok = false;
        assert!(w.needs_replacement());
    }

    #[test]
    fn test_health() {
        let w = WheelHub::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
