/// Body height sensor: air suspension level, ground clearance
/// Phase 344

#[derive(Debug, Clone)]
pub struct BodyHeight {
    pub height_mm: f64,
    pub target_mm: f64,
    pub sensor_ok: bool,
    pub air_spring_ok: bool,
    pub compressor_ok: bool,
}

impl Default for BodyHeight {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyHeight {
    pub fn new() -> Self {
        Self {
            height_mm: 180.0,
            target_mm: 180.0,
            sensor_ok: true,
            air_spring_ok: true,
            compressor_ok: true,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.height_mm - self.target_mm).abs() < 10.0
    }

    pub fn raised(&self) -> bool {
        self.height_mm > self.target_mm + 20.0
    }

    pub fn lowered(&self) -> bool {
        self.height_mm < self.target_mm - 20.0
    }

    pub fn system_ok(&self) -> bool {
        self.sensor_ok && self.air_spring_ok && self.compressor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.air_spring_ok {
            return 0.0;
        }
        if !self.compressor_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let b = BodyHeight::new();
        assert!(b.at_target());
    }

    #[test]
    fn test_not_raised() {
        let b = BodyHeight::new();
        assert!(!b.raised());
    }

    #[test]
    fn test_not_lowered() {
        let b = BodyHeight::new();
        assert!(!b.lowered());
    }

    #[test]
    fn test_system_ok() {
        let b = BodyHeight::new();
        assert!(b.system_ok());
    }

    #[test]
    fn test_raised() {
        let mut b = BodyHeight::new();
        b.height_mm = 220.0;
        assert!(b.raised());
    }

    #[test]
    fn test_health() {
        let b = BodyHeight::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
