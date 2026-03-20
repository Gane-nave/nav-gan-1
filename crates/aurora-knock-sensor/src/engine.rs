/// Knock sensor: detonation detection, timing retard, frequency analysis
/// Phase 308

#[derive(Debug, Clone)]
pub struct KnockSensor {
    pub knock_detected: bool,
    pub intensity: f64,
    pub retard_deg: f64,
    pub frequency_khz: f64,
    pub sensor_ok: bool,
}

impl Default for KnockSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl KnockSensor {
    pub fn new() -> Self {
        Self {
            knock_detected: false,
            intensity: 0.0,
            retard_deg: 0.0,
            frequency_khz: 6.0,
            sensor_ok: true,
        }
    }

    pub fn knocking(&self) -> bool {
        self.knock_detected && self.intensity > 0.3
    }

    pub fn severe_knock(&self) -> bool {
        self.knock_detected && self.intensity > 0.7
    }

    pub fn timing_retarded(&self) -> bool {
        self.retard_deg > 1.0
    }

    pub fn needs_attention(&self) -> bool {
        self.retard_deg > 5.0 || self.severe_knock()
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if self.severe_knock() {
            return 30.0;
        }
        if self.knocking() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_knock() {
        let k = KnockSensor::new();
        assert!(!k.knocking());
    }

    #[test]
    fn test_no_severe() {
        let k = KnockSensor::new();
        assert!(!k.severe_knock());
    }

    #[test]
    fn test_no_retard() {
        let k = KnockSensor::new();
        assert!(!k.timing_retarded());
    }

    #[test]
    fn test_no_attention() {
        let k = KnockSensor::new();
        assert!(!k.needs_attention());
    }

    #[test]
    fn test_knock() {
        let mut k = KnockSensor::new();
        k.knock_detected = true;
        k.intensity = 0.5;
        assert!(k.knocking());
    }

    #[test]
    fn test_health() {
        let k = KnockSensor::new();
        assert!((k.health_score() - 100.0).abs() < 0.1);
    }
}
