/// Airbag ECU: crash sensor, squib, pretensioner
/// Phase 711

#[derive(Debug, Clone)]
pub struct AirbagEcu {
    pub crash_sensor_ok: bool,
    pub squib_ok: bool,
    pub pretensioner_ok: bool,
    pub indicator_ok: bool,
    pub comm_ok: bool,
}

impl Default for AirbagEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl AirbagEcu {
    pub fn new() -> Self {
        Self {
            crash_sensor_ok: true,
            squib_ok: true,
            pretensioner_ok: true,
            indicator_ok: true,
            comm_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.crash_sensor_ok && self.comm_ok
    }

    pub fn deployment_ok(&self) -> bool {
        self.squib_ok && self.pretensioner_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.deployment_ok() && self.indicator_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.crash_sensor_ok || !self.squib_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.crash_sensor_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = AirbagEcu::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_deployment() {
        let c = AirbagEcu::new();
        assert!(c.deployment_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AirbagEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AirbagEcu::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_crash() {
        let mut c = AirbagEcu::new();
        c.crash_sensor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AirbagEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
