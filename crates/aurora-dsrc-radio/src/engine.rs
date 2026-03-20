/// DSRC radio: V2V, V2I, BSM, SPaT, MAP
/// Phase 981

#[derive(Debug, Clone)]
pub struct DsrcRadio {
    pub v2v_ok: bool,
    pub v2i_ok: bool,
    pub bsm_ok: bool,
    pub spat_ok: bool,
    pub map_ok: bool,
}

impl Default for DsrcRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl DsrcRadio {
    pub fn new() -> Self {
        Self {
            v2v_ok: true,
            v2i_ok: true,
            bsm_ok: true,
            spat_ok: true,
            map_ok: true,
        }
    }

    pub fn vehicle_ok(&self) -> bool {
        self.v2v_ok && self.bsm_ok
    }

    pub fn infrastructure_ok(&self) -> bool {
        self.v2i_ok && self.spat_ok && self.map_ok
    }

    pub fn all_ok(&self) -> bool {
        self.vehicle_ok() && self.infrastructure_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.v2v_ok || !self.v2i_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.v2v_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle() {
        let c = DsrcRadio::new();
        assert!(c.vehicle_ok());
    }

    #[test]
    fn test_infrastructure() {
        let c = DsrcRadio::new();
        assert!(c.infrastructure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DsrcRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = DsrcRadio::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_v2v() {
        let mut c = DsrcRadio::new();
        c.v2v_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = DsrcRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
