/// DPF: diesel particulate filter, soot load, regen
/// Phase 495

#[derive(Debug, Clone)]
pub struct DpfFilter {
    pub soot_load_pct: f64,
    pub max_soot_pct: f64,
    pub regen_active: bool,
    pub ash_load_pct: f64,
    pub pressure_drop_ok: bool,
}

impl Default for DpfFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DpfFilter {
    pub fn new() -> Self {
        Self {
            soot_load_pct: 30.0,
            max_soot_pct: 80.0,
            regen_active: false,
            ash_load_pct: 10.0,
            pressure_drop_ok: true,
        }
    }

    pub fn soot_ok(&self) -> bool {
        self.soot_load_pct < self.max_soot_pct
    }

    pub fn needs_regen(&self) -> bool {
        self.soot_load_pct > 60.0
    }

    pub fn all_ok(&self) -> bool {
        self.soot_ok() && self.pressure_drop_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.ash_load_pct > 90.0
    }

    pub fn health_score(&self) -> f64 {
        if self.ash_load_pct > 90.0 {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soot() {
        let c = DpfFilter::new();
        assert!(c.soot_ok());
    }

    #[test]
    fn test_no_regen() {
        let c = DpfFilter::new();
        assert!(!c.needs_regen());
    }

    #[test]
    fn test_all_ok() {
        let c = DpfFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = DpfFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_high_ash() {
        let mut c = DpfFilter::new();
        c.ash_load_pct = 95.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = DpfFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
