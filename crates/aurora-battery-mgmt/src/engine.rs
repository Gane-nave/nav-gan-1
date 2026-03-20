/// Battery management: state of charge, range estimation, charging optimization.
#[derive(Debug, Clone, PartialEq)]
pub enum BatteryChemistry {
    LithiumIon,
    LithiumPolymer,
    LithiumIronPhosphate,
    SolidState,
    NickelMetalHydride,
    LeadAcid,
}

impl BatteryChemistry {
    pub fn energy_density_whkg(&self) -> f64 {
        match self {
            BatteryChemistry::SolidState => 500.0,
            BatteryChemistry::LithiumPolymer => 250.0,
            BatteryChemistry::LithiumIon => 200.0,
            BatteryChemistry::LithiumIronPhosphate => 160.0,
            BatteryChemistry::NickelMetalHydride => 80.0,
            BatteryChemistry::LeadAcid => 40.0,
        }
    }

    pub fn cycle_life(&self) -> u32 {
        match self {
            BatteryChemistry::LithiumIronPhosphate => 5000,
            BatteryChemistry::SolidState => 3000,
            BatteryChemistry::LithiumIon => 1500,
            BatteryChemistry::LithiumPolymer => 1000,
            BatteryChemistry::NickelMetalHydride => 800,
            BatteryChemistry::LeadAcid => 500,
        }
    }

    pub fn fast_charge_capable(&self) -> bool {
        matches!(
            self,
            BatteryChemistry::LithiumIon
                | BatteryChemistry::LithiumIronPhosphate
                | BatteryChemistry::SolidState
        )
    }
}

#[derive(Debug, Clone)]
pub struct BatteryPack {
    pub chemistry: BatteryChemistry,
    pub capacity_kwh: f64,
    pub soc_pct: f64,
    pub health_pct: f64,
    pub temperature_c: f64,
    pub voltage_v: f64,
    pub current_a: f64,
}

impl BatteryPack {
    pub fn new(chemistry: BatteryChemistry, capacity_kwh: f64) -> Self {
        Self {
            chemistry,
            capacity_kwh,
            soc_pct: 80.0,
            health_pct: 100.0,
            temperature_c: 25.0,
            voltage_v: 400.0,
            current_a: 0.0,
        }
    }

    pub fn usable_energy_kwh(&self) -> f64 {
        self.capacity_kwh * (self.soc_pct / 100.0) * (self.health_pct / 100.0)
    }

    pub fn range_km(&self, consumption_kwh_per_km: f64) -> f64 {
        if consumption_kwh_per_km <= 0.0 {
            return f64::INFINITY;
        }
        self.usable_energy_kwh() / consumption_kwh_per_km
    }

    pub fn power_kw(&self) -> f64 {
        (self.voltage_v * self.current_a) / 1000.0
    }

    pub fn temp_ok(&self) -> bool {
        (-10.0..=45.0).contains(&self.temperature_c)
    }

    pub fn needs_cooling(&self) -> bool {
        self.temperature_c > 35.0
    }

    pub fn needs_heating(&self) -> bool {
        self.temperature_c < 5.0
    }

    pub fn charge_time_hr(&self, charger_kw: f64) -> f64 {
        if charger_kw <= 0.0 {
            return f64::INFINITY;
        }
        let needed = self.capacity_kwh * (1.0 - self.soc_pct / 100.0);
        let efficiency = 0.9;
        needed / (charger_kw * efficiency)
    }

    pub fn is_low(&self) -> bool {
        self.soc_pct < 20.0
    }

    pub fn is_critical(&self) -> bool {
        self.soc_pct < 5.0
    }

    pub fn degradation_pct(&self) -> f64 {
        100.0 - self.health_pct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_density() {
        assert!(
            BatteryChemistry::SolidState.energy_density_whkg()
                > BatteryChemistry::LeadAcid.energy_density_whkg()
        );
    }

    #[test]
    fn test_cycle_life() {
        assert!(
            BatteryChemistry::LithiumIronPhosphate.cycle_life()
                > BatteryChemistry::LeadAcid.cycle_life()
        );
    }

    #[test]
    fn test_fast_charge() {
        assert!(BatteryChemistry::LithiumIon.fast_charge_capable());
        assert!(!BatteryChemistry::LeadAcid.fast_charge_capable());
    }

    #[test]
    fn test_usable_energy() {
        let b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        assert!((b.usable_energy_kwh() - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_range() {
        let b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        let range = b.range_km(0.2);
        assert!((range - 300.0).abs() < 0.1);
    }

    #[test]
    fn test_temp_ok() {
        let b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        assert!(b.temp_ok());
    }

    #[test]
    fn test_needs_cooling() {
        let mut b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        b.temperature_c = 40.0;
        assert!(b.needs_cooling());
    }

    #[test]
    fn test_needs_heating() {
        let mut b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        b.temperature_c = 0.0;
        assert!(b.needs_heating());
    }

    #[test]
    fn test_charge_time() {
        let mut b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        b.soc_pct = 20.0;
        let t = b.charge_time_hr(150.0);
        assert!(t > 0.3 && t < 1.0);
    }

    #[test]
    fn test_is_low() {
        let mut b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        b.soc_pct = 15.0;
        assert!(b.is_low());
    }

    #[test]
    fn test_is_critical() {
        let mut b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        b.soc_pct = 3.0;
        assert!(b.is_critical());
    }

    #[test]
    fn test_degradation() {
        let mut b = BatteryPack::new(BatteryChemistry::LithiumIon, 75.0);
        b.health_pct = 85.0;
        assert!((b.degradation_pct() - 15.0).abs() < 0.01);
    }
}
