/// Telemetry logger: GPS, accel, brake, speed, export
/// Phase 949

#[derive(Debug, Clone)]
pub struct TelemetryLog {
    pub gps_ok: bool,
    pub accel_ok: bool,
    pub brake_ok: bool,
    pub speed_ok: bool,
    pub export_ok: bool,
}

impl Default for TelemetryLog {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryLog {
    pub fn new() -> Self {
        Self {
            gps_ok: true,
            accel_ok: true,
            brake_ok: true,
            speed_ok: true,
            export_ok: true,
        }
    }

    pub fn capture_ok(&self) -> bool {
        self.gps_ok && self.accel_ok && self.speed_ok
    }

    pub fn output_ok(&self) -> bool {
        self.brake_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.capture_ok() && self.output_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.gps_ok || !self.accel_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gps_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture() {
        let c = TelemetryLog::new();
        assert!(c.capture_ok());
    }

    #[test]
    fn test_output() {
        let c = TelemetryLog::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TelemetryLog::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = TelemetryLog::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_gps() {
        let mut c = TelemetryLog::new();
        c.gps_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = TelemetryLog::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
