/// Window seal: run channel, belt molding, water drainage
/// Phase 372

#[derive(Debug, Clone)]
pub struct WindowSeal {
    pub run_channel_ok: bool,
    pub belt_molding_ok: bool,
    pub drainage_ok: bool,
    pub noise_db: f64,
    pub age_years: f64,
}

impl Default for WindowSeal {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowSeal {
    pub fn new() -> Self {
        Self {
            run_channel_ok: true,
            belt_molding_ok: true,
            drainage_ok: true,
            noise_db: 5.0,
            age_years: 2.0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.run_channel_ok && self.belt_molding_ok && self.drainage_ok
    }

    pub fn quiet(&self) -> bool {
        self.noise_db < 10.0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.run_channel_ok || !self.belt_molding_ok
    }

    pub fn water_ok(&self) -> bool {
        self.drainage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.run_channel_ok {
            return 20.0;
        }
        if !self.drainage_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let w = WindowSeal::new();
        assert!(w.all_ok());
    }

    #[test]
    fn test_quiet() {
        let w = WindowSeal::new();
        assert!(w.quiet());
    }

    #[test]
    fn test_no_replace() {
        let w = WindowSeal::new();
        assert!(!w.needs_replacement());
    }

    #[test]
    fn test_water() {
        let w = WindowSeal::new();
        assert!(w.water_ok());
    }

    #[test]
    fn test_bad_channel() {
        let mut w = WindowSeal::new();
        w.run_channel_ok = false;
        assert!(w.needs_replacement());
    }

    #[test]
    fn test_health() {
        let w = WindowSeal::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
