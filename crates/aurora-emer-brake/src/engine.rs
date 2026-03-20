/// Emergency braking: AEB, FCW, reaction time, force
/// Phase 840

#[derive(Debug, Clone)]
pub struct EmerBrake {
    pub aeb_ok: bool,
    pub fcw_ok: bool,
    pub reaction_ok: bool,
    pub force_ok: bool,
    pub sensor_ok: bool,
}

impl Default for EmerBrake {
    fn default() -> Self {
        Self::new()
    }
}

impl EmerBrake {
    pub fn new() -> Self {
        Self {
            aeb_ok: true,
            fcw_ok: true,
            reaction_ok: true,
            force_ok: true,
            sensor_ok: true,
        }
    }

    pub fn automation_ok(&self) -> bool {
        self.aeb_ok && self.fcw_ok && self.sensor_ok
    }

    pub fn performance_ok(&self) -> bool {
        self.reaction_ok && self.force_ok
    }

    pub fn all_ok(&self) -> bool {
        self.automation_ok() && self.performance_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.aeb_ok || !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.aeb_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation() {
        let c = EmerBrake::new();
        assert!(c.automation_ok());
    }

    #[test]
    fn test_performance() {
        let c = EmerBrake::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EmerBrake::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EmerBrake::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_aeb() {
        let mut c = EmerBrake::new();
        c.aeb_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EmerBrake::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
