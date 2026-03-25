/// Axle shaft: half-shaft, spline wear, bearing, seal
/// Phase 460

#[derive(Debug, Clone)]
pub struct AxleShaft {
    pub spline_ok: bool,
    pub bearing_ok: bool,
    pub seal_ok: bool,
    pub vibration_ok: bool,
    pub play_mm: f64,
}

impl Default for AxleShaft {
    fn default() -> Self {
        Self::new()
    }
}

impl AxleShaft {
    pub fn new() -> Self {
        Self {
            spline_ok: true,
            bearing_ok: true,
            seal_ok: true,
            vibration_ok: true,
            play_mm: 0.1,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.spline_ok && self.bearing_ok && self.seal_ok && self.vibration_ok
    }

    pub fn play_ok(&self) -> bool {
        self.play_mm < 1.0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.spline_ok || !self.bearing_ok
    }

    pub fn leak_risk(&self) -> bool {
        !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spline_ok {
            return 0.0;
        }
        if !self.bearing_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let a = AxleShaft::new();
        assert!(a.all_ok());
    }

    #[test]
    fn test_play() {
        let a = AxleShaft::new();
        assert!(a.play_ok());
    }

    #[test]
    fn test_no_replace() {
        let a = AxleShaft::new();
        assert!(!a.needs_replacement());
    }

    #[test]
    fn test_no_leak() {
        let a = AxleShaft::new();
        assert!(!a.leak_risk());
    }

    #[test]
    fn test_worn_spline() {
        let mut a = AxleShaft::new();
        a.spline_ok = false;
        assert!(a.needs_replacement());
    }

    #[test]
    fn test_health() {
        let a = AxleShaft::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
