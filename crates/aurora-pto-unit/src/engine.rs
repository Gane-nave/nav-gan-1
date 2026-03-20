/// PTO unit: power take-off, auxiliary drive, engagement
/// Phase 464

#[derive(Debug, Clone)]
pub struct PtoUnit {
    pub engaged: bool,
    pub clutch_ok: bool,
    pub shaft_ok: bool,
    pub speed_rpm: f64,
    pub max_rpm: f64,
}

impl Default for PtoUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl PtoUnit {
    pub fn new() -> Self {
        Self {
            engaged: false,
            clutch_ok: true,
            shaft_ok: true,
            speed_rpm: 0.0,
            max_rpm: 1500.0,
        }
    }

    pub fn ready(&self) -> bool {
        self.clutch_ok && self.shaft_ok
    }

    pub fn all_ok(&self) -> bool {
        self.clutch_ok && self.shaft_ok
    }

    pub fn overspeed(&self) -> bool {
        self.speed_rpm > self.max_rpm
    }

    pub fn needs_service(&self) -> bool {
        !self.clutch_ok || !self.shaft_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.shaft_ok {
            return 10.0;
        }
        if !self.clutch_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ready() {
        let p = PtoUnit::new();
        assert!(p.ready());
    }

    #[test]
    fn test_all_ok() {
        let p = PtoUnit::new();
        assert!(p.all_ok());
    }

    #[test]
    fn test_no_overspeed() {
        let p = PtoUnit::new();
        assert!(!p.overspeed());
    }

    #[test]
    fn test_no_service() {
        let p = PtoUnit::new();
        assert!(!p.needs_service());
    }

    #[test]
    fn test_bad_clutch() {
        let mut p = PtoUnit::new();
        p.clutch_ok = false;
        assert!(p.needs_service());
    }

    #[test]
    fn test_health() {
        let p = PtoUnit::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
