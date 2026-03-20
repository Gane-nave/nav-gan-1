/// Corrosion monitoring: electrochemical, humidity sensor, salt exposure
/// Phase 385

#[derive(Debug, Clone)]
pub struct CorrosionMon {
    pub corrosion_rate_um_yr: f64,
    pub max_rate_um_yr: f64,
    pub humidity_pct: f64,
    pub salt_exposure: bool,
    pub sensor_ok: bool,
}

impl Default for CorrosionMon {
    fn default() -> Self {
        Self::new()
    }
}

impl CorrosionMon {
    pub fn new() -> Self {
        Self {
            corrosion_rate_um_yr: 5.0,
            max_rate_um_yr: 25.0,
            humidity_pct: 45.0,
            salt_exposure: false,
            sensor_ok: true,
        }
    }

    pub fn rate_ok(&self) -> bool {
        self.corrosion_rate_um_yr < self.max_rate_um_yr
    }

    pub fn high_risk(&self) -> bool {
        self.salt_exposure && self.humidity_pct > 70.0
    }

    pub fn needs_attention(&self) -> bool {
        self.corrosion_rate_um_yr > self.max_rate_um_yr * 0.7
    }

    pub fn monitoring_ok(&self) -> bool {
        self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 20.0;
        }
        if !self.rate_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate() {
        let c = CorrosionMon::new();
        assert!(c.rate_ok());
    }

    #[test]
    fn test_no_risk() {
        let c = CorrosionMon::new();
        assert!(!c.high_risk());
    }

    #[test]
    fn test_no_attention() {
        let c = CorrosionMon::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_monitoring() {
        let c = CorrosionMon::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_high_risk() {
        let mut c = CorrosionMon::new();
        c.salt_exposure = true;
        c.humidity_pct = 80.0;
        assert!(c.high_risk());
    }

    #[test]
    fn test_health() {
        let c = CorrosionMon::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
