/// Solar charging: panel efficiency, sun tracking, charge optimization
/// Phase 166

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelPosition {
    Roof,
    Hood,
    Trunk,
    Sides,
}

impl PanelPosition {
    pub fn area_m2(&self) -> f64 {
        match self {
            PanelPosition::Roof => 2.0,
            PanelPosition::Hood => 1.2,
            PanelPosition::Trunk => 0.8,
            PanelPosition::Sides => 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolarPanel {
    pub position: PanelPosition,
    pub efficiency_pct: f64,
    pub irradiance_w_per_m2: f64,
    pub degradation_pct: f64,
}

impl SolarPanel {
    pub fn new(position: PanelPosition) -> Self {
        Self {
            position,
            efficiency_pct: 22.0,
            irradiance_w_per_m2: 800.0,
            degradation_pct: 0.0,
        }
    }

    pub fn power_output_w(&self) -> f64 {
        let area = self.position.area_m2();
        let eff = (self.efficiency_pct - self.degradation_pct) / 100.0;
        area * self.irradiance_w_per_m2 * eff
    }

    pub fn daily_energy_wh(&self, sun_hours: f64) -> f64 {
        self.power_output_w() * sun_hours
    }
}

#[derive(Debug, Clone)]
pub struct SolarChargeSystem {
    pub panels: Vec<SolarPanel>,
    pub charging_active: bool,
    pub total_generated_kwh: f64,
}

impl Default for SolarChargeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SolarChargeSystem {
    pub fn new() -> Self {
        Self {
            panels: vec![SolarPanel::new(PanelPosition::Roof)],
            charging_active: true,
            total_generated_kwh: 0.0,
        }
    }

    pub fn total_power_w(&self) -> f64 {
        if !self.charging_active {
            return 0.0;
        }
        self.panels.iter().map(|p| p.power_output_w()).sum()
    }

    pub fn daily_range_km(&self, sun_hours: f64) -> f64 {
        let daily_wh: f64 = self
            .panels
            .iter()
            .map(|p| p.daily_energy_wh(sun_hours))
            .sum();
        daily_wh / 1000.0 * 5.0
    }

    pub fn panel_count(&self) -> usize {
        self.panels.len()
    }

    pub fn total_area_m2(&self) -> f64 {
        self.panels.iter().map(|p| p.position.area_m2()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert!(PanelPosition::Roof.area_m2() > PanelPosition::Sides.area_m2());
    }

    #[test]
    fn test_power_output() {
        let p = SolarPanel::new(PanelPosition::Roof);
        assert!(p.power_output_w() > 300.0);
    }

    #[test]
    fn test_daily_energy() {
        let p = SolarPanel::new(PanelPosition::Roof);
        assert!(p.daily_energy_wh(5.0) > 1500.0);
    }

    #[test]
    fn test_degradation() {
        let mut p = SolarPanel::new(PanelPosition::Roof);
        let before = p.power_output_w();
        p.degradation_pct = 5.0;
        assert!(p.power_output_w() < before);
    }

    #[test]
    fn test_system_power() {
        let s = SolarChargeSystem::new();
        assert!(s.total_power_w() > 300.0);
    }

    #[test]
    fn test_inactive() {
        let mut s = SolarChargeSystem::new();
        s.charging_active = false;
        assert!((s.total_power_w() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_daily_range() {
        let s = SolarChargeSystem::new();
        assert!(s.daily_range_km(5.0) > 1.0);
    }

    #[test]
    fn test_panel_count() {
        let s = SolarChargeSystem::new();
        assert_eq!(s.panel_count(), 1);
    }
}
