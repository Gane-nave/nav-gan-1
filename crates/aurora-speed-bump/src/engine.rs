/// Speed bump: detection, warning, suspension adjust, comfort
/// Phase 940

#[derive(Debug, Clone)]
pub struct SpeedBump {
    pub detect_ok: bool,
    pub warning_ok: bool,
    pub suspend_ok: bool,
    pub comfort_ok: bool,
    pub map_ok: bool,
}

impl Default for SpeedBump {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeedBump {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            warning_ok: true,
            suspend_ok: true,
            comfort_ok: true,
            map_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.detect_ok && self.map_ok
    }

    pub fn response_ok(&self) -> bool {
        self.warning_ok && self.suspend_ok && self.comfort_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.response_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.map_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.map_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = SpeedBump::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_response() {
        let c = SpeedBump::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpeedBump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = SpeedBump::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_map() {
        let mut c = SpeedBump::new();
        c.map_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = SpeedBump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
