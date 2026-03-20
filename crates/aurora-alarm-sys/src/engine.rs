/// alarm sys: arm, detect, trigger, notify, disarm
/// Phase 1287

#[derive(Debug, Clone)]
pub struct AlarmSys {
    pub arm_ok: bool,
    pub detect_ok: bool,
    pub trigger_ok: bool,
    pub notify_ok: bool,
    pub disarm_ok: bool,
}

impl Default for AlarmSys {
    fn default() -> Self {
        Self::new()
    }
}

impl AlarmSys {
    pub fn new() -> Self {
        Self {
            arm_ok: true,
            detect_ok: true,
            trigger_ok: true,
            notify_ok: true,
            disarm_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.arm_ok && self.detect_ok && self.trigger_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.notify_ok && self.disarm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.arm_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.arm_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AlarmSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AlarmSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AlarmSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AlarmSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AlarmSys::new();
        c.arm_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AlarmSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
