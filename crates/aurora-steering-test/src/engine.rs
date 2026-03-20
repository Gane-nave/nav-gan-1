/// Steering test: play, effort, returnability, noise
/// Phase 827

#[derive(Debug, Clone)]
pub struct SteeringTest {
    pub play_ok: bool,
    pub effort_ok: bool,
    pub return_ok: bool,
    pub noise_free: bool,
    pub fluid_ok: bool,
}

impl Default for SteeringTest {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringTest {
    pub fn new() -> Self {
        Self {
            play_ok: true,
            effort_ok: true,
            return_ok: true,
            noise_free: true,
            fluid_ok: true,
        }
    }

    pub fn response_ok(&self) -> bool {
        self.play_ok && self.effort_ok && self.return_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.noise_free && self.fluid_ok
    }

    pub fn all_ok(&self) -> bool {
        self.response_ok() && self.condition_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.play_ok || !self.effort_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.play_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        let c = SteeringTest::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_condition() {
        let c = SteeringTest::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SteeringTest::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_play() {
        let mut c = SteeringTest::new();
        c.play_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SteeringTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
