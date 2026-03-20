/// ESC module: stability control, yaw sensor, lateral accel
/// Phase 489

#[derive(Debug, Clone)]
pub struct EscModule {
    pub yaw_rate_dps: f64,
    pub lateral_g: f64,
    pub esc_active: bool,
    pub sensor_ok: bool,
    pub ecu_ok: bool,
}

impl Default for EscModule {
    fn default() -> Self {
        Self::new()
    }
}

impl EscModule {
    pub fn new() -> Self {
        Self {
            yaw_rate_dps: 5.0,
            lateral_g: 0.3,
            esc_active: true,
            sensor_ok: true,
            ecu_ok: true,
        }
    }

    pub fn stable(&self) -> bool {
        self.yaw_rate_dps.abs() < 30.0 && self.lateral_g.abs() < 0.8
    }

    pub fn system_ok(&self) -> bool {
        self.sensor_ok && self.ecu_ok
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && self.esc_active
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok || !self.ecu_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ecu_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        let c = EscModule::new();
        assert!(c.stable());
    }

    #[test]
    fn test_system() {
        let c = EscModule::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EscModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EscModule::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ecu_fail() {
        let mut c = EscModule::new();
        c.ecu_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EscModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
