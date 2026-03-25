/// cam sensor: detect, phase, sync, signal, check
/// Phase 1246

#[derive(Debug, Clone)]
pub struct CamSensor {
    pub detect_ok: bool,
    pub phase_ok: bool,
    pub sync_ok: bool,
    pub signal_ok: bool,
    pub check_ok: bool,
}

impl Default for CamSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CamSensor {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            phase_ok: true,
            sync_ok: true,
            signal_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.phase_ok && self.sync_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.signal_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.phase_ok
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
    fn test_primary() {
        let c = CamSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CamSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CamSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CamSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CamSensor::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CamSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
