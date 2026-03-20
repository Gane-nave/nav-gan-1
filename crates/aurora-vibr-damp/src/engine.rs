/// Vibration damper: harmonic balancer, engine mount isolation
/// Phase 364

#[derive(Debug, Clone)]
pub struct VibrDamp {
    pub amplitude_mm: f64,
    pub max_amplitude_mm: f64,
    pub frequency_hz: f64,
    pub mount_ok: bool,
    pub balancer_ok: bool,
}

impl Default for VibrDamp {
    fn default() -> Self {
        Self::new()
    }
}

impl VibrDamp {
    pub fn new() -> Self {
        Self {
            amplitude_mm: 0.1,
            max_amplitude_mm: 1.0,
            frequency_hz: 30.0,
            mount_ok: true,
            balancer_ok: true,
        }
    }

    pub fn vibration_ok(&self) -> bool {
        self.amplitude_mm < self.max_amplitude_mm
    }

    pub fn resonance_risk(&self) -> bool {
        self.frequency_hz > 20.0 && self.frequency_hz < 40.0 && self.amplitude_mm > 0.5
    }

    pub fn all_ok(&self) -> bool {
        self.mount_ok && self.balancer_ok && self.vibration_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.mount_ok || !self.balancer_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.balancer_ok {
            return 0.0;
        }
        if !self.mount_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibration() {
        let v = VibrDamp::new();
        assert!(v.vibration_ok());
    }

    #[test]
    fn test_no_resonance() {
        let v = VibrDamp::new();
        assert!(!v.resonance_risk());
    }

    #[test]
    fn test_all_ok() {
        let v = VibrDamp::new();
        assert!(v.all_ok());
    }

    #[test]
    fn test_no_service() {
        let v = VibrDamp::new();
        assert!(!v.needs_service());
    }

    #[test]
    fn test_bad_mount() {
        let mut v = VibrDamp::new();
        v.mount_ok = false;
        assert!(v.needs_service());
    }

    #[test]
    fn test_health() {
        let v = VibrDamp::new();
        assert!((v.health_score() - 100.0).abs() < 0.1);
    }
}
