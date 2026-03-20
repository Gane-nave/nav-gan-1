/// Bumper cover: fascia, grille, fog housing, sensor mount
/// Phase 787

#[derive(Debug, Clone)]
pub struct BumperCover {
    pub fascia_ok: bool,
    pub grille_ok: bool,
    pub fog_ok: bool,
    pub sensor_ok: bool,
    pub paint_ok: bool,
}

impl Default for BumperCover {
    fn default() -> Self {
        Self::new()
    }
}

impl BumperCover {
    pub fn new() -> Self {
        Self {
            fascia_ok: true,
            grille_ok: true,
            fog_ok: true,
            sensor_ok: true,
            paint_ok: true,
        }
    }

    pub fn appearance_ok(&self) -> bool {
        self.fascia_ok && self.paint_ok
    }

    pub fn function_ok(&self) -> bool {
        self.grille_ok && self.fog_ok && self.sensor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.appearance_ok() && self.function_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.fascia_ok || !self.paint_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fascia_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance() {
        let c = BumperCover::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_function() {
        let c = BumperCover::new();
        assert!(c.function_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BumperCover::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = BumperCover::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_fascia() {
        let mut c = BumperCover::new();
        c.fascia_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = BumperCover::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
