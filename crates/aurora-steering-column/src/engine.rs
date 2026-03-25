/// Steering column: shaft, universal joint, lock
/// Phase 637

#[derive(Debug, Clone)]
pub struct SteeringColumn {
    pub shaft_ok: bool,
    pub uj_ok: bool,
    pub lock_ok: bool,
    pub tilt_ok: bool,
    pub telescope_ok: bool,
}

impl Default for SteeringColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringColumn {
    pub fn new() -> Self {
        Self {
            shaft_ok: true,
            uj_ok: true,
            lock_ok: true,
            tilt_ok: true,
            telescope_ok: true,
        }
    }

    pub fn drivetrain_ok(&self) -> bool {
        self.shaft_ok && self.uj_ok
    }

    pub fn adjustment_ok(&self) -> bool {
        self.tilt_ok && self.telescope_ok
    }

    pub fn all_ok(&self) -> bool {
        self.drivetrain_ok() && self.adjustment_ok() && self.lock_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.shaft_ok || !self.uj_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.shaft_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drivetrain() {
        let c = SteeringColumn::new();
        assert!(c.drivetrain_ok());
    }

    #[test]
    fn test_adjustment() {
        let c = SteeringColumn::new();
        assert!(c.adjustment_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringColumn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SteeringColumn::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_shaft() {
        let mut c = SteeringColumn::new();
        c.shaft_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SteeringColumn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
