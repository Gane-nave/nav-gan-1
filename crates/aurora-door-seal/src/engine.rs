/// Door seal: weatherstrip condition, compression set, water ingress
/// Phase 371

#[derive(Debug, Clone)]
pub struct DoorSeal {
    pub compression_pct: f64,
    pub water_tight: bool,
    pub air_tight: bool,
    pub age_years: f64,
    pub cracked: bool,
}

impl Default for DoorSeal {
    fn default() -> Self {
        Self::new()
    }
}

impl DoorSeal {
    pub fn new() -> Self {
        Self {
            compression_pct: 80.0,
            water_tight: true,
            air_tight: true,
            age_years: 2.0,
            cracked: false,
        }
    }

    pub fn effective(&self) -> bool {
        self.water_tight && self.air_tight && !self.cracked
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || self.compression_pct < 40.0
    }

    pub fn aging(&self) -> bool {
        self.age_years > 7.0
    }

    pub fn seal_quality(&self) -> f64 {
        if self.cracked {
            return 0.0;
        }
        self.compression_pct
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 0.0;
        }
        if !self.water_tight {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective() {
        let d = DoorSeal::new();
        assert!(d.effective());
    }

    #[test]
    fn test_no_replace() {
        let d = DoorSeal::new();
        assert!(!d.needs_replacement());
    }

    #[test]
    fn test_not_aging() {
        let d = DoorSeal::new();
        assert!(!d.aging());
    }

    #[test]
    fn test_quality() {
        let d = DoorSeal::new();
        assert!(d.seal_quality() > 70.0);
    }

    #[test]
    fn test_cracked() {
        let mut d = DoorSeal::new();
        d.cracked = true;
        assert!(d.needs_replacement());
    }

    #[test]
    fn test_health() {
        let d = DoorSeal::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
