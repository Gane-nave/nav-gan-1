/// V2X communication: DSRC, C-V2X, antenna, security
/// Phase 705

#[derive(Debug, Clone)]
pub struct V2xModule {
    pub dsrc_ok: bool,
    pub cv2x_ok: bool,
    pub antenna_ok: bool,
    pub security_ok: bool,
    pub latency_ok: bool,
}

impl Default for V2xModule {
    fn default() -> Self {
        Self::new()
    }
}

impl V2xModule {
    pub fn new() -> Self {
        Self {
            dsrc_ok: true,
            cv2x_ok: true,
            antenna_ok: true,
            security_ok: true,
            latency_ok: true,
        }
    }

    pub fn radio_ok(&self) -> bool {
        self.dsrc_ok && self.cv2x_ok && self.antenna_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.security_ok && self.latency_ok
    }

    pub fn all_ok(&self) -> bool {
        self.radio_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.dsrc_ok || !self.security_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.dsrc_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio() {
        let c = V2xModule::new();
        assert!(c.radio_ok());
    }

    #[test]
    fn test_safety() {
        let c = V2xModule::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2xModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = V2xModule::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_dsrc() {
        let mut c = V2xModule::new();
        c.dsrc_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = V2xModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
