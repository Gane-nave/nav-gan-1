/// Snow plow mount: bracket, pin, hydraulic, blade
/// Phase 846

#[derive(Debug, Clone)]
pub struct SnowPlow {
    pub bracket_ok: bool,
    pub pin_ok: bool,
    pub hydraulic_ok: bool,
    pub blade_ok: bool,
    pub light_ok: bool,
}

impl Default for SnowPlow {
    fn default() -> Self {
        Self::new()
    }
}

impl SnowPlow {
    pub fn new() -> Self {
        Self {
            bracket_ok: true,
            pin_ok: true,
            hydraulic_ok: true,
            blade_ok: true,
            light_ok: true,
        }
    }

    pub fn mounting_ok(&self) -> bool {
        self.bracket_ok && self.pin_ok
    }

    pub fn operation_ok(&self) -> bool {
        self.hydraulic_ok && self.blade_ok && self.light_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mounting_ok() && self.operation_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.hydraulic_ok || !self.blade_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hydraulic_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mounting() {
        let c = SnowPlow::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_operation() {
        let c = SnowPlow::new();
        assert!(c.operation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SnowPlow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SnowPlow::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_hydraulic() {
        let mut c = SnowPlow::new();
        c.hydraulic_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SnowPlow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
