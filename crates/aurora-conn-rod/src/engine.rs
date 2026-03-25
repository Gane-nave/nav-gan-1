/// Connecting rod: bearing clearance, bolt stretch, big-end wear
/// Phase 315

#[derive(Debug, Clone)]
pub struct ConnectingRod {
    pub bearing_clearance_mm: f64,
    pub max_clearance_mm: f64,
    pub bolt_torque_nm: f64,
    pub target_torque_nm: f64,
    pub rod_ok: bool,
}

impl Default for ConnectingRod {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectingRod {
    pub fn new() -> Self {
        Self {
            bearing_clearance_mm: 0.04,
            max_clearance_mm: 0.08,
            bolt_torque_nm: 45.0,
            target_torque_nm: 45.0,
            rod_ok: true,
        }
    }

    pub fn clearance_ok(&self) -> bool {
        self.bearing_clearance_mm < self.max_clearance_mm
    }

    pub fn torque_ok(&self) -> bool {
        (self.bolt_torque_nm - self.target_torque_nm).abs() < 5.0
    }

    pub fn needs_service(&self) -> bool {
        !self.clearance_ok() || !self.rod_ok
    }

    pub fn wear_pct(&self) -> f64 {
        (self.bearing_clearance_mm / self.max_clearance_mm * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.rod_ok {
            return 0.0;
        }
        if !self.clearance_ok() {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clearance() {
        let c = ConnectingRod::new();
        assert!(c.clearance_ok());
    }

    #[test]
    fn test_torque() {
        let c = ConnectingRod::new();
        assert!(c.torque_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ConnectingRod::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_wear() {
        let c = ConnectingRod::new();
        assert!(c.wear_pct() < 60.0);
    }

    #[test]
    fn test_worn() {
        let mut c = ConnectingRod::new();
        c.bearing_clearance_mm = 0.1;
        assert!(!c.clearance_ok());
    }

    #[test]
    fn test_health() {
        let c = ConnectingRod::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
