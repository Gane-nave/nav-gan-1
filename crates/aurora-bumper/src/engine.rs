/// Bumper: energy absorber, foam, bracket, cover
/// Phase 551

#[derive(Debug, Clone)]
pub struct Bumper {
    pub absorber_ok: bool,
    pub foam_ok: bool,
    pub bracket_ok: bool,
    pub cover_ok: bool,
    pub sensor_ok: bool,
}

impl Default for Bumper {
    fn default() -> Self {
        Self::new()
    }
}

impl Bumper {
    pub fn new() -> Self {
        Self {
            absorber_ok: true,
            foam_ok: true,
            bracket_ok: true,
            cover_ok: true,
            sensor_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.absorber_ok && self.foam_ok && self.bracket_ok
    }

    pub fn appearance_ok(&self) -> bool {
        self.cover_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.appearance_ok() && self.sensor_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.absorber_ok || !self.bracket_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.absorber_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = Bumper::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_appearance() {
        let c = Bumper::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Bumper::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Bumper::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_absorber() {
        let mut c = Bumper::new();
        c.absorber_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Bumper::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
