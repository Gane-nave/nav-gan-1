/// Instrument cluster ECU: gauges, warnings, display
/// Phase 713

#[derive(Debug, Clone)]
pub struct ClusterEcu {
    pub gauges_ok: bool,
    pub warnings_ok: bool,
    pub display_ok: bool,
    pub backlight_ok: bool,
    pub comm_ok: bool,
}

impl Default for ClusterEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterEcu {
    pub fn new() -> Self {
        Self {
            gauges_ok: true,
            warnings_ok: true,
            display_ok: true,
            backlight_ok: true,
            comm_ok: true,
        }
    }

    pub fn instruments_ok(&self) -> bool {
        self.gauges_ok && self.warnings_ok
    }

    pub fn visual_ok(&self) -> bool {
        self.display_ok && self.backlight_ok
    }

    pub fn all_ok(&self) -> bool {
        self.instruments_ok() && self.visual_ok() && self.comm_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.gauges_ok || !self.display_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gauges_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruments() {
        let c = ClusterEcu::new();
        assert!(c.instruments_ok());
    }

    #[test]
    fn test_visual() {
        let c = ClusterEcu::new();
        assert!(c.visual_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClusterEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ClusterEcu::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_gauges() {
        let mut c = ClusterEcu::new();
        c.gauges_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ClusterEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
