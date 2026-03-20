/// Transmission filter: screen, magnet, flow
/// Phase 574

#[derive(Debug, Clone)]
pub struct TransFilter {
    pub screen_ok: bool,
    pub magnet_ok: bool,
    pub flow_ok: bool,
    pub debris_level: f64,
    pub max_debris: f64,
}

impl Default for TransFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TransFilter {
    pub fn new() -> Self {
        Self {
            screen_ok: true,
            magnet_ok: true,
            flow_ok: true,
            debris_level: 10.0,
            max_debris: 50.0,
        }
    }

    pub fn screen_good(&self) -> bool {
        self.screen_ok && self.flow_ok
    }

    pub fn debris_ok(&self) -> bool {
        self.debris_level < self.max_debris
    }

    pub fn all_ok(&self) -> bool {
        self.screen_good() && self.debris_ok() && self.magnet_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.debris_level > self.max_debris || !self.screen_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.screen_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen() {
        let c = TransFilter::new();
        assert!(c.screen_good());
    }

    #[test]
    fn test_debris() {
        let c = TransFilter::new();
        assert!(c.debris_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TransFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_high_debris() {
        let mut c = TransFilter::new();
        c.debris_level = 60.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TransFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
