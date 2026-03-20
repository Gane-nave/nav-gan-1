/// Fog lamp: front fog, rear fog, beam pattern, switch operation
/// Phase 423

#[derive(Debug, Clone)]
pub struct FogLamp {
    pub front_ok: bool,
    pub rear_ok: bool,
    pub switch_ok: bool,
    pub aim_ok: bool,
    pub lens_ok: bool,
}

impl Default for FogLamp {
    fn default() -> Self {
        Self::new()
    }
}

impl FogLamp {
    pub fn new() -> Self {
        Self {
            front_ok: true,
            rear_ok: true,
            switch_ok: true,
            aim_ok: true,
            lens_ok: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.front_ok && self.rear_ok && self.switch_ok && self.aim_ok
    }

    pub fn functional(&self) -> bool {
        (self.front_ok || self.rear_ok) && self.switch_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.switch_ok || !self.lens_ok
    }

    pub fn compliant(&self) -> bool {
        self.aim_ok && self.lens_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.switch_ok {
            return 20.0;
        }
        if !self.aim_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let f = FogLamp::new();
        assert!(f.all_ok());
    }

    #[test]
    fn test_functional() {
        let f = FogLamp::new();
        assert!(f.functional());
    }

    #[test]
    fn test_no_service() {
        let f = FogLamp::new();
        assert!(!f.needs_service());
    }

    #[test]
    fn test_compliant() {
        let f = FogLamp::new();
        assert!(f.compliant());
    }

    #[test]
    fn test_bad_switch() {
        let mut f = FogLamp::new();
        f.switch_ok = false;
        assert!(f.needs_service());
    }

    #[test]
    fn test_health() {
        let f = FogLamp::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
