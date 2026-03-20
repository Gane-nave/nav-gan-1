/// Fragrance dispenser: cartridge, intensity, schedule, filter
/// Phase 901

#[derive(Debug, Clone)]
pub struct Fragrance {
    pub cartridge_ok: bool,
    pub intensity_ok: bool,
    pub schedule_ok: bool,
    pub filter_ok: bool,
    pub level_ok: bool,
}

impl Default for Fragrance {
    fn default() -> Self {
        Self::new()
    }
}

impl Fragrance {
    pub fn new() -> Self {
        Self {
            cartridge_ok: true,
            intensity_ok: true,
            schedule_ok: true,
            filter_ok: true,
            level_ok: true,
        }
    }

    pub fn dispensing_ok(&self) -> bool {
        self.cartridge_ok && self.intensity_ok && self.level_ok
    }

    pub fn management_ok(&self) -> bool {
        self.schedule_ok && self.filter_ok
    }

    pub fn all_ok(&self) -> bool {
        self.dispensing_ok() && self.management_ok()
    }

    pub fn needs_refill(&self) -> bool {
        !self.level_ok || !self.cartridge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.level_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispensing() {
        let c = Fragrance::new();
        assert!(c.dispensing_ok());
    }

    #[test]
    fn test_management() {
        let c = Fragrance::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Fragrance::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refill() {
        let c = Fragrance::new();
        assert!(!c.needs_refill());
    }

    #[test]
    fn test_level() {
        let mut c = Fragrance::new();
        c.level_ok = false;
        assert!(c.needs_refill());
    }

    #[test]
    fn test_health() {
        let c = Fragrance::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
