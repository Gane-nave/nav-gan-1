/// Traffic light: detection, countdown, green wave, priority
/// Phase 931

#[derive(Debug, Clone)]
pub struct TrafficLight {
    pub detect_ok: bool,
    pub countdown_ok: bool,
    pub wave_ok: bool,
    pub priority_ok: bool,
    pub v2i_ok: bool,
}

impl Default for TrafficLight {
    fn default() -> Self {
        Self::new()
    }
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            countdown_ok: true,
            wave_ok: true,
            priority_ok: true,
            v2i_ok: true,
        }
    }

    pub fn recognition_ok(&self) -> bool {
        self.detect_ok && self.countdown_ok && self.v2i_ok
    }

    pub fn optimization_ok(&self) -> bool {
        self.wave_ok && self.priority_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recognition_ok() && self.optimization_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.v2i_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognition() {
        let c = TrafficLight::new();
        assert!(c.recognition_ok());
    }

    #[test]
    fn test_optimization() {
        let c = TrafficLight::new();
        assert!(c.optimization_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrafficLight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = TrafficLight::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_detect() {
        let mut c = TrafficLight::new();
        c.detect_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = TrafficLight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
