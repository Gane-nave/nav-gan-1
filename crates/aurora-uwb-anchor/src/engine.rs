/// UWB anchor: ranging, angle, position, fence, key
/// Phase 982

#[derive(Debug, Clone)]
pub struct UwbAnchor {
    pub ranging_ok: bool,
    pub angle_ok: bool,
    pub position_ok: bool,
    pub fence_ok: bool,
    pub key_ok: bool,
}

impl Default for UwbAnchor {
    fn default() -> Self {
        Self::new()
    }
}

impl UwbAnchor {
    pub fn new() -> Self {
        Self {
            ranging_ok: true,
            angle_ok: true,
            position_ok: true,
            fence_ok: true,
            key_ok: true,
        }
    }

    pub fn localization_ok(&self) -> bool {
        self.ranging_ok && self.angle_ok && self.position_ok
    }

    pub fn security_ok(&self) -> bool {
        self.fence_ok && self.key_ok
    }

    pub fn all_ok(&self) -> bool {
        self.localization_ok() && self.security_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.ranging_ok || !self.angle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ranging_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localization() {
        let c = UwbAnchor::new();
        assert!(c.localization_ok());
    }

    #[test]
    fn test_security() {
        let c = UwbAnchor::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UwbAnchor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = UwbAnchor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_ranging() {
        let mut c = UwbAnchor::new();
        c.ranging_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = UwbAnchor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
