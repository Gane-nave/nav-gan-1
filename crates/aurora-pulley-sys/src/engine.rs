/// Pulley system: idler pulley, bearing, alignment, noise
/// Phase 323

#[derive(Debug, Clone)]
pub struct PulleySystem {
    pub pulley_count: u8,
    pub all_bearings_ok: bool,
    pub alignment_ok: bool,
    pub noise_detected: bool,
    pub wobble_detected: bool,
}

impl Default for PulleySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PulleySystem {
    pub fn new() -> Self {
        Self {
            pulley_count: 4,
            all_bearings_ok: true,
            alignment_ok: true,
            noise_detected: false,
            wobble_detected: false,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.all_bearings_ok && self.alignment_ok && !self.noise_detected && !self.wobble_detected
    }

    pub fn needs_service(&self) -> bool {
        !self.all_bearings_ok || self.wobble_detected
    }

    pub fn running_smooth(&self) -> bool {
        !self.noise_detected && !self.wobble_detected
    }

    pub fn health_score(&self) -> f64 {
        if !self.all_bearings_ok {
            return 0.0;
        }
        if self.wobble_detected {
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
    fn test_all_ok() {
        let p = PulleySystem::new();
        assert!(p.all_ok());
    }

    #[test]
    fn test_no_service() {
        let p = PulleySystem::new();
        assert!(!p.needs_service());
    }

    #[test]
    fn test_smooth() {
        let p = PulleySystem::new();
        assert!(p.running_smooth());
    }

    #[test]
    fn test_count() {
        let p = PulleySystem::new();
        assert_eq!(p.pulley_count, 4);
    }

    #[test]
    fn test_wobble() {
        let mut p = PulleySystem::new();
        p.wobble_detected = true;
        assert!(p.needs_service());
    }

    #[test]
    fn test_health() {
        let p = PulleySystem::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
