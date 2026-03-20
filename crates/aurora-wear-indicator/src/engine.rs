/// Wear indicator: brake pad, tire tread, belt, wiper
/// Phase 818

#[derive(Debug, Clone)]
pub struct WearIndicator {
    pub pad_ok: bool,
    pub tread_ok: bool,
    pub belt_ok: bool,
    pub wiper_ok: bool,
    pub sensor_ok: bool,
}

impl Default for WearIndicator {
    fn default() -> Self {
        Self::new()
    }
}

impl WearIndicator {
    pub fn new() -> Self {
        Self {
            pad_ok: true,
            tread_ok: true,
            belt_ok: true,
            wiper_ok: true,
            sensor_ok: true,
        }
    }

    pub fn safety_ok(&self) -> bool {
        self.pad_ok && self.tread_ok
    }

    pub fn comfort_ok(&self) -> bool {
        self.belt_ok && self.wiper_ok && self.sensor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.safety_ok() && self.comfort_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.pad_ok || !self.tread_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pad_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety() {
        let c = WearIndicator::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_comfort() {
        let c = WearIndicator::new();
        assert!(c.comfort_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WearIndicator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WearIndicator::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_pad() {
        let mut c = WearIndicator::new();
        c.pad_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WearIndicator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
