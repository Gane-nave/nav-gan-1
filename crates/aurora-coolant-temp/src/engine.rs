/// Coolant temperature sensor: engine temp monitoring, thermostat status
/// Phase 216

#[derive(Debug, Clone)]
pub struct CoolantTempSensor {
    pub temp_c: f64,
    pub thermostat_open: bool,
    pub fan_on: bool,
    pub sensor_voltage: f64,
    pub sensor_ok: bool,
}

impl Default for CoolantTempSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CoolantTempSensor {
    pub fn new() -> Self {
        Self {
            temp_c: 90.0,
            thermostat_open: true,
            fan_on: false,
            sensor_voltage: 2.0,
            sensor_ok: true,
        }
    }

    pub fn operating_temp(&self) -> bool {
        (80.0..=105.0).contains(&self.temp_c)
    }

    pub fn overheating(&self) -> bool {
        self.temp_c > 110.0
    }

    pub fn cold(&self) -> bool {
        self.temp_c < 60.0
    }

    pub fn fan_needed(&self) -> bool {
        self.temp_c > 95.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if self.overheating() {
            return 20.0;
        }
        if !self.operating_temp() {
            return 70.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operating_temp() {
        let c = CoolantTempSensor::new();
        assert!(c.operating_temp());
    }

    #[test]
    fn test_not_overheating() {
        let c = CoolantTempSensor::new();
        assert!(!c.overheating());
    }

    #[test]
    fn test_not_cold() {
        let c = CoolantTempSensor::new();
        assert!(!c.cold());
    }

    #[test]
    fn test_no_fan_needed() {
        let c = CoolantTempSensor::new();
        assert!(!c.fan_needed());
    }

    #[test]
    fn test_overheating() {
        let mut c = CoolantTempSensor::new();
        c.temp_c = 120.0;
        assert!(c.overheating());
    }

    #[test]
    fn test_health() {
        let c = CoolantTempSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
