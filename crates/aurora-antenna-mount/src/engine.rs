/// Antenna mount: shark fin, mast, embedded, GPS/LTE/radio reception
/// Phase 417

#[derive(Debug, Clone)]
pub struct AntennaMount {
    pub signal_dbm: f64,
    pub min_dbm: f64,
    pub mount_secure: bool,
    pub cable_ok: bool,
    pub waterproof: bool,
}

impl Default for AntennaMount {
    fn default() -> Self {
        Self::new()
    }
}

impl AntennaMount {
    pub fn new() -> Self {
        Self {
            signal_dbm: -65.0,
            min_dbm: -90.0,
            mount_secure: true,
            cable_ok: true,
            waterproof: true,
        }
    }

    pub fn signal_ok(&self) -> bool {
        self.signal_dbm > self.min_dbm
    }

    pub fn all_ok(&self) -> bool {
        self.signal_ok() && self.mount_secure && self.cable_ok && self.waterproof
    }

    pub fn needs_service(&self) -> bool {
        !self.mount_secure || !self.cable_ok
    }

    pub fn signal_quality_pct(&self) -> f64 {
        ((self.signal_dbm + 100.0) / 60.0 * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.mount_secure {
            return 20.0;
        }
        if !self.cable_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        let a = AntennaMount::new();
        assert!(a.signal_ok());
    }

    #[test]
    fn test_all_ok() {
        let a = AntennaMount::new();
        assert!(a.all_ok());
    }

    #[test]
    fn test_no_service() {
        let a = AntennaMount::new();
        assert!(!a.needs_service());
    }

    #[test]
    fn test_quality() {
        let a = AntennaMount::new();
        assert!(a.signal_quality_pct() > 50.0);
    }

    #[test]
    fn test_loose() {
        let mut a = AntennaMount::new();
        a.mount_secure = false;
        assert!(a.needs_service());
    }

    #[test]
    fn test_health() {
        let a = AntennaMount::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
