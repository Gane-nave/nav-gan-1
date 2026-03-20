/// Navigation antenna: GPS/GNSS reception, multiband, ground plane
/// Phase 433

#[derive(Debug, Clone)]
pub struct NavAntenna {
    pub signal_dbm: f64,
    pub min_dbm: f64,
    pub satellites: u8,
    pub multiband: bool,
    pub ground_plane_ok: bool,
}

impl Default for NavAntenna {
    fn default() -> Self {
        Self::new()
    }
}

impl NavAntenna {
    pub fn new() -> Self {
        Self {
            signal_dbm: -130.0,
            min_dbm: -160.0,
            satellites: 12,
            multiband: true,
            ground_plane_ok: true,
        }
    }

    pub fn signal_ok(&self) -> bool {
        self.signal_dbm > self.min_dbm
    }

    pub fn fix_quality(&self) -> &str {
        if self.satellites >= 8 {
            "excellent"
        } else if self.satellites >= 4 {
            "good"
        } else {
            "poor"
        }
    }

    pub fn all_ok(&self) -> bool {
        self.signal_ok() && self.satellites >= 4 && self.ground_plane_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.signal_ok() || !self.ground_plane_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.signal_ok() {
            return 20.0;
        }
        if self.satellites < 4 {
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
        let n = NavAntenna::new();
        assert!(n.signal_ok());
    }

    #[test]
    fn test_fix() {
        let n = NavAntenna::new();
        assert_eq!(n.fix_quality(), "excellent");
    }

    #[test]
    fn test_all_ok() {
        let n = NavAntenna::new();
        assert!(n.all_ok());
    }

    #[test]
    fn test_no_service() {
        let n = NavAntenna::new();
        assert!(!n.needs_service());
    }

    #[test]
    fn test_weak() {
        let mut n = NavAntenna::new();
        n.signal_dbm = -170.0;
        assert!(n.needs_service());
    }

    #[test]
    fn test_health() {
        let n = NavAntenna::new();
        assert!((n.health_score() - 100.0).abs() < 0.1);
    }
}
