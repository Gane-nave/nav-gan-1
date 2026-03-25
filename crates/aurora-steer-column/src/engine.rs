/// Steering column: tilt/telescopic, collapsible, lock mechanism
/// Phase 337

#[derive(Debug, Clone)]
pub struct SteerColumn {
    pub tilt_deg: f64,
    pub telescope_mm: f64,
    pub locked: bool,
    pub collapse_ok: bool,
    pub motor_ok: bool,
}

impl Default for SteerColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl SteerColumn {
    pub fn new() -> Self {
        Self {
            tilt_deg: 0.0,
            telescope_mm: 0.0,
            locked: true,
            collapse_ok: true,
            motor_ok: true,
        }
    }

    pub fn adjusted(&self) -> bool {
        self.tilt_deg.abs() > 0.5 || self.telescope_mm.abs() > 0.5
    }

    pub fn safety_ok(&self) -> bool {
        self.collapse_ok
    }

    pub fn can_adjust(&self) -> bool {
        !self.locked && self.motor_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.collapse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collapse_ok {
            return 0.0;
        }
        if !self.motor_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_adjusted() {
        let s = SteerColumn::new();
        assert!(!s.adjusted());
    }

    #[test]
    fn test_safety() {
        let s = SteerColumn::new();
        assert!(s.safety_ok());
    }

    #[test]
    fn test_cannot_adjust() {
        let s = SteerColumn::new();
        assert!(!s.can_adjust());
    }

    #[test]
    fn test_no_service() {
        let s = SteerColumn::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_unlocked() {
        let mut s = SteerColumn::new();
        s.locked = false;
        assert!(s.can_adjust());
    }

    #[test]
    fn test_health() {
        let s = SteerColumn::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
