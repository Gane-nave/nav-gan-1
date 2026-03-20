/// Strut mount: bearing, rubber, plate, noise
/// Phase 642

#[derive(Debug, Clone)]
pub struct StrutMount {
    pub bearing_ok: bool,
    pub rubber_ok: bool,
    pub plate_ok: bool,
    pub noise_free: bool,
    pub aligned: bool,
}

impl Default for StrutMount {
    fn default() -> Self {
        Self::new()
    }
}

impl StrutMount {
    pub fn new() -> Self {
        Self {
            bearing_ok: true,
            rubber_ok: true,
            plate_ok: true,
            noise_free: true,
            aligned: true,
        }
    }

    pub fn bearing_good(&self) -> bool {
        self.bearing_ok && self.noise_free
    }

    pub fn mount_ok(&self) -> bool {
        self.rubber_ok && self.plate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.bearing_good() && self.mount_ok() && self.aligned
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bearing_ok || !self.rubber_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bearing() {
        let c = StrutMount::new();
        assert!(c.bearing_good());
    }

    #[test]
    fn test_mount() {
        let c = StrutMount::new();
        assert!(c.mount_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StrutMount::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = StrutMount::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bearing_fail() {
        let mut c = StrutMount::new();
        c.bearing_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = StrutMount::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
