/// Satellite link: LEO, MEO, GEO, handover, bandwidth
/// Phase 984

#[derive(Debug, Clone)]
pub struct SatLink {
    pub leo_ok: bool,
    pub meo_ok: bool,
    pub geo_ok: bool,
    pub handover_ok: bool,
    pub bandwidth_ok: bool,
}

impl Default for SatLink {
    fn default() -> Self {
        Self::new()
    }
}

impl SatLink {
    pub fn new() -> Self {
        Self {
            leo_ok: true,
            meo_ok: true,
            geo_ok: true,
            handover_ok: true,
            bandwidth_ok: true,
        }
    }

    pub fn orbit_ok(&self) -> bool {
        self.leo_ok && self.meo_ok && self.geo_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.handover_ok && self.bandwidth_ok
    }

    pub fn all_ok(&self) -> bool {
        self.orbit_ok() && self.quality_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.leo_ok || !self.handover_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.leo_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit() {
        let c = SatLink::new();
        assert!(c.orbit_ok());
    }

    #[test]
    fn test_quality() {
        let c = SatLink::new();
        assert!(c.quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SatLink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = SatLink::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_leo() {
        let mut c = SatLink::new();
        c.leo_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = SatLink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
