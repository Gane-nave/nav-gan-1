/// Seat ventilation: fan, duct, filter, control
/// Phase 749

#[derive(Debug, Clone)]
pub struct SeatCool {
    pub fan_ok: bool,
    pub duct_ok: bool,
    pub filter_ok: bool,
    pub control_ok: bool,
    pub airflow_ok: bool,
}

impl Default for SeatCool {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatCool {
    pub fn new() -> Self {
        Self {
            fan_ok: true,
            duct_ok: true,
            filter_ok: true,
            control_ok: true,
            airflow_ok: true,
        }
    }

    pub fn ventilation_ok(&self) -> bool {
        self.fan_ok && self.duct_ok && self.airflow_ok
    }

    pub fn system_ok(&self) -> bool {
        self.filter_ok && self.control_ok
    }

    pub fn all_ok(&self) -> bool {
        self.ventilation_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.fan_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fan_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ventilation() {
        let c = SeatCool::new();
        assert!(c.ventilation_ok());
    }

    #[test]
    fn test_system() {
        let c = SeatCool::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SeatCool::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SeatCool::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_fan() {
        let mut c = SeatCool::new();
        c.fan_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SeatCool::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
