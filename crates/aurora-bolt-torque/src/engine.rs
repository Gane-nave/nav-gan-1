/// Bolt torque: fastener preload, angle tightening, yield control
/// Phase 402

#[derive(Debug, Clone)]
pub struct BoltTorque {
    pub torque_nm: f64,
    pub target_nm: f64,
    pub tolerance_pct: f64,
    pub angle_deg: f64,
    pub yield_reached: bool,
}

impl Default for BoltTorque {
    fn default() -> Self {
        Self::new()
    }
}

impl BoltTorque {
    pub fn new() -> Self {
        Self {
            torque_nm: 95.0,
            target_nm: 100.0,
            tolerance_pct: 10.0,
            angle_deg: 90.0,
            yield_reached: false,
        }
    }

    pub fn in_spec(&self) -> bool {
        let min = self.target_nm * (1.0 - self.tolerance_pct / 100.0);
        let max = self.target_nm * (1.0 + self.tolerance_pct / 100.0);
        (min..=max).contains(&self.torque_nm)
    }

    pub fn needs_retorque(&self) -> bool {
        !self.in_spec()
    }

    pub fn over_torqued(&self) -> bool {
        self.yield_reached
    }

    pub fn torque_pct(&self) -> f64 {
        if self.target_nm <= 0.0 {
            return 0.0;
        }
        (self.torque_nm / self.target_nm * 100.0).clamp(0.0, 200.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.yield_reached {
            return 0.0;
        }
        if !self.in_spec() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_spec() {
        let b = BoltTorque::new();
        assert!(b.in_spec());
    }

    #[test]
    fn test_no_retorque() {
        let b = BoltTorque::new();
        assert!(!b.needs_retorque());
    }

    #[test]
    fn test_not_over() {
        let b = BoltTorque::new();
        assert!(!b.over_torqued());
    }

    #[test]
    fn test_pct() {
        let b = BoltTorque::new();
        assert!(b.torque_pct() > 90.0);
    }

    #[test]
    fn test_yield() {
        let mut b = BoltTorque::new();
        b.yield_reached = true;
        assert!(b.over_torqued());
    }

    #[test]
    fn test_health() {
        let b = BoltTorque::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
