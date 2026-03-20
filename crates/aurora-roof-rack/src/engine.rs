/// Roof rack: cross bars, mounts, load capacity, wind noise
/// Phase 558

#[derive(Debug, Clone)]
pub struct RoofRack {
    pub max_load_kg: f64,
    pub current_load_kg: f64,
    pub mounts_ok: bool,
    pub bars_ok: bool,
    pub wind_strip_ok: bool,
}

impl Default for RoofRack {
    fn default() -> Self {
        Self::new()
    }
}

impl RoofRack {
    pub fn new() -> Self {
        Self {
            max_load_kg: 75.0,
            current_load_kg: 0.0,
            mounts_ok: true,
            bars_ok: true,
            wind_strip_ok: true,
        }
    }

    pub fn load_ok(&self) -> bool {
        self.current_load_kg < self.max_load_kg
    }

    pub fn structural_ok(&self) -> bool {
        self.mounts_ok && self.bars_ok
    }

    pub fn all_ok(&self) -> bool {
        self.load_ok() && self.structural_ok() && self.wind_strip_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.mounts_ok || !self.bars_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mounts_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let c = RoofRack::new();
        assert!(c.load_ok());
    }

    #[test]
    fn test_structural() {
        let c = RoofRack::new();
        assert!(c.structural_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoofRack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RoofRack::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_mounts() {
        let mut c = RoofRack::new();
        c.mounts_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RoofRack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
