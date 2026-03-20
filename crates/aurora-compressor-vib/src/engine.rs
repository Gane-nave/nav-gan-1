/// Compressor vibration: AC compressor, turbo, supercharger vibration
/// Phase 377

#[derive(Debug, Clone)]
pub struct CompressorVib {
    pub vibration_mm_s: f64,
    pub max_vibration_mm_s: f64,
    pub mount_ok: bool,
    pub bearing_ok: bool,
    pub clutch_ok: bool,
}

impl Default for CompressorVib {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressorVib {
    pub fn new() -> Self {
        Self {
            vibration_mm_s: 1.5,
            max_vibration_mm_s: 8.0,
            mount_ok: true,
            bearing_ok: true,
            clutch_ok: true,
        }
    }

    pub fn vibration_ok(&self) -> bool {
        self.vibration_mm_s < self.max_vibration_mm_s
    }

    pub fn all_ok(&self) -> bool {
        self.mount_ok && self.bearing_ok && self.clutch_ok && self.vibration_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.bearing_ok || !self.mount_ok
    }

    pub fn severity_pct(&self) -> f64 {
        if self.max_vibration_mm_s <= 0.0 {
            return 0.0;
        }
        (self.vibration_mm_s / self.max_vibration_mm_s * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok {
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
        let c = CompressorVib::new();
        assert!(c.vibration_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CompressorVib::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CompressorVib::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_severity() {
        let c = CompressorVib::new();
        assert!(c.severity_pct() < 25.0);
    }

    #[test]
    fn test_bad_bearing() {
        let mut c = CompressorVib::new();
        c.bearing_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CompressorVib::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
