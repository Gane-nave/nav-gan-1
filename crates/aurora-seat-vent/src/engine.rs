/// Seat ventilation: cooling fans, airflow control, temperature zones
/// Phase 252

#[derive(Debug, Clone)]
pub struct SeatVentilation {
    pub level: u8,
    pub max_level: u8,
    pub fan_ok: bool,
    pub temp_c: f64,
    pub active: bool,
}

impl Default for SeatVentilation {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatVentilation {
    pub fn new() -> Self {
        Self {
            level: 0,
            max_level: 3,
            fan_ok: true,
            temp_c: 25.0,
            active: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active && self.level > 0
    }

    pub fn level_pct(&self) -> f64 {
        if self.max_level == 0 {
            return 0.0;
        }
        self.level as f64 / self.max_level as f64 * 100.0
    }

    pub fn should_activate(&self, cabin_temp_c: f64) -> bool {
        cabin_temp_c > 28.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.fan_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_active() {
        let s = SeatVentilation::new();
        assert!(!s.is_active());
    }

    #[test]
    fn test_level_zero() {
        let s = SeatVentilation::new();
        assert!((s.level_pct()).abs() < 0.1);
    }

    #[test]
    fn test_no_activate() {
        let s = SeatVentilation::new();
        assert!(!s.should_activate(22.0));
    }

    #[test]
    fn test_fan_ok() {
        let s = SeatVentilation::new();
        assert!(s.fan_ok);
    }

    #[test]
    fn test_hot_activate() {
        let s = SeatVentilation::new();
        assert!(s.should_activate(35.0));
    }

    #[test]
    fn test_health() {
        let s = SeatVentilation::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
