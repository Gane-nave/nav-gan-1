/// Wastegate: turbo boost control, overboost protection, duty cycle
/// Phase 309

#[derive(Debug, Clone)]
pub struct Wastegate {
    pub position_pct: f64,
    pub boost_bar: f64,
    pub target_boost_bar: f64,
    pub duty_cycle_pct: f64,
    pub actuator_ok: bool,
}

impl Default for Wastegate {
    fn default() -> Self {
        Self::new()
    }
}

impl Wastegate {
    pub fn new() -> Self {
        Self {
            position_pct: 50.0,
            boost_bar: 1.0,
            target_boost_bar: 1.2,
            duty_cycle_pct: 40.0,
            actuator_ok: true,
        }
    }

    pub fn boost_ok(&self) -> bool {
        (self.boost_bar - self.target_boost_bar).abs() < 0.15
    }

    pub fn overboost(&self) -> bool {
        self.boost_bar > self.target_boost_bar * 1.2
    }

    pub fn underboost(&self) -> bool {
        self.boost_bar < self.target_boost_bar * 0.7
    }

    pub fn fully_open(&self) -> bool {
        self.position_pct > 95.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok {
            return 0.0;
        }
        if self.overboost() {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boost_ok() {
        let w = Wastegate::new();
        assert!(!w.boost_ok());
    }

    #[test]
    fn test_no_overboost() {
        let w = Wastegate::new();
        assert!(!w.overboost());
    }

    #[test]
    fn test_no_underboost() {
        let w = Wastegate::new();
        assert!(!w.underboost());
    }

    #[test]
    fn test_not_open() {
        let w = Wastegate::new();
        assert!(!w.fully_open());
    }

    #[test]
    fn test_overboost() {
        let mut w = Wastegate::new();
        w.boost_bar = 2.0;
        assert!(w.overboost());
    }

    #[test]
    fn test_health() {
        let w = Wastegate::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
