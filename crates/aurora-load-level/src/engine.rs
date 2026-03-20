/// Load leveling: automatic height adjustment under load, self-leveling
/// Phase 346

#[derive(Debug, Clone)]
pub struct LoadLevel {
    pub load_kg: f64,
    pub max_load_kg: f64,
    pub leveled: bool,
    pub pump_ok: bool,
    pub valve_ok: bool,
}

impl Default for LoadLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadLevel {
    pub fn new() -> Self {
        Self {
            load_kg: 200.0,
            max_load_kg: 600.0,
            leveled: true,
            pump_ok: true,
            valve_ok: true,
        }
    }

    pub fn overloaded(&self) -> bool {
        self.load_kg > self.max_load_kg
    }

    pub fn is_level(&self) -> bool {
        self.leveled
    }

    pub fn system_ok(&self) -> bool {
        self.pump_ok && self.valve_ok
    }

    pub fn load_pct(&self) -> f64 {
        if self.max_load_kg <= 0.0 {
            return 0.0;
        }
        (self.load_kg / self.max_load_kg * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.pump_ok {
            return 0.0;
        }
        if !self.valve_ok {
            return 30.0;
        }
        if self.overloaded() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_overloaded() {
        let l = LoadLevel::new();
        assert!(!l.overloaded());
    }

    #[test]
    fn test_level() {
        let l = LoadLevel::new();
        assert!(l.is_level());
    }

    #[test]
    fn test_system() {
        let l = LoadLevel::new();
        assert!(l.system_ok());
    }

    #[test]
    fn test_load_pct() {
        let l = LoadLevel::new();
        assert!(l.load_pct() < 40.0);
    }

    #[test]
    fn test_overloaded() {
        let mut l = LoadLevel::new();
        l.load_kg = 800.0;
        assert!(l.overloaded());
    }

    #[test]
    fn test_health() {
        let l = LoadLevel::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
