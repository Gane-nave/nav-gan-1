/// Exhaust monitoring: emissions, catalytic converter, DPF status
/// Phase 158

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmissionStandard {
    Euro6,
    Euro7,
    Ulev,
    ZeroEmission,
}

#[derive(Debug, Clone)]
pub struct ExhaustSystem {
    pub standard: EmissionStandard,
    pub co2_g_per_km: f64,
    pub nox_mg_per_km: f64,
    pub cat_temp_c: f64,
    pub dpf_load_pct: f64,
    pub exhaust_temp_c: f64,
}

impl Default for ExhaustSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustSystem {
    pub fn new() -> Self {
        Self {
            standard: EmissionStandard::Euro6,
            co2_g_per_km: 120.0,
            nox_mg_per_km: 40.0,
            cat_temp_c: 350.0,
            dpf_load_pct: 30.0,
            exhaust_temp_c: 250.0,
        }
    }

    pub fn cat_at_operating_temp(&self) -> bool {
        self.cat_temp_c > 300.0
    }

    pub fn dpf_needs_regen(&self) -> bool {
        self.dpf_load_pct > 80.0
    }

    pub fn emissions_compliant(&self) -> bool {
        match self.standard {
            EmissionStandard::Euro6 => self.co2_g_per_km <= 130.0 && self.nox_mg_per_km <= 60.0,
            EmissionStandard::Euro7 => self.co2_g_per_km <= 95.0 && self.nox_mg_per_km <= 30.0,
            EmissionStandard::Ulev => self.co2_g_per_km <= 50.0,
            EmissionStandard::ZeroEmission => self.co2_g_per_km <= 0.0,
        }
    }

    pub fn exhaust_health_score(&self) -> f64 {
        let cat_score = if self.cat_at_operating_temp() {
            30.0
        } else {
            15.0
        };
        let dpf_score = (1.0 - self.dpf_load_pct / 100.0) * 40.0;
        let emission_score = if self.emissions_compliant() {
            30.0
        } else {
            10.0
        };
        cat_score + dpf_score + emission_score
    }

    pub fn overheating(&self) -> bool {
        self.exhaust_temp_c > 700.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_operating() {
        let s = ExhaustSystem::new();
        assert!(s.cat_at_operating_temp());
    }

    #[test]
    fn test_cat_cold() {
        let mut s = ExhaustSystem::new();
        s.cat_temp_c = 100.0;
        assert!(!s.cat_at_operating_temp());
    }

    #[test]
    fn test_dpf_ok() {
        let s = ExhaustSystem::new();
        assert!(!s.dpf_needs_regen());
    }

    #[test]
    fn test_dpf_regen() {
        let mut s = ExhaustSystem::new();
        s.dpf_load_pct = 90.0;
        assert!(s.dpf_needs_regen());
    }

    #[test]
    fn test_compliant() {
        let s = ExhaustSystem::new();
        assert!(s.emissions_compliant());
    }

    #[test]
    fn test_not_compliant() {
        let mut s = ExhaustSystem::new();
        s.standard = EmissionStandard::ZeroEmission;
        assert!(!s.emissions_compliant());
    }

    #[test]
    fn test_health_score() {
        let s = ExhaustSystem::new();
        assert!(s.exhaust_health_score() > 70.0);
    }

    #[test]
    fn test_not_overheating() {
        let s = ExhaustSystem::new();
        assert!(!s.overheating());
    }
}
