/// Wastegate: boost control, actuator, spring
/// Phase 505

#[derive(Debug, Clone)]
pub struct Wastegate {
    pub position_pct: f64,
    pub target_pct: f64,
    pub actuator_ok: bool,
    pub spring_ok: bool,
    pub stuck: bool,
}

impl Default for Wastegate {
    fn default() -> Self {
        Self::new()
    }
}

impl Wastegate {
    pub fn new() -> Self {
        Self {
            position_pct: 30.0,
            target_pct: 30.0,
            actuator_ok: true,
            spring_ok: true,
            stuck: false,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.position_pct - self.target_pct).abs() < 5.0
    }

    pub fn is_functional(&self) -> bool {
        self.actuator_ok && self.spring_ok && !self.stuck
    }

    pub fn all_ok(&self) -> bool {
        self.at_target() && self.is_functional()
    }

    pub fn needs_service(&self) -> bool {
        self.stuck || !self.actuator_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let c = Wastegate::new();
        assert!(c.at_target());
    }

    #[test]
    fn test_functional() {
        let c = Wastegate::new();
        assert!(c.is_functional());
    }

    #[test]
    fn test_all_ok() {
        let c = Wastegate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Wastegate::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_stuck() {
        let mut c = Wastegate::new();
        c.stuck = true;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Wastegate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
