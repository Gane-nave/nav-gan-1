/// Power steering fluid: level, color, leak, foaming
/// Phase 568

#[derive(Debug, Clone)]
pub struct PowerSteerFluid {
    pub level_ok: bool,
    pub color_ok: bool,
    pub leak_free: bool,
    pub foaming: bool,
    pub temp_ok: bool,
}

impl Default for PowerSteerFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerSteerFluid {
    pub fn new() -> Self {
        Self {
            level_ok: true,
            color_ok: true,
            leak_free: true,
            foaming: false,
            temp_ok: true,
        }
    }

    pub fn fluid_ok(&self) -> bool {
        self.level_ok && self.color_ok
    }

    pub fn system_ok(&self) -> bool {
        self.fluid_ok() && self.leak_free && !self.foaming
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && self.temp_ok
    }

    pub fn needs_change(&self) -> bool {
        !self.color_ok || self.foaming
    }

    pub fn health_score(&self) -> f64 {
        if !self.level_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid() {
        let c = PowerSteerFluid::new();
        assert!(c.fluid_ok());
    }

    #[test]
    fn test_system() {
        let c = PowerSteerFluid::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PowerSteerFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_change() {
        let c = PowerSteerFluid::new();
        assert!(!c.needs_change());
    }

    #[test]
    fn test_foaming() {
        let mut c = PowerSteerFluid::new();
        c.foaming = true;
        assert!(c.needs_change());
    }

    #[test]
    fn test_health() {
        let c = PowerSteerFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
