/// Instrument cluster: gauges, warning lights, LCD
/// Phase 542

#[derive(Debug, Clone)]
pub struct InstrumentCluster {
    pub speedometer_ok: bool,
    pub tachometer_ok: bool,
    pub fuel_gauge_ok: bool,
    pub lcd_ok: bool,
    pub backlight_ok: bool,
}

impl Default for InstrumentCluster {
    fn default() -> Self {
        Self::new()
    }
}

impl InstrumentCluster {
    pub fn new() -> Self {
        Self {
            speedometer_ok: true,
            tachometer_ok: true,
            fuel_gauge_ok: true,
            lcd_ok: true,
            backlight_ok: true,
        }
    }

    pub fn gauges_ok(&self) -> bool {
        self.speedometer_ok && self.tachometer_ok && self.fuel_gauge_ok
    }

    pub fn display_ok(&self) -> bool {
        self.lcd_ok && self.backlight_ok
    }

    pub fn all_ok(&self) -> bool {
        self.gauges_ok() && self.display_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.speedometer_ok || !self.lcd_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.speedometer_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauges() {
        let c = InstrumentCluster::new();
        assert!(c.gauges_ok());
    }

    #[test]
    fn test_display() {
        let c = InstrumentCluster::new();
        assert!(c.display_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InstrumentCluster::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = InstrumentCluster::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_speedo() {
        let mut c = InstrumentCluster::new();
        c.speedometer_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = InstrumentCluster::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
