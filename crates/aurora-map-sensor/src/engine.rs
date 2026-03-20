/// MAP sensor: manifold absolute pressure, boost measurement, load calculation
/// Phase 214

#[derive(Debug, Clone)]
pub struct MapSensor {
    pub pressure_kpa: f64,
    pub barometric_kpa: f64,
    pub voltage: f64,
    pub sensor_ok: bool,
}

impl Default for MapSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl MapSensor {
    pub fn new() -> Self {
        Self {
            pressure_kpa: 95.0,
            barometric_kpa: 101.3,
            voltage: 4.0,
            sensor_ok: true,
        }
    }

    pub fn vacuum_kpa(&self) -> f64 {
        (self.barometric_kpa - self.pressure_kpa).max(0.0)
    }

    pub fn boost_kpa(&self) -> f64 {
        (self.pressure_kpa - self.barometric_kpa).max(0.0)
    }

    pub fn is_boosted(&self) -> bool {
        self.pressure_kpa > self.barometric_kpa
    }

    pub fn engine_load_pct(&self) -> f64 {
        if self.barometric_kpa <= 0.0 {
            return 0.0;
        }
        (self.pressure_kpa / self.barometric_kpa * 100.0).clamp(0.0, 200.0)
    }

    pub fn voltage_ok(&self) -> bool {
        self.voltage > 0.5 && self.voltage < 4.8
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if !self.voltage_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum() {
        let m = MapSensor::new();
        assert!(m.vacuum_kpa() > 5.0);
    }

    #[test]
    fn test_no_boost() {
        let m = MapSensor::new();
        assert!(!m.is_boosted());
    }

    #[test]
    fn test_engine_load() {
        let m = MapSensor::new();
        assert!(m.engine_load_pct() > 90.0);
    }

    #[test]
    fn test_voltage_ok() {
        let m = MapSensor::new();
        assert!(m.voltage_ok());
    }

    #[test]
    fn test_boost() {
        let mut m = MapSensor::new();
        m.pressure_kpa = 150.0;
        assert!(m.is_boosted());
    }

    #[test]
    fn test_health() {
        let m = MapSensor::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
