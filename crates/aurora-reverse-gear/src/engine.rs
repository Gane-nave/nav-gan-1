/// Reverse gear: idler gear, reverse light switch, engagement
/// Phase 469

#[derive(Debug, Clone)]
pub struct ReverseGear {
    pub engagement_ok: bool,
    pub switch_ok: bool,
    pub idler_ok: bool,
    pub noise_ok: bool,
    pub interlock_ok: bool,
}

impl Default for ReverseGear {
    fn default() -> Self {
        Self::new()
    }
}

impl ReverseGear {
    pub fn new() -> Self {
        Self {
            engagement_ok: true,
            switch_ok: true,
            idler_ok: true,
            noise_ok: true,
            interlock_ok: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.engagement_ok && self.switch_ok && self.idler_ok && self.noise_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.switch_ok && self.interlock_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.engagement_ok || !self.idler_ok
    }

    pub fn light_ok(&self) -> bool {
        self.switch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.engagement_ok {
            return 10.0;
        }
        if !self.idler_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let r = ReverseGear::new();
        assert!(r.all_ok());
    }

    #[test]
    fn test_safety() {
        let r = ReverseGear::new();
        assert!(r.safety_ok());
    }

    #[test]
    fn test_no_service() {
        let r = ReverseGear::new();
        assert!(!r.needs_service());
    }

    #[test]
    fn test_light() {
        let r = ReverseGear::new();
        assert!(r.light_ok());
    }

    #[test]
    fn test_bad_engage() {
        let mut r = ReverseGear::new();
        r.engagement_ok = false;
        assert!(r.needs_service());
    }

    #[test]
    fn test_health() {
        let r = ReverseGear::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
