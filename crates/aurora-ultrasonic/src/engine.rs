/// Ultrasonic sensor: ping, echo, distance, array, filter
/// Phase 1107

#[derive(Debug, Clone)]
pub struct Ultrasonic {
    pub ping_ok: bool,
    pub echo_ok: bool,
    pub distance_ok: bool,
    pub array_ok: bool,
    pub filter_ok: bool,
}

impl Default for Ultrasonic {
    fn default() -> Self {
        Self::new()
    }
}

impl Ultrasonic {
    pub fn new() -> Self {
        Self {
            ping_ok: true,
            echo_ok: true,
            distance_ok: true,
            array_ok: true,
            filter_ok: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.ping_ok && self.echo_ok && self.distance_ok
    }

    pub fn processing_ok(&self) -> bool {
        self.array_ok && self.filter_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.processing_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.ping_ok || !self.echo_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ping_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = Ultrasonic::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_processing() {
        let c = Ultrasonic::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Ultrasonic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = Ultrasonic::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_ping() {
        let mut c = Ultrasonic::new();
        c.ping_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = Ultrasonic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
