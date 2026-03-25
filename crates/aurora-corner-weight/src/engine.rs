/// Corner weight: per-wheel load measurement, balance optimization
/// Phase 347

#[derive(Debug, Clone)]
pub struct CornerWeight {
    pub front_left_kg: f64,
    pub front_right_kg: f64,
    pub rear_left_kg: f64,
    pub rear_right_kg: f64,
    pub sensors_ok: bool,
}

impl Default for CornerWeight {
    fn default() -> Self {
        Self::new()
    }
}

impl CornerWeight {
    pub fn new() -> Self {
        Self {
            front_left_kg: 380.0,
            front_right_kg: 375.0,
            rear_left_kg: 340.0,
            rear_right_kg: 335.0,
            sensors_ok: true,
        }
    }

    pub fn total_kg(&self) -> f64 {
        self.front_left_kg + self.front_right_kg + self.rear_left_kg + self.rear_right_kg
    }

    pub fn front_pct(&self) -> f64 {
        let total = self.total_kg();
        if total <= 0.0 {
            return 0.0;
        }
        (self.front_left_kg + self.front_right_kg) / total * 100.0
    }

    pub fn cross_weight_pct(&self) -> f64 {
        let total = self.total_kg();
        if total <= 0.0 {
            return 0.0;
        }
        (self.front_left_kg + self.rear_right_kg) / total * 100.0
    }

    pub fn balanced(&self) -> bool {
        let cw = self.cross_weight_pct();
        cw > 48.0 && cw < 52.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensors_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        let c = CornerWeight::new();
        assert!((c.total_kg() - 1430.0).abs() < 1.0);
    }

    #[test]
    fn test_front_pct() {
        let c = CornerWeight::new();
        assert!(c.front_pct() > 50.0);
    }

    #[test]
    fn test_cross_weight() {
        let c = CornerWeight::new();
        assert!(c.cross_weight_pct() > 45.0);
    }

    #[test]
    fn test_balanced() {
        let c = CornerWeight::new();
        assert!(c.balanced());
    }

    #[test]
    fn test_no_sensor() {
        let mut c = CornerWeight::new();
        c.sensors_ok = false;
        assert!((c.health_score()).abs() < 0.1);
    }

    #[test]
    fn test_health() {
        let c = CornerWeight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
