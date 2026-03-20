/// Toll calculation engine: road pricing, congestion charges, multi-currency support.
#[derive(Debug, Clone, PartialEq)]
pub enum TollType {
    FixedRate,
    PerKilometer,
    Congestion,
    Bridge,
    Tunnel,
    Express,
    Environmental,
}

impl TollType {
    pub fn is_distance_based(&self) -> bool {
        matches!(self, TollType::PerKilometer | TollType::Express)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VehicleClass {
    Motorcycle,
    Car,
    SUV,
    Van,
    Truck,
    Bus,
    HeavyTruck,
}

impl VehicleClass {
    pub fn multiplier(&self) -> f64 {
        match self {
            VehicleClass::Motorcycle => 0.5,
            VehicleClass::Car => 1.0,
            VehicleClass::SUV => 1.2,
            VehicleClass::Van => 1.5,
            VehicleClass::Truck => 2.0,
            VehicleClass::Bus => 1.8,
            VehicleClass::HeavyTruck => 3.0,
        }
    }

    pub fn axle_count(&self) -> u32 {
        match self {
            VehicleClass::Motorcycle => 2,
            VehicleClass::Car | VehicleClass::SUV => 2,
            VehicleClass::Van => 2,
            VehicleClass::Truck | VehicleClass::Bus => 3,
            VehicleClass::HeavyTruck => 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TollStation {
    pub name: String,
    pub toll_type: TollType,
    pub base_rate: f64,
    pub currency: String,
    pub position_km: f64,
    pub peak_multiplier: f64,
    pub eco_discount_pct: f64,
}

impl TollStation {
    pub fn new(name: &str, toll_type: TollType, base_rate: f64, position_km: f64) -> Self {
        Self {
            name: name.to_string(),
            toll_type,
            base_rate,
            currency: "USD".to_string(),
            position_km,
            peak_multiplier: 1.0,
            eco_discount_pct: 0.0,
        }
    }

    pub fn calculate(&self, vehicle: &VehicleClass, distance_km: f64, is_peak: bool) -> f64 {
        let base = if self.toll_type.is_distance_based() {
            self.base_rate * distance_km
        } else {
            self.base_rate
        };
        let class_adj = base * vehicle.multiplier();
        let peak_adj = if is_peak {
            class_adj * self.peak_multiplier
        } else {
            class_adj
        };
        let discount = peak_adj * (self.eco_discount_pct / 100.0);
        (peak_adj - discount).max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct TollRoute {
    pub stations: Vec<TollStation>,
}

impl Default for TollRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl TollRoute {
    pub fn new() -> Self {
        Self {
            stations: Vec::new(),
        }
    }

    pub fn add_station(&mut self, station: TollStation) {
        self.stations.push(station);
    }

    pub fn total_cost(&self, vehicle: &VehicleClass, total_distance_km: f64, is_peak: bool) -> f64 {
        self.stations
            .iter()
            .map(|s| s.calculate(vehicle, total_distance_km, is_peak))
            .sum()
    }

    pub fn station_count(&self) -> usize {
        self.stations.len()
    }

    pub fn most_expensive(&self, vehicle: &VehicleClass, distance_km: f64) -> Option<&TollStation> {
        self.stations.iter().max_by(|a, b| {
            let ca = a.calculate(vehicle, distance_km, false);
            let cb = b.calculate(vehicle, distance_km, false);
            ca.partial_cmp(&cb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn cost_per_km(&self, vehicle: &VehicleClass, distance_km: f64, is_peak: bool) -> f64 {
        if distance_km < f64::EPSILON {
            return 0.0;
        }
        self.total_cost(vehicle, distance_km, is_peak) / distance_km
    }

    pub fn savings_off_peak(&self, vehicle: &VehicleClass, distance_km: f64) -> f64 {
        let peak = self.total_cost(vehicle, distance_km, true);
        let off_peak = self.total_cost(vehicle, distance_km, false);
        (peak - off_peak).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_rate() {
        let s = TollStation::new("Gate A", TollType::FixedRate, 5.0, 10.0);
        let cost = s.calculate(&VehicleClass::Car, 100.0, false);
        assert!((cost - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_per_km_rate() {
        let s = TollStation::new("Highway", TollType::PerKilometer, 0.10, 0.0);
        let cost = s.calculate(&VehicleClass::Car, 100.0, false);
        assert!((cost - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_truck_multiplier() {
        let s = TollStation::new("Gate", TollType::FixedRate, 10.0, 5.0);
        let car = s.calculate(&VehicleClass::Car, 0.0, false);
        let truck = s.calculate(&VehicleClass::Truck, 0.0, false);
        assert!((truck - car * 2.0).abs() < 0.01);
    }

    #[test]
    fn test_peak_multiplier() {
        let mut s = TollStation::new("Gate", TollType::FixedRate, 10.0, 5.0);
        s.peak_multiplier = 1.5;
        let off = s.calculate(&VehicleClass::Car, 0.0, false);
        let peak = s.calculate(&VehicleClass::Car, 0.0, true);
        assert!((peak - off * 1.5).abs() < 0.01);
    }

    #[test]
    fn test_eco_discount() {
        let mut s = TollStation::new("Gate", TollType::FixedRate, 10.0, 5.0);
        s.eco_discount_pct = 50.0;
        let cost = s.calculate(&VehicleClass::Car, 0.0, false);
        assert!((cost - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_motorcycle_cheaper() {
        let s = TollStation::new("Gate", TollType::FixedRate, 10.0, 5.0);
        let moto = s.calculate(&VehicleClass::Motorcycle, 0.0, false);
        let car = s.calculate(&VehicleClass::Car, 0.0, false);
        assert!(moto < car);
    }

    #[test]
    fn test_distance_based() {
        assert!(TollType::PerKilometer.is_distance_based());
        assert!(!TollType::FixedRate.is_distance_based());
    }

    #[test]
    fn test_axle_count() {
        assert_eq!(VehicleClass::Car.axle_count(), 2);
        assert_eq!(VehicleClass::HeavyTruck.axle_count(), 5);
    }

    #[test]
    fn test_route_total_cost() {
        let mut r = TollRoute::new();
        r.add_station(TollStation::new("A", TollType::FixedRate, 5.0, 10.0));
        r.add_station(TollStation::new("B", TollType::FixedRate, 3.0, 50.0));
        let cost = r.total_cost(&VehicleClass::Car, 100.0, false);
        assert!((cost - 8.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_route() {
        let r = TollRoute::new();
        assert_eq!(r.station_count(), 0);
        assert_eq!(r.total_cost(&VehicleClass::Car, 100.0, false), 0.0);
    }

    #[test]
    fn test_cost_per_km() {
        let mut r = TollRoute::new();
        r.add_station(TollStation::new("A", TollType::FixedRate, 10.0, 0.0));
        let cpk = r.cost_per_km(&VehicleClass::Car, 100.0, false);
        assert!((cpk - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_cost_per_km_zero_distance() {
        let r = TollRoute::new();
        assert_eq!(r.cost_per_km(&VehicleClass::Car, 0.0, false), 0.0);
    }

    #[test]
    fn test_savings_off_peak() {
        let mut r = TollRoute::new();
        let mut s = TollStation::new("A", TollType::FixedRate, 10.0, 0.0);
        s.peak_multiplier = 2.0;
        r.add_station(s);
        let savings = r.savings_off_peak(&VehicleClass::Car, 100.0);
        assert!((savings - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_most_expensive() {
        let mut r = TollRoute::new();
        r.add_station(TollStation::new("Cheap", TollType::FixedRate, 2.0, 0.0));
        r.add_station(TollStation::new(
            "Expensive",
            TollType::FixedRate,
            20.0,
            50.0,
        ));
        let exp = r.most_expensive(&VehicleClass::Car, 100.0).unwrap();
        assert_eq!(exp.name, "Expensive");
    }

    #[test]
    fn test_negative_cost_floor() {
        let mut s = TollStation::new("Gate", TollType::FixedRate, 10.0, 5.0);
        s.eco_discount_pct = 200.0; // over 100%
        let cost = s.calculate(&VehicleClass::Car, 0.0, false);
        assert!(cost >= 0.0);
    }
}
