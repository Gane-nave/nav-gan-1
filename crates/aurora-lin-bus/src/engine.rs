/// lin bus: master, slave, schedule, wake, check
/// Phase 1270

#[derive(Debug, Clone)]
pub struct LinBus {
    pub master_ok: bool,
    pub slave_ok: bool,
    pub schedule_ok: bool,
    pub wake_ok: bool,
    pub check_ok: bool,
}

impl Default for LinBus {
    fn default() -> Self {
        Self::new()
    }
}

impl LinBus {
    pub fn new() -> Self {
        Self {
            master_ok: true,
            slave_ok: true,
            schedule_ok: true,
            wake_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.master_ok && self.slave_ok && self.schedule_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.wake_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.master_ok || !self.slave_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.master_ok {
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
        let c = LinBus::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LinBus::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LinBus::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LinBus::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LinBus::new();
        c.master_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LinBus::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
