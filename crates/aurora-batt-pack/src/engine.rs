/// batt pack: configure, monitor, isolate, report, log
/// Phase 1359

#[derive(Debug, Clone)]
pub struct BattPack {
    pub configure_ok: bool,
    pub monitor_ok: bool,
    pub isolate_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for BattPack {
    fn default() -> Self {
        Self::new()
    }
}

impl BattPack {
    pub fn new() -> Self {
        Self {
            configure_ok: true,
            monitor_ok: true,
            isolate_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.configure_ok && self.monitor_ok && self.isolate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.configure_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.configure_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BattPack::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BattPack::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BattPack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BattPack::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BattPack::new();
        c.configure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BattPack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
