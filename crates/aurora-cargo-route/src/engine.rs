/// Cargo routing: weight restrictions, loading zones, delivery optimization.
#[derive(Debug, Clone, PartialEq)]
pub enum CargoType {
    General,
    Refrigerated,
    Fragile,
    Oversized,
    Liquid,
    Livestock,
    Perishable,
    HighValue,
}

impl CargoType {
    pub fn max_speed_kmh(&self) -> f64 {
        match self {
            CargoType::Fragile => 60.0,
            CargoType::Liquid => 70.0,
            CargoType::Livestock => 65.0,
            CargoType::Oversized => 50.0,
            _ => 90.0,
        }
    }

    pub fn requires_climate_control(&self) -> bool {
        matches!(
            self,
            CargoType::Refrigerated | CargoType::Perishable | CargoType::Livestock
        )
    }

    pub fn insurance_multiplier(&self) -> f64 {
        match self {
            CargoType::HighValue => 3.0,
            CargoType::Fragile => 2.0,
            CargoType::Refrigerated => 1.5,
            CargoType::Oversized => 1.8,
            _ => 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CargoShipment {
    pub cargo_type: CargoType,
    pub weight_tonnes: f64,
    pub volume_m3: f64,
    pub value_usd: f64,
    pub time_sensitive: bool,
}

impl CargoShipment {
    pub fn new(cargo_type: CargoType, weight: f64, volume: f64) -> Self {
        Self {
            cargo_type,
            weight_tonnes: weight,
            volume_m3: volume,
            value_usd: 10000.0,
            time_sensitive: false,
        }
    }

    pub fn density(&self) -> f64 {
        if self.volume_m3 <= 0.0 {
            return 0.0;
        }
        self.weight_tonnes / self.volume_m3
    }

    pub fn insurance_cost(&self) -> f64 {
        self.value_usd * 0.005 * self.cargo_type.insurance_multiplier()
    }

    pub fn needs_escort(&self) -> bool {
        self.cargo_type == CargoType::Oversized || self.weight_tonnes > 40.0
    }

    pub fn max_route_speed(&self) -> f64 {
        let type_limit = self.cargo_type.max_speed_kmh();
        let weight_limit = if self.weight_tonnes > 30.0 {
            70.0
        } else {
            90.0
        };
        type_limit.min(weight_limit)
    }
}

#[derive(Debug, Clone)]
pub struct DeliveryStop {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub unload_time_min: f64,
    pub time_window_start: f64,
    pub time_window_end: f64,
}

impl DeliveryStop {
    pub fn new(name: &str, lat: f64, lon: f64) -> Self {
        Self {
            name: name.to_string(),
            lat,
            lon,
            unload_time_min: 30.0,
            time_window_start: 8.0,
            time_window_end: 18.0,
        }
    }

    pub fn window_hours(&self) -> f64 {
        (self.time_window_end - self.time_window_start).max(0.0)
    }

    pub fn is_within_window(&self, hour: f64) -> bool {
        hour >= self.time_window_start && hour <= self.time_window_end
    }
}

#[derive(Debug, Clone)]
pub struct CargoRoute {
    pub shipment: CargoShipment,
    pub stops: Vec<DeliveryStop>,
    pub total_distance_km: f64,
}

impl CargoRoute {
    pub fn new(shipment: CargoShipment) -> Self {
        Self {
            shipment,
            stops: Vec::new(),
            total_distance_km: 0.0,
        }
    }

    pub fn add_stop(&mut self, stop: DeliveryStop) {
        self.stops.push(stop);
    }

    pub fn total_unload_time_min(&self) -> f64 {
        self.stops.iter().map(|s| s.unload_time_min).sum()
    }

    pub fn estimated_drive_time_hr(&self) -> f64 {
        let speed = self.shipment.max_route_speed();
        if speed <= 0.0 {
            return f64::INFINITY;
        }
        self.total_distance_km / speed
    }

    pub fn total_time_hr(&self) -> f64 {
        self.estimated_drive_time_hr() + self.total_unload_time_min() / 60.0
    }

    pub fn cost_per_km(&self) -> f64 {
        let fuel = 0.35;
        let weight_adj = self.shipment.weight_tonnes * 0.01;
        fuel + weight_adj
    }

    pub fn total_fuel_cost(&self) -> f64 {
        self.cost_per_km() * self.total_distance_km
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_speed() {
        assert!(CargoType::Oversized.max_speed_kmh() < CargoType::General.max_speed_kmh());
    }

    #[test]
    fn test_climate_control() {
        assert!(CargoType::Refrigerated.requires_climate_control());
        assert!(!CargoType::General.requires_climate_control());
    }

    #[test]
    fn test_insurance() {
        assert!(
            CargoType::HighValue.insurance_multiplier() > CargoType::General.insurance_multiplier()
        );
    }

    #[test]
    fn test_density() {
        let s = CargoShipment::new(CargoType::General, 10.0, 20.0);
        assert!((s.density() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_needs_escort() {
        let s = CargoShipment::new(CargoType::Oversized, 20.0, 50.0);
        assert!(s.needs_escort());
    }

    #[test]
    fn test_max_route_speed() {
        let s = CargoShipment::new(CargoType::Fragile, 10.0, 5.0);
        assert!((s.max_route_speed() - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_delivery_window() {
        let d = DeliveryStop::new("Warehouse", 32.0, 34.0);
        assert!(d.is_within_window(10.0));
        assert!(!d.is_within_window(22.0));
    }

    #[test]
    fn test_window_hours() {
        let d = DeliveryStop::new("Warehouse", 32.0, 34.0);
        assert!((d.window_hours() - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_route_unload_time() {
        let mut r = CargoRoute::new(CargoShipment::new(CargoType::General, 10.0, 20.0));
        r.add_stop(DeliveryStop::new("A", 32.0, 34.0));
        r.add_stop(DeliveryStop::new("B", 32.1, 34.1));
        assert!((r.total_unload_time_min() - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_route_drive_time() {
        let mut r = CargoRoute::new(CargoShipment::new(CargoType::General, 10.0, 20.0));
        r.total_distance_km = 180.0;
        assert!(r.estimated_drive_time_hr() > 1.5 && r.estimated_drive_time_hr() < 3.0);
    }

    #[test]
    fn test_fuel_cost() {
        let mut r = CargoRoute::new(CargoShipment::new(CargoType::General, 10.0, 20.0));
        r.total_distance_km = 100.0;
        assert!(r.total_fuel_cost() > 30.0);
    }

    #[test]
    fn test_insurance_cost() {
        let s = CargoShipment::new(CargoType::HighValue, 5.0, 2.0);
        assert!(s.insurance_cost() > 100.0);
    }
}
