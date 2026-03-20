/// map parking: scan, detect, navigate, reserve, log
/// Phase 1444

#[derive(Debug, Clone)]
pub struct MapParking {
    pub scan_ok: bool,
    pub detect_ok: bool,
    pub navigate_ok: bool,
    pub reserve_ok: bool,
    pub log_ok: bool,
}

impl Default for MapParking {
    fn default() -> Self {
        Self::new()
    }
}

impl MapParking {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            detect_ok: true,
            navigate_ok: true,
            reserve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.detect_ok && self.navigate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reserve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MapParking::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapParking::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapParking::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapParking::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapParking::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapParking::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
