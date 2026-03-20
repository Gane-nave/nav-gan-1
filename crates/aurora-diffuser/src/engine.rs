/// Rear diffuser: underbody airflow, ground effect, expansion ratio
/// Phase 352

#[derive(Debug, Clone)]
pub struct Diffuser {
    pub expansion_ratio: f64,
    pub angle_deg: f64,
    pub channel_count: u8,
    pub intact: bool,
    pub effective: bool,
}

impl Default for Diffuser {
    fn default() -> Self {
        Self::new()
    }
}

impl Diffuser {
    pub fn new() -> Self {
        Self {
            expansion_ratio: 2.5,
            angle_deg: 12.0,
            channel_count: 5,
            intact: true,
            effective: true,
        }
    }

    pub fn ratio_ok(&self) -> bool {
        self.expansion_ratio > 1.5 && self.expansion_ratio < 4.0
    }

    pub fn angle_ok(&self) -> bool {
        self.angle_deg > 5.0 && self.angle_deg < 20.0
    }

    pub fn all_ok(&self) -> bool {
        self.intact && self.ratio_ok() && self.angle_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.intact
    }

    pub fn health_score(&self) -> f64 {
        if !self.intact {
            return 0.0;
        }
        if !self.effective {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio() {
        let d = Diffuser::new();
        assert!(d.ratio_ok());
    }

    #[test]
    fn test_angle() {
        let d = Diffuser::new();
        assert!(d.angle_ok());
    }

    #[test]
    fn test_all_ok() {
        let d = Diffuser::new();
        assert!(d.all_ok());
    }

    #[test]
    fn test_no_service() {
        let d = Diffuser::new();
        assert!(!d.needs_service());
    }

    #[test]
    fn test_broken() {
        let mut d = Diffuser::new();
        d.intact = false;
        assert!(d.needs_service());
    }

    #[test]
    fn test_health() {
        let d = Diffuser::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
