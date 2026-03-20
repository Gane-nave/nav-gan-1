/// Keyless entry: RF receiver, proximity sensor, antenna
/// Phase 543

#[derive(Debug, Clone)]
pub struct KeylessEntry {
    pub rf_ok: bool,
    pub proximity_ok: bool,
    pub antenna_ok: bool,
    pub battery_ok: bool,
    pub range_m: f64,
}

impl Default for KeylessEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl KeylessEntry {
    pub fn new() -> Self {
        Self {
            rf_ok: true,
            proximity_ok: true,
            antenna_ok: true,
            battery_ok: true,
            range_m: 10.0,
        }
    }

    pub fn comm_ok(&self) -> bool {
        self.rf_ok && self.antenna_ok
    }

    pub fn detection_ok(&self) -> bool {
        self.proximity_ok && self.range_m > 2.0
    }

    pub fn all_ok(&self) -> bool {
        self.comm_ok() && self.detection_ok() && self.battery_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.rf_ok || !self.battery_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rf_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comm() {
        let c = KeylessEntry::new();
        assert!(c.comm_ok());
    }

    #[test]
    fn test_detection() {
        let c = KeylessEntry::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KeylessEntry::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = KeylessEntry::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_rf() {
        let mut c = KeylessEntry::new();
        c.rf_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = KeylessEntry::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
