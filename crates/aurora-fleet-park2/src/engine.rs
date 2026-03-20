/// fleet park: locate, reserve, enter, exit, log
/// Phase 1427

#[derive(Debug, Clone)]
pub struct FleetPark2 {
    pub locate_ok: bool,
    pub reserve_ok: bool,
    pub enter_ok: bool,
    pub exit_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetPark2 {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetPark2 {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            reserve_ok: true,
            enter_ok: true,
            exit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.locate_ok && self.reserve_ok && self.enter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.exit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.locate_ok || !self.reserve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.locate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FleetPark2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetPark2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetPark2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetPark2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetPark2::new();
        c.locate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetPark2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
