/// Fuel cap management: cap status, fuel type detection, vapor recovery
/// Phase 144

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FuelType {
    Gasoline,
    Diesel,
    E85,
    Hydrogen,
    Electric,
}

impl FuelType {
    pub fn energy_density_kwh_per_l(&self) -> f64 {
        match self {
            FuelType::Gasoline => 9.5,
            FuelType::Diesel => 10.7,
            FuelType::E85 => 6.3,
            FuelType::Hydrogen => 2.6,
            FuelType::Electric => 0.0,
        }
    }

    pub fn is_flammable(&self) -> bool {
        !matches!(self, FuelType::Electric)
    }

    pub fn vapor_recovery_needed(&self) -> bool {
        matches!(self, FuelType::Gasoline | FuelType::E85)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapState {
    Sealed,
    Open,
    Missing,
    Faulty,
}

impl CapState {
    pub fn is_safe(&self) -> bool {
        matches!(self, CapState::Sealed)
    }
}

#[derive(Debug, Clone)]
pub struct FuelCapSystem {
    pub fuel_type: FuelType,
    pub cap_state: CapState,
    pub tank_level_pct: f64,
    pub tank_capacity_l: f64,
    pub vapor_pressure_kpa: f64,
}

impl FuelCapSystem {
    pub fn new(fuel: FuelType, capacity: f64) -> Self {
        Self {
            fuel_type: fuel,
            cap_state: CapState::Sealed,
            tank_level_pct: 100.0,
            tank_capacity_l: capacity,
            vapor_pressure_kpa: 0.0,
        }
    }

    pub fn fuel_remaining_l(&self) -> f64 {
        self.tank_capacity_l * (self.tank_level_pct / 100.0)
    }

    pub fn is_low_fuel(&self) -> bool {
        self.tank_level_pct < 15.0
    }

    pub fn is_empty(&self) -> bool {
        self.tank_level_pct < 2.0
    }

    pub fn energy_remaining_kwh(&self) -> f64 {
        self.fuel_remaining_l() * self.fuel_type.energy_density_kwh_per_l()
    }

    pub fn cap_warning(&self) -> bool {
        !self.cap_state.is_safe()
    }

    pub fn vapor_leak(&self) -> bool {
        self.cap_state != CapState::Sealed && self.fuel_type.vapor_recovery_needed()
    }

    pub fn safe_to_refuel(&self) -> bool {
        self.cap_state == CapState::Open
    }

    pub fn range_estimate_km(&self, consumption_l_per_100km: f64) -> f64 {
        if consumption_l_per_100km <= 0.0 {
            return 0.0;
        }
        self.fuel_remaining_l() / consumption_l_per_100km * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_density() {
        assert!(
            FuelType::Diesel.energy_density_kwh_per_l()
                > FuelType::Gasoline.energy_density_kwh_per_l()
        );
    }

    #[test]
    fn test_flammable() {
        assert!(FuelType::Gasoline.is_flammable());
        assert!(!FuelType::Electric.is_flammable());
    }

    #[test]
    fn test_cap_safe() {
        assert!(CapState::Sealed.is_safe());
        assert!(!CapState::Open.is_safe());
    }

    #[test]
    fn test_fuel_remaining() {
        let s = FuelCapSystem::new(FuelType::Gasoline, 60.0);
        assert!((s.fuel_remaining_l() - 60.0).abs() < 0.1);
    }

    #[test]
    fn test_low_fuel() {
        let mut s = FuelCapSystem::new(FuelType::Diesel, 50.0);
        s.tank_level_pct = 10.0;
        assert!(s.is_low_fuel());
    }

    #[test]
    fn test_not_low() {
        let s = FuelCapSystem::new(FuelType::Gasoline, 50.0);
        assert!(!s.is_low_fuel());
    }

    #[test]
    fn test_cap_warning() {
        let mut s = FuelCapSystem::new(FuelType::Gasoline, 50.0);
        s.cap_state = CapState::Missing;
        assert!(s.cap_warning());
    }

    #[test]
    fn test_vapor_leak() {
        let mut s = FuelCapSystem::new(FuelType::Gasoline, 50.0);
        s.cap_state = CapState::Open;
        assert!(s.vapor_leak());
    }

    #[test]
    fn test_range() {
        let s = FuelCapSystem::new(FuelType::Gasoline, 60.0);
        assert!(s.range_estimate_km(8.0) > 700.0);
    }

    #[test]
    fn test_energy() {
        let s = FuelCapSystem::new(FuelType::Diesel, 50.0);
        assert!(s.energy_remaining_kwh() > 400.0);
    }
}
