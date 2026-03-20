/// solar panel: harvest, track, optimize, clean, report
/// Phase 1144

#[derive(Debug, Clone)]
pub struct SolarPanel {
    pub harvest_ok: bool,
    pub track_ok: bool,
    pub optimize_ok: bool,
    pub clean_ok: bool,
    pub report_ok: bool,
}

impl Default for SolarPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl SolarPanel {
    pub fn new() -> Self {
        Self {
            harvest_ok: true,
            track_ok: true,
            optimize_ok: true,
            clean_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.harvest_ok && self.track_ok && self.optimize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clean_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.harvest_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.harvest_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SolarPanel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SolarPanel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SolarPanel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SolarPanel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SolarPanel::new();
        c.harvest_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SolarPanel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
