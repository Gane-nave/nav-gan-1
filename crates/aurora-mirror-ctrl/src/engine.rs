/// Mirror control: power fold, heating, auto-dim
/// Phase 538

#[derive(Debug, Clone)]
pub struct MirrorControl {
    pub fold_ok: bool,
    pub heater_ok: bool,
    pub auto_dim_ok: bool,
    pub motor_ok: bool,
    pub glass_ok: bool,
}

impl Default for MirrorControl {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorControl {
    pub fn new() -> Self {
        Self {
            fold_ok: true,
            heater_ok: true,
            auto_dim_ok: true,
            motor_ok: true,
            glass_ok: true,
        }
    }

    pub fn electric_ok(&self) -> bool {
        self.fold_ok && self.motor_ok
    }

    pub fn features_ok(&self) -> bool {
        self.heater_ok && self.auto_dim_ok
    }

    pub fn all_ok(&self) -> bool {
        self.electric_ok() && self.features_ok() && self.glass_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric() {
        let c = MirrorControl::new();
        assert!(c.electric_ok());
    }

    #[test]
    fn test_features() {
        let c = MirrorControl::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MirrorControl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = MirrorControl::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = MirrorControl::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = MirrorControl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
