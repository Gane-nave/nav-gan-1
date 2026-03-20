/// Mirror motor: tilt, fold, heat, memory
/// Phase 681

#[derive(Debug, Clone)]
pub struct MirrorMotor {
    pub tilt_ok: bool,
    pub fold_ok: bool,
    pub heat_ok: bool,
    pub memory_ok: bool,
    pub position_ok: bool,
}

impl Default for MirrorMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorMotor {
    pub fn new() -> Self {
        Self {
            tilt_ok: true,
            fold_ok: true,
            heat_ok: true,
            memory_ok: true,
            position_ok: true,
        }
    }

    pub fn adjustment_ok(&self) -> bool {
        self.tilt_ok && self.fold_ok && self.position_ok
    }

    pub fn features_ok(&self) -> bool {
        self.heat_ok && self.memory_ok
    }

    pub fn all_ok(&self) -> bool {
        self.adjustment_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.tilt_ok || !self.fold_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tilt_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjustment() {
        let c = MirrorMotor::new();
        assert!(c.adjustment_ok());
    }

    #[test]
    fn test_features() {
        let c = MirrorMotor::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MirrorMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = MirrorMotor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_tilt() {
        let mut c = MirrorMotor::new();
        c.tilt_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = MirrorMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
