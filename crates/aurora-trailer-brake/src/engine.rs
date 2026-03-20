/// Trailer brake controller: gain, timing, sync, output
/// Phase 851

#[derive(Debug, Clone)]
pub struct TrailerBrake {
    pub gain_ok: bool,
    pub timing_ok: bool,
    pub sync_ok: bool,
    pub output_ok: bool,
    pub wiring_ok: bool,
}

impl Default for TrailerBrake {
    fn default() -> Self {
        Self::new()
    }
}

impl TrailerBrake {
    pub fn new() -> Self {
        Self {
            gain_ok: true,
            timing_ok: true,
            sync_ok: true,
            output_ok: true,
            wiring_ok: true,
        }
    }

    pub fn control_ok(&self) -> bool {
        self.gain_ok && self.timing_ok && self.sync_ok
    }

    pub fn electrical_ok(&self) -> bool {
        self.output_ok && self.wiring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.control_ok() && self.electrical_ok()
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.gain_ok || !self.sync_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gain_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control() {
        let c = TrailerBrake::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_electrical() {
        let c = TrailerBrake::new();
        assert!(c.electrical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrailerBrake::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let c = TrailerBrake::new();
        assert!(!c.needs_adjustment());
    }

    #[test]
    fn test_gain() {
        let mut c = TrailerBrake::new();
        c.gain_ok = false;
        assert!(c.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let c = TrailerBrake::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
