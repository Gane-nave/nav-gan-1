/// tire press: sense, transmit, warn, calibrate, log
/// Phase 1210

#[derive(Debug, Clone)]
pub struct TirePress {
    pub sense_ok: bool,
    pub transmit_ok: bool,
    pub warn_ok: bool,
    pub calibrate_ok: bool,
    pub log_ok: bool,
}

impl Default for TirePress {
    fn default() -> Self {
        Self::new()
    }
}

impl TirePress {
    pub fn new() -> Self {
        Self {
            sense_ok: true,
            transmit_ok: true,
            warn_ok: true,
            calibrate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sense_ok && self.transmit_ok && self.warn_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.calibrate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sense_ok || !self.transmit_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sense_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TirePress::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TirePress::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TirePress::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TirePress::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TirePress::new();
        c.sense_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TirePress::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
