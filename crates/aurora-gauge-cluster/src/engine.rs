/// Gauge cluster: speedometer, tachometer, fuel gauge, temp gauge, digital
/// Phase 427

#[derive(Debug, Clone)]
pub struct GaugeCluster {
    pub speedo_ok: bool,
    pub tach_ok: bool,
    pub fuel_ok: bool,
    pub temp_ok: bool,
    pub backlight_ok: bool,
}

impl Default for GaugeCluster {
    fn default() -> Self {
        Self::new()
    }
}

impl GaugeCluster {
    pub fn new() -> Self {
        Self {
            speedo_ok: true,
            tach_ok: true,
            fuel_ok: true,
            temp_ok: true,
            backlight_ok: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.speedo_ok && self.tach_ok && self.fuel_ok && self.temp_ok
    }

    pub fn safety_critical_ok(&self) -> bool {
        self.speedo_ok && self.temp_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.speedo_ok || !self.backlight_ok
    }

    pub fn working_count(&self) -> u8 {
        [self.speedo_ok, self.tach_ok, self.fuel_ok, self.temp_ok]
            .iter()
            .filter(|&&x| x)
            .count() as u8
    }

    pub fn health_score(&self) -> f64 {
        if !self.speedo_ok {
            return 0.0;
        }
        if !self.all_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let g = GaugeCluster::new();
        assert!(g.all_ok());
    }

    #[test]
    fn test_safety() {
        let g = GaugeCluster::new();
        assert!(g.safety_critical_ok());
    }

    #[test]
    fn test_no_service() {
        let g = GaugeCluster::new();
        assert!(!g.needs_service());
    }

    #[test]
    fn test_count() {
        let g = GaugeCluster::new();
        assert_eq!(g.working_count(), 4);
    }

    #[test]
    fn test_speedo_out() {
        let mut g = GaugeCluster::new();
        g.speedo_ok = false;
        assert!(g.needs_service());
    }

    #[test]
    fn test_health() {
        let g = GaugeCluster::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
