/// Crash structure: crumple zone, energy absorption, integrity
/// Phase 530

#[derive(Debug, Clone)]
pub struct CrashStructure {
    pub deformation_mm: f64,
    pub max_deformation_mm: f64,
    pub integrity_pct: f64,
    pub reinforcement_ok: bool,
    pub corrosion_free: bool,
}

impl Default for CrashStructure {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashStructure {
    pub fn new() -> Self {
        Self {
            deformation_mm: 0.0,
            max_deformation_mm: 50.0,
            integrity_pct: 100.0,
            reinforcement_ok: true,
            corrosion_free: true,
        }
    }

    pub fn deformation_ok(&self) -> bool {
        self.deformation_mm < self.max_deformation_mm
    }

    pub fn structurally_sound(&self) -> bool {
        self.integrity_pct > 90.0 && self.reinforcement_ok
    }

    pub fn all_ok(&self) -> bool {
        self.deformation_ok() && self.structurally_sound() && self.corrosion_free
    }

    pub fn needs_repair(&self) -> bool {
        self.deformation_mm > 0.0 || !self.reinforcement_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.integrity_pct < 90.0 { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deformation() {
        let c = CrashStructure::new();
        assert!(c.deformation_ok());
    }

    #[test]
    fn test_sound() {
        let c = CrashStructure::new();
        assert!(c.structurally_sound());
    }

    #[test]
    fn test_all_ok() {
        let c = CrashStructure::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = CrashStructure::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_damaged() {
        let mut c = CrashStructure::new();
        c.deformation_mm = 60.0;
        assert!(!c.deformation_ok());
    }

    #[test]
    fn test_health() {
        let c = CrashStructure::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
