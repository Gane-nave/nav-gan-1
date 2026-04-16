/// Quarter glass: fixed, vent, seal, trim
/// Phase 783

#[derive(Debug, Clone)]
pub struct QuarterGlass {
    pub fixed_ok: bool,
    pub vent_ok: bool,
    pub seal_ok: bool,
    pub trim_ok: bool,
    pub clarity_ok: bool,
}

impl Default for QuarterGlass {
    fn default() -> Self {
        Self::new()
    }
}

impl QuarterGlass {
    pub fn new() -> Self {
        Self {
            fixed_ok: true,
            vent_ok: true,
            seal_ok: true,
            trim_ok: true,
            clarity_ok: true,
        }
    }

    pub fn glass_ok(&self) -> bool {
        self.fixed_ok && self.clarity_ok
    }

    pub fn installation_ok(&self) -> bool {
        self.vent_ok && self.seal_ok && self.trim_ok
    }

    pub fn all_ok(&self) -> bool {
        self.glass_ok() && self.installation_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.fixed_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fixed_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glass() {
        let c = QuarterGlass::new();
        assert!(c.glass_ok());
    }

    #[test]
    fn test_installation() {
        let c = QuarterGlass::new();
        assert!(c.installation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QuarterGlass::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = QuarterGlass::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_fixed() {
        let mut c = QuarterGlass::new();
        c.fixed_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = QuarterGlass::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
