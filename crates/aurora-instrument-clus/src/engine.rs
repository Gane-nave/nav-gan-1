/// Instrument cluster: digital display, gauges, warning lights, themes
/// Phase 273

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClusterTheme {
    Classic,
    Sport,
    Eco,
    Minimal,
}

#[derive(Debug, Clone)]
pub struct InstrumentCluster {
    pub theme: ClusterTheme,
    pub brightness_pct: f64,
    pub warning_count: u8,
    pub display_ok: bool,
    pub speed_shown: bool,
    pub rpm_shown: bool,
}

impl Default for InstrumentCluster {
    fn default() -> Self {
        Self::new()
    }
}

impl InstrumentCluster {
    pub fn new() -> Self {
        Self {
            theme: ClusterTheme::Classic,
            brightness_pct: 80.0,
            warning_count: 0,
            display_ok: true,
            speed_shown: true,
            rpm_shown: true,
        }
    }

    pub fn has_warnings(&self) -> bool {
        self.warning_count > 0
    }

    pub fn critical_warnings(&self) -> bool {
        self.warning_count > 3
    }

    pub fn night_mode(&self) -> bool {
        self.brightness_pct < 30.0
    }

    pub fn all_gauges_visible(&self) -> bool {
        self.display_ok && self.speed_shown && self.rpm_shown
    }

    pub fn health_score(&self) -> f64 {
        if !self.display_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_warnings() {
        let i = InstrumentCluster::new();
        assert!(!i.has_warnings());
    }

    #[test]
    fn test_no_critical() {
        let i = InstrumentCluster::new();
        assert!(!i.critical_warnings());
    }

    #[test]
    fn test_not_night() {
        let i = InstrumentCluster::new();
        assert!(!i.night_mode());
    }

    #[test]
    fn test_all_gauges() {
        let i = InstrumentCluster::new();
        assert!(i.all_gauges_visible());
    }

    #[test]
    fn test_warnings() {
        let mut i = InstrumentCluster::new();
        i.warning_count = 2;
        assert!(i.has_warnings());
    }

    #[test]
    fn test_health() {
        let i = InstrumentCluster::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
