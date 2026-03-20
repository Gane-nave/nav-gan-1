/// Axle sensor: wheel speed, rotation direction, axle load
/// Phase 329

#[derive(Debug, Clone)]
pub struct AxleSensor {
    pub speed_kmh: f64,
    pub forward: bool,
    pub load_kg: f64,
    pub max_load_kg: f64,
    pub sensor_ok: bool,
}

impl Default for AxleSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl AxleSensor {
    pub fn new() -> Self {
        Self {
            speed_kmh: 60.0,
            forward: true,
            load_kg: 500.0,
            max_load_kg: 1200.0,
            sensor_ok: true,
        }
    }

    pub fn moving(&self) -> bool {
        self.speed_kmh > 1.0
    }

    pub fn load_ok(&self) -> bool {
        self.load_kg <= self.max_load_kg
    }

    pub fn overloaded(&self) -> bool {
        self.load_kg > self.max_load_kg
    }

    pub fn load_pct(&self) -> f64 {
        if self.max_load_kg <= 0.0 {
            return 0.0;
        }
        (self.load_kg / self.max_load_kg * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if self.overloaded() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving() {
        let a = AxleSensor::new();
        assert!(a.moving());
    }

    #[test]
    fn test_load_ok() {
        let a = AxleSensor::new();
        assert!(a.load_ok());
    }

    #[test]
    fn test_not_overloaded() {
        let a = AxleSensor::new();
        assert!(!a.overloaded());
    }

    #[test]
    fn test_load_pct() {
        let a = AxleSensor::new();
        assert!(a.load_pct() < 50.0);
    }

    #[test]
    fn test_overloaded() {
        let mut a = AxleSensor::new();
        a.load_kg = 1500.0;
        assert!(a.overloaded());
    }

    #[test]
    fn test_health() {
        let a = AxleSensor::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
