/// Body control module: lighting, windows, locks, wipers
/// Phase 526

#[derive(Debug, Clone)]
pub struct BodyControlModule {
    pub lighting_ok: bool,
    pub windows_ok: bool,
    pub locks_ok: bool,
    pub wipers_ok: bool,
    pub ecu_ok: bool,
}

impl Default for BodyControlModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyControlModule {
    pub fn new() -> Self {
        Self {
            lighting_ok: true,
            windows_ok: true,
            locks_ok: true,
            wipers_ok: true,
            ecu_ok: true,
        }
    }

    pub fn exterior_ok(&self) -> bool {
        self.lighting_ok && self.wipers_ok
    }

    pub fn interior_ok(&self) -> bool {
        self.windows_ok && self.locks_ok
    }

    pub fn all_ok(&self) -> bool {
        self.exterior_ok() && self.interior_ok() && self.ecu_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.ecu_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ecu_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exterior() {
        let c = BodyControlModule::new();
        assert!(c.exterior_ok());
    }

    #[test]
    fn test_interior() {
        let c = BodyControlModule::new();
        assert!(c.interior_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BodyControlModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BodyControlModule::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ecu_fail() {
        let mut c = BodyControlModule::new();
        c.ecu_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BodyControlModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
