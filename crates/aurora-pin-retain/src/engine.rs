/// Pin retention: cotter pin, roll pin, clevis pin, spring pin
/// Phase 406

#[derive(Debug, Clone)]
pub struct PinRetain {
    pub installed: bool,
    pub correct_size: bool,
    pub bent: bool,
    pub corrosion_free: bool,
    pub safety_critical: bool,
}

impl Default for PinRetain {
    fn default() -> Self {
        Self::new()
    }
}

impl PinRetain {
    pub fn new() -> Self {
        Self {
            installed: true,
            correct_size: true,
            bent: false,
            corrosion_free: true,
            safety_critical: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.installed && self.correct_size && !self.bent && self.corrosion_free
    }

    pub fn safe(&self) -> bool {
        self.installed && (!self.safety_critical || self.correct_size)
    }

    pub fn needs_replacement(&self) -> bool {
        !self.installed || self.bent || !self.corrosion_free
    }

    pub fn critical_missing(&self) -> bool {
        self.safety_critical && !self.installed
    }

    pub fn health_score(&self) -> f64 {
        if self.critical_missing() {
            return 0.0;
        }
        if !self.all_ok() {
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
        let p = PinRetain::new();
        assert!(p.all_ok());
    }

    #[test]
    fn test_safe() {
        let p = PinRetain::new();
        assert!(p.safe());
    }

    #[test]
    fn test_no_replace() {
        let p = PinRetain::new();
        assert!(!p.needs_replacement());
    }

    #[test]
    fn test_not_missing() {
        let p = PinRetain::new();
        assert!(!p.critical_missing());
    }

    #[test]
    fn test_missing() {
        let mut p = PinRetain::new();
        p.installed = false;
        assert!(p.critical_missing());
    }

    #[test]
    fn test_health() {
        let p = PinRetain::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
