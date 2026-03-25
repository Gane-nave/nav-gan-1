/// Catalytic converter monitoring: efficiency, temperature, O2 sensors
/// Phase 194

#[derive(Debug, Clone)]
pub struct CatalyticConverter {
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,
    pub efficiency_pct: f64,
    pub light_off: bool,
    pub upstream_o2_v: f64,
    pub downstream_o2_v: f64,
}

impl Default for CatalyticConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalyticConverter {
    pub fn new() -> Self {
        Self {
            inlet_temp_c: 400.0,
            outlet_temp_c: 450.0,
            efficiency_pct: 95.0,
            light_off: true,
            upstream_o2_v: 0.45,
            downstream_o2_v: 0.45,
        }
    }

    pub fn temp_rise_c(&self) -> f64 {
        self.outlet_temp_c - self.inlet_temp_c
    }

    pub fn overheating(&self) -> bool {
        self.outlet_temp_c > 900.0
    }

    pub fn is_efficient(&self) -> bool {
        self.efficiency_pct >= 90.0
    }

    pub fn needs_replacement(&self) -> bool {
        self.efficiency_pct < 70.0
    }

    pub fn o2_sensor_healthy(&self) -> bool {
        self.upstream_o2_v > 0.1
            && self.upstream_o2_v < 0.9
            && self.downstream_o2_v > 0.1
            && self.downstream_o2_v < 0.9
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.is_efficient() {
            score -= 30.0;
        }
        if self.overheating() {
            score -= 40.0;
        }
        if !self.o2_sensor_healthy() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efficient() {
        let c = CatalyticConverter::new();
        assert!(c.is_efficient());
    }

    #[test]
    fn test_not_overheating() {
        let c = CatalyticConverter::new();
        assert!(!c.overheating());
    }

    #[test]
    fn test_no_replacement() {
        let c = CatalyticConverter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_temp_rise() {
        let c = CatalyticConverter::new();
        assert!((c.temp_rise_c() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_o2_healthy() {
        let c = CatalyticConverter::new();
        assert!(c.o2_sensor_healthy());
    }

    #[test]
    fn test_health() {
        let c = CatalyticConverter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_failing_cat() {
        let mut c = CatalyticConverter::new();
        c.efficiency_pct = 50.0;
        assert!(c.needs_replacement());
    }
}
