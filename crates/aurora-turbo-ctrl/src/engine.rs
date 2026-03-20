/// Turbocharger control: boost pressure, wastegate, compressor maps
/// Phase 191

#[derive(Debug, Clone)]
pub struct TurboSystem {
    pub boost_psi: f64,
    pub target_boost_psi: f64,
    pub wastegate_duty_pct: f64,
    pub shaft_rpm: f64,
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,
}

impl Default for TurboSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TurboSystem {
    pub fn new() -> Self {
        Self {
            boost_psi: 0.0,
            target_boost_psi: 14.7,
            wastegate_duty_pct: 0.0,
            shaft_rpm: 0.0,
            inlet_temp_c: 25.0,
            outlet_temp_c: 25.0,
        }
    }

    pub fn boost_error_psi(&self) -> f64 {
        self.boost_psi - self.target_boost_psi
    }

    pub fn on_boost(&self) -> bool {
        self.boost_psi > 1.0
    }

    pub fn over_boost(&self) -> bool {
        self.boost_psi > self.target_boost_psi * 1.15
    }

    pub fn compressor_ratio(&self) -> f64 {
        if self.inlet_temp_c.abs() < 0.01 {
            return 1.0;
        }
        (self.outlet_temp_c + 273.15) / (self.inlet_temp_c + 273.15)
    }

    pub fn shaft_speed_ok(&self) -> bool {
        self.shaft_rpm < 200_000.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if self.over_boost() {
            score -= 30.0;
        }
        if !self.shaft_speed_ok() {
            score -= 40.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_boost() {
        let t = TurboSystem::new();
        assert!(!t.on_boost());
    }

    #[test]
    fn test_on_boost() {
        let mut t = TurboSystem::new();
        t.boost_psi = 10.0;
        assert!(t.on_boost());
    }

    #[test]
    fn test_over_boost() {
        let mut t = TurboSystem::new();
        t.boost_psi = 20.0;
        assert!(t.over_boost());
    }

    #[test]
    fn test_shaft_ok() {
        let t = TurboSystem::new();
        assert!(t.shaft_speed_ok());
    }

    #[test]
    fn test_compressor_ratio() {
        let mut t = TurboSystem::new();
        t.inlet_temp_c = 25.0;
        t.outlet_temp_c = 100.0;
        assert!(t.compressor_ratio() > 1.0);
    }

    #[test]
    fn test_health() {
        let t = TurboSystem::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
