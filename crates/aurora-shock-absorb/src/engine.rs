/// shock absorb: dampen, rebound, adjust, monitor, report
/// Phase 1196

#[derive(Debug, Clone)]
pub struct ShockAbsorb {
    pub dampen_ok: bool,
    pub rebound_ok: bool,
    pub adjust_ok: bool,
    pub monitor_ok: bool,
    pub report_ok: bool,
}

impl Default for ShockAbsorb {
    fn default() -> Self {
        Self::new()
    }
}

impl ShockAbsorb {
    pub fn new() -> Self {
        Self {
            dampen_ok: true,
            rebound_ok: true,
            adjust_ok: true,
            monitor_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.dampen_ok && self.rebound_ok && self.adjust_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.dampen_ok || !self.rebound_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.dampen_ok {
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
        let c = ShockAbsorb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ShockAbsorb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ShockAbsorb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ShockAbsorb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ShockAbsorb::new();
        c.dampen_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ShockAbsorb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
