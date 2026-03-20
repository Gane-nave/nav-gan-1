/// Suspension test: bounce, sway, noise, alignment
/// Phase 826

#[derive(Debug, Clone)]
pub struct SuspensionTest {
    pub bounce_ok: bool,
    pub sway_ok: bool,
    pub noise_free: bool,
    pub aligned: bool,
    pub ride_ok: bool,
}

impl Default for SuspensionTest {
    fn default() -> Self {
        Self::new()
    }
}

impl SuspensionTest {
    pub fn new() -> Self {
        Self {
            bounce_ok: true,
            sway_ok: true,
            noise_free: true,
            aligned: true,
            ride_ok: true,
        }
    }

    pub fn dynamics_ok(&self) -> bool {
        self.bounce_ok && self.sway_ok && self.ride_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.noise_free && self.aligned
    }

    pub fn all_ok(&self) -> bool {
        self.dynamics_ok() && self.condition_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.bounce_ok || !self.aligned
    }

    pub fn health_score(&self) -> f64 {
        if !self.bounce_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamics() {
        let c = SuspensionTest::new();
        assert!(c.dynamics_ok());
    }

    #[test]
    fn test_condition() {
        let c = SuspensionTest::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SuspensionTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SuspensionTest::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_bounce() {
        let mut c = SuspensionTest::new();
        c.bounce_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SuspensionTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
