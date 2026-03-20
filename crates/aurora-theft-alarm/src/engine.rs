/// Theft alarm: sensor, siren, notification, GPS track
/// Phase 843

#[derive(Debug, Clone)]
pub struct TheftAlarm {
    pub sensor_ok: bool,
    pub siren_ok: bool,
    pub notify_ok: bool,
    pub gps_ok: bool,
    pub armed: bool,
}

impl Default for TheftAlarm {
    fn default() -> Self {
        Self::new()
    }
}

impl TheftAlarm {
    pub fn new() -> Self {
        Self {
            sensor_ok: true,
            siren_ok: true,
            notify_ok: true,
            gps_ok: true,
            armed: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.sensor_ok && self.armed
    }

    pub fn response_ok(&self) -> bool {
        self.siren_ok && self.notify_ok && self.gps_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.response_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok || !self.siren_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = TheftAlarm::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_response() {
        let c = TheftAlarm::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TheftAlarm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TheftAlarm::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_sensor() {
        let mut c = TheftAlarm::new();
        c.sensor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TheftAlarm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
