/// Defrost duct: windshield defroster, side window, rear window
/// Phase 445

#[derive(Debug, Clone)]
pub struct DefrostDuct {
    pub windshield_ok: bool,
    pub side_ok: bool,
    pub rear_ok: bool,
    pub airflow_ok: bool,
    pub temp_c: f64,
}

impl Default for DefrostDuct {
    fn default() -> Self {
        Self::new()
    }
}

impl DefrostDuct {
    pub fn new() -> Self {
        Self {
            windshield_ok: true,
            side_ok: true,
            rear_ok: true,
            airflow_ok: true,
            temp_c: 55.0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.windshield_ok && self.side_ok && self.rear_ok && self.airflow_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.windshield_ok && self.airflow_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.windshield_ok || !self.airflow_ok
    }

    pub fn effective(&self) -> bool {
        self.temp_c > 40.0 && self.airflow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.windshield_ok {
            return 0.0;
        }
        if !self.airflow_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let d = DefrostDuct::new();
        assert!(d.all_ok());
    }

    #[test]
    fn test_safety() {
        let d = DefrostDuct::new();
        assert!(d.safety_ok());
    }

    #[test]
    fn test_no_service() {
        let d = DefrostDuct::new();
        assert!(!d.needs_service());
    }

    #[test]
    fn test_effective() {
        let d = DefrostDuct::new();
        assert!(d.effective());
    }

    #[test]
    fn test_blocked() {
        let mut d = DefrostDuct::new();
        d.airflow_ok = false;
        assert!(d.needs_service());
    }

    #[test]
    fn test_health() {
        let d = DefrostDuct::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
