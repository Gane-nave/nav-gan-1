/// rpa sys: connect, command, steer, park, complete
/// Phase 1172

#[derive(Debug, Clone)]
pub struct RpaSys {
    pub connect_ok: bool,
    pub command_ok: bool,
    pub steer_ok: bool,
    pub park_ok: bool,
    pub complete_ok: bool,
}

impl Default for RpaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl RpaSys {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            command_ok: true,
            steer_ok: true,
            park_ok: true,
            complete_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.command_ok && self.steer_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.park_ok && self.complete_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.command_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RpaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RpaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RpaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RpaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RpaSys::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RpaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
