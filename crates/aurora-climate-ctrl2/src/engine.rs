/// climate ctrl: temp, fan, mode, zone, auto
/// Phase 1315

#[derive(Debug, Clone)]
pub struct ClimateCtrl2 {
    pub temp_ok: bool,
    pub fan_ok: bool,
    pub mode_ok: bool,
    pub zone_ok: bool,
    pub auto_ok: bool,
}

impl Default for ClimateCtrl2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ClimateCtrl2 {
    pub fn new() -> Self {
        Self {
            temp_ok: true,
            fan_ok: true,
            mode_ok: true,
            zone_ok: true,
            auto_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.temp_ok && self.fan_ok && self.mode_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.zone_ok && self.auto_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.temp_ok || !self.fan_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.temp_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ClimateCtrl2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ClimateCtrl2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClimateCtrl2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ClimateCtrl2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ClimateCtrl2::new();
        c.temp_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ClimateCtrl2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
