/// Sound deadening: butyl, foam, mass loaded vinyl
/// Phase 766

#[derive(Debug, Clone)]
pub struct SoundDeadener {
    pub butyl_ok: bool,
    pub foam_ok: bool,
    pub mlv_ok: bool,
    pub coverage_ok: bool,
    pub adhesion_ok: bool,
}

impl Default for SoundDeadener {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundDeadener {
    pub fn new() -> Self {
        Self {
            butyl_ok: true,
            foam_ok: true,
            mlv_ok: true,
            coverage_ok: true,
            adhesion_ok: true,
        }
    }

    pub fn damping_ok(&self) -> bool {
        self.butyl_ok && self.mlv_ok
    }

    pub fn installation_ok(&self) -> bool {
        self.foam_ok && self.coverage_ok && self.adhesion_ok
    }

    pub fn all_ok(&self) -> bool {
        self.damping_ok() && self.installation_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.butyl_ok || !self.adhesion_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.butyl_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damping() {
        let c = SoundDeadener::new();
        assert!(c.damping_ok());
    }

    #[test]
    fn test_installation() {
        let c = SoundDeadener::new();
        assert!(c.installation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SoundDeadener::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SoundDeadener::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_butyl() {
        let mut c = SoundDeadener::new();
        c.butyl_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SoundDeadener::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
