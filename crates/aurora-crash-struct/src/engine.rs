/// Crash structure: crumple zone, reinforcement, energy absorb
/// Phase 795

#[derive(Debug, Clone)]
pub struct CrashStruct {
    pub crumple_ok: bool,
    pub reinforcement_ok: bool,
    pub absorber_ok: bool,
    pub intrusion_ok: bool,
    pub weld_ok: bool,
}

impl Default for CrashStruct {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashStruct {
    pub fn new() -> Self {
        Self {
            crumple_ok: true,
            reinforcement_ok: true,
            absorber_ok: true,
            intrusion_ok: true,
            weld_ok: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.crumple_ok && self.absorber_ok && self.intrusion_ok
    }

    pub fn integrity_ok(&self) -> bool {
        self.reinforcement_ok && self.weld_ok
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.integrity_ok()
    }

    pub fn needs_inspection(&self) -> bool {
        !self.crumple_ok || !self.weld_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.crumple_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = CrashStruct::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_integrity() {
        let c = CrashStruct::new();
        assert!(c.integrity_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CrashStruct::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_inspect() {
        let c = CrashStruct::new();
        assert!(!c.needs_inspection());
    }

    #[test]
    fn test_crumple() {
        let mut c = CrashStruct::new();
        c.crumple_ok = false;
        assert!(c.needs_inspection());
    }

    #[test]
    fn test_health() {
        let c = CrashStruct::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
