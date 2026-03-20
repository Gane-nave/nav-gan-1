/// ground net: connect, measure, protect, isolate, log
/// Phase 1369

#[derive(Debug, Clone)]
pub struct GroundNet {
    pub connect_ok: bool,
    pub measure_ok: bool,
    pub protect_ok: bool,
    pub isolate_ok: bool,
    pub log_ok: bool,
}

impl Default for GroundNet {
    fn default() -> Self {
        Self::new()
    }
}

impl GroundNet {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            measure_ok: true,
            protect_ok: true,
            isolate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.measure_ok && self.protect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.isolate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = GroundNet::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GroundNet::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GroundNet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GroundNet::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GroundNet::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GroundNet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
