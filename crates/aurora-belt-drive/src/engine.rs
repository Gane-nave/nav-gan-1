/// belt drive: tension, route, align, wear, check
/// Phase 1226

#[derive(Debug, Clone)]
pub struct BeltDrive {
    pub tension_ok: bool,
    pub route_ok: bool,
    pub align_ok: bool,
    pub wear_ok: bool,
    pub check_ok: bool,
}

impl Default for BeltDrive {
    fn default() -> Self {
        Self::new()
    }
}

impl BeltDrive {
    pub fn new() -> Self {
        Self {
            tension_ok: true,
            route_ok: true,
            align_ok: true,
            wear_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.tension_ok && self.route_ok && self.align_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.wear_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.tension_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tension_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BeltDrive::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BeltDrive::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BeltDrive::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BeltDrive::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BeltDrive::new();
        c.tension_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BeltDrive::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
