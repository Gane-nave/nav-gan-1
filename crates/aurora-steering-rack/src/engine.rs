/// Steering rack: power steering, pinion, fluid
/// Phase 480

#[derive(Debug, Clone)]
pub struct SteeringRack {
    pub fluid_level_pct: f64,
    pub leak_detected: bool,
    pub noise_db: f64,
    pub play_deg: f64,
    pub assist_ok: bool,
}

impl Default for SteeringRack {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringRack {
    pub fn new() -> Self {
        Self {
            fluid_level_pct: 95.0,
            leak_detected: false,
            noise_db: 30.0,
            play_deg: 1.0,
            assist_ok: true,
        }
    }

    pub fn fluid_ok(&self) -> bool {
        self.fluid_level_pct > 60.0 && !self.leak_detected
    }

    pub fn noise_ok(&self) -> bool {
        self.noise_db < 50.0
    }

    pub fn all_ok(&self) -> bool {
        self.fluid_ok() && self.noise_ok() && self.assist_ok
    }

    pub fn needs_service(&self) -> bool {
        self.leak_detected || self.fluid_level_pct < 40.0
    }

    pub fn health_score(&self) -> f64 {
        if self.leak_detected { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid() {
        let c = SteeringRack::new();
        assert!(c.fluid_ok());
    }

    #[test]
    fn test_noise() {
        let c = SteeringRack::new();
        assert!(c.noise_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringRack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SteeringRack::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = SteeringRack::new();
        c.leak_detected = true;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SteeringRack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
