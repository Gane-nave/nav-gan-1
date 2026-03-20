/// EGR valve monitoring: position, flow rate, carbon buildup
/// Phase 193

#[derive(Debug, Clone)]
pub struct EgrValve {
    pub position_pct: f64,
    pub target_position_pct: f64,
    pub flow_rate_gps: f64,
    pub carbon_buildup_pct: f64,
    pub temp_c: f64,
}

impl Default for EgrValve {
    fn default() -> Self {
        Self::new()
    }
}

impl EgrValve {
    pub fn new() -> Self {
        Self {
            position_pct: 0.0,
            target_position_pct: 0.0,
            flow_rate_gps: 0.0,
            carbon_buildup_pct: 5.0,
            temp_c: 200.0,
        }
    }

    pub fn position_error(&self) -> f64 {
        (self.position_pct - self.target_position_pct).abs()
    }

    pub fn position_ok(&self) -> bool {
        self.position_error() < 5.0
    }

    pub fn is_open(&self) -> bool {
        self.position_pct > 5.0
    }

    pub fn needs_cleaning(&self) -> bool {
        self.carbon_buildup_pct > 30.0
    }

    pub fn stuck(&self) -> bool {
        self.position_error() > 20.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.position_ok() {
            score -= 25.0;
        }
        if self.needs_cleaning() {
            score -= 25.0;
        }
        if self.stuck() {
            score -= 50.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed() {
        let e = EgrValve::new();
        assert!(!e.is_open());
    }

    #[test]
    fn test_position_ok() {
        let e = EgrValve::new();
        assert!(e.position_ok());
    }

    #[test]
    fn test_no_cleaning() {
        let e = EgrValve::new();
        assert!(!e.needs_cleaning());
    }

    #[test]
    fn test_stuck() {
        let mut e = EgrValve::new();
        e.position_pct = 0.0;
        e.target_position_pct = 50.0;
        assert!(e.stuck());
    }

    #[test]
    fn test_health() {
        let e = EgrValve::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_carbon_buildup() {
        let mut e = EgrValve::new();
        e.carbon_buildup_pct = 40.0;
        assert!(e.needs_cleaning());
    }
}
