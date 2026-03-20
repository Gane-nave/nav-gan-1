/// Clamp force: bolt tension, preload, relaxation monitoring
/// Phase 401

#[derive(Debug, Clone)]
pub struct ClampForce {
    pub force_kn: f64,
    pub target_kn: f64,
    pub tolerance_pct: f64,
    pub relaxed: bool,
    pub retorqued: bool,
}

impl Default for ClampForce {
    fn default() -> Self {
        Self::new()
    }
}

impl ClampForce {
    pub fn new() -> Self {
        Self {
            force_kn: 45.0,
            target_kn: 50.0,
            tolerance_pct: 15.0,
            relaxed: false,
            retorqued: false,
        }
    }

    pub fn in_spec(&self) -> bool {
        let min = self.target_kn * (1.0 - self.tolerance_pct / 100.0);
        let max = self.target_kn * (1.0 + self.tolerance_pct / 100.0);
        (min..=max).contains(&self.force_kn)
    }

    pub fn needs_retorque(&self) -> bool {
        self.relaxed || !self.in_spec()
    }

    pub fn force_pct(&self) -> f64 {
        if self.target_kn <= 0.0 {
            return 0.0;
        }
        (self.force_kn / self.target_kn * 100.0).clamp(0.0, 200.0)
    }

    pub fn margin_ok(&self) -> bool {
        self.force_pct() > 85.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.in_spec() {
            return 20.0;
        }
        if self.relaxed {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_spec() {
        let c = ClampForce::new();
        assert!(c.in_spec());
    }

    #[test]
    fn test_no_retorque() {
        let c = ClampForce::new();
        assert!(!c.needs_retorque());
    }

    #[test]
    fn test_force_pct() {
        let c = ClampForce::new();
        assert!(c.force_pct() > 85.0);
    }

    #[test]
    fn test_margin() {
        let c = ClampForce::new();
        assert!(c.margin_ok());
    }

    #[test]
    fn test_relaxed() {
        let mut c = ClampForce::new();
        c.relaxed = true;
        assert!(c.needs_retorque());
    }

    #[test]
    fn test_health() {
        let c = ClampForce::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
