/// Ride height: static/dynamic clearance, speed-based adjustment
/// Phase 345

#[derive(Debug, Clone)]
pub struct RideHeight {
    pub front_mm: f64,
    pub rear_mm: f64,
    pub min_mm: f64,
    pub speed_adjusted: bool,
    pub sensors_ok: bool,
}

impl Default for RideHeight {
    fn default() -> Self {
        Self::new()
    }
}

impl RideHeight {
    pub fn new() -> Self {
        Self {
            front_mm: 150.0,
            rear_mm: 155.0,
            min_mm: 100.0,
            speed_adjusted: false,
            sensors_ok: true,
        }
    }

    pub fn clearance_ok(&self) -> bool {
        self.front_mm > self.min_mm && self.rear_mm > self.min_mm
    }

    pub fn balanced(&self) -> bool {
        (self.front_mm - self.rear_mm).abs() < 15.0
    }

    pub fn too_low(&self) -> bool {
        self.front_mm < self.min_mm || self.rear_mm < self.min_mm
    }

    pub fn rake_mm(&self) -> f64 {
        self.rear_mm - self.front_mm
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensors_ok {
            return 0.0;
        }
        if self.too_low() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clearance() {
        let r = RideHeight::new();
        assert!(r.clearance_ok());
    }

    #[test]
    fn test_balanced() {
        let r = RideHeight::new();
        assert!(r.balanced());
    }

    #[test]
    fn test_not_low() {
        let r = RideHeight::new();
        assert!(!r.too_low());
    }

    #[test]
    fn test_rake() {
        let r = RideHeight::new();
        assert!((r.rake_mm() - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_low() {
        let mut r = RideHeight::new();
        r.front_mm = 80.0;
        assert!(r.too_low());
    }

    #[test]
    fn test_health() {
        let r = RideHeight::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
