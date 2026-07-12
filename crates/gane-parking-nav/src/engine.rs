/// Parking navigation engine: spot finding, availability prediction, garage routing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParkingType {
    Street,
    Garage,
    Lot,
    Underground,
    Rooftop,
    Valet,
    Disabled,
    EvCharging,
}

impl ParkingType {
    pub fn is_covered(&self) -> bool {
        matches!(self, ParkingType::Garage | ParkingType::Underground)
    }

    pub fn base_hourly_rate(&self) -> f64 {
        match self {
            ParkingType::Street => 2.0,
            ParkingType::Garage => 4.0,
            ParkingType::Lot => 3.0,
            ParkingType::Underground => 5.0,
            ParkingType::Rooftop => 3.5,
            ParkingType::Valet => 8.0,
            ParkingType::Disabled => 0.0,
            ParkingType::EvCharging => 6.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParkingSpot {
    pub id: String,
    pub parking_type: ParkingType,
    pub lat: f64,
    pub lon: f64,
    pub available: bool,
    pub width_m: f64,
    pub length_m: f64,
    pub hourly_rate: f64,
}

impl ParkingSpot {
    pub fn new(id: &str, parking_type: ParkingType, lat: f64, lon: f64) -> Self {
        let rate = parking_type.base_hourly_rate();
        Self {
            id: id.to_string(),
            parking_type,
            lat,
            lon,
            available: true,
            width_m: 2.5,
            length_m: 5.0,
            hourly_rate: rate,
        }
    }

    pub fn area_m2(&self) -> f64 {
        self.width_m * self.length_m
    }

    pub fn fits_vehicle(&self, vehicle_width: f64, vehicle_length: f64) -> bool {
        self.width_m >= vehicle_width + 0.3 && self.length_m >= vehicle_length + 0.5
    }

    pub fn cost_for_hours(&self, hours: f64) -> f64 {
        (self.hourly_rate * hours).max(0.0)
    }

    pub fn distance_to(&self, lat: f64, lon: f64) -> f64 {
        let dlat = (self.lat - lat).to_radians();
        let dlon = (self.lon - lon).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + self.lat.to_radians().cos() * lat.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        6_371_000.0 * c
    }
}

#[derive(Debug, Clone)]
pub struct ParkingFacility {
    pub name: String,
    pub spots: Vec<ParkingSpot>,
    pub floors: u32,
    pub has_ev_charging: bool,
    pub max_height_m: f64,
}

impl ParkingFacility {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            spots: Vec::new(),
            floors: 1,
            has_ev_charging: false,
            max_height_m: 3.0,
        }
    }

    pub fn add_spot(&mut self, spot: ParkingSpot) {
        self.spots.push(spot);
    }

    pub fn total_spots(&self) -> usize {
        self.spots.len()
    }

    pub fn available_spots(&self) -> usize {
        self.spots.iter().filter(|s| s.available).count()
    }

    pub fn occupancy_pct(&self) -> f64 {
        if self.spots.is_empty() {
            return 0.0;
        }
        let occupied = self.spots.iter().filter(|s| !s.available).count();
        (occupied as f64 / self.spots.len() as f64) * 100.0
    }

    pub fn cheapest_available(&self) -> Option<&ParkingSpot> {
        self.spots
            .iter()
            .filter(|s| s.available)
            .min_by(|a, b| a.hourly_rate.partial_cmp(&b.hourly_rate).unwrap())
    }

    pub fn nearest_available(&self, lat: f64, lon: f64) -> Option<&ParkingSpot> {
        self.spots.iter().filter(|s| s.available).min_by(|a, b| {
            let da = a.distance_to(lat, lon);
            let db = b.distance_to(lat, lon);
            da.partial_cmp(&db).unwrap()
        })
    }

    pub fn ev_spots_available(&self) -> usize {
        self.spots
            .iter()
            .filter(|s| s.available && s.parking_type == ParkingType::EvCharging)
            .count()
    }

    pub fn average_rate(&self) -> f64 {
        if self.spots.is_empty() {
            return 0.0;
        }
        let total: f64 = self.spots.iter().map(|s| s.hourly_rate).sum();
        total / self.spots.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parking_type_covered() {
        assert!(ParkingType::Garage.is_covered());
        assert!(ParkingType::Underground.is_covered());
        assert!(!ParkingType::Street.is_covered());
    }

    #[test]
    fn test_disabled_free() {
        assert!(ParkingType::Disabled.base_hourly_rate() < f64::EPSILON);
    }

    #[test]
    fn test_spot_area() {
        let s = ParkingSpot::new("A1", ParkingType::Garage, 32.0, 34.0);
        assert!((s.area_m2() - 12.5).abs() < 0.01);
    }

    #[test]
    fn test_fits_vehicle() {
        let s = ParkingSpot::new("A1", ParkingType::Garage, 32.0, 34.0);
        assert!(s.fits_vehicle(1.8, 4.0));
        assert!(!s.fits_vehicle(2.5, 5.0));
    }

    #[test]
    fn test_cost_for_hours() {
        let s = ParkingSpot::new("A1", ParkingType::Garage, 32.0, 34.0);
        assert!((s.cost_for_hours(3.0) - 12.0).abs() < 0.01);
    }

    #[test]
    fn test_distance_to() {
        let s = ParkingSpot::new("A1", ParkingType::Street, 32.0, 34.0);
        let d = s.distance_to(32.001, 34.001);
        assert!(d > 50.0 && d < 200.0);
    }

    #[test]
    fn test_distance_same_point() {
        let s = ParkingSpot::new("A1", ParkingType::Street, 32.0, 34.0);
        assert!(s.distance_to(32.0, 34.0) < 1.0);
    }

    #[test]
    fn test_facility_total_spots() {
        let mut f = ParkingFacility::new("Central Garage");
        f.add_spot(ParkingSpot::new("A1", ParkingType::Garage, 32.0, 34.0));
        f.add_spot(ParkingSpot::new("A2", ParkingType::Garage, 32.0, 34.0));
        assert_eq!(f.total_spots(), 2);
    }

    #[test]
    fn test_available_spots() {
        let mut f = ParkingFacility::new("Lot");
        let mut s1 = ParkingSpot::new("A1", ParkingType::Lot, 32.0, 34.0);
        s1.available = false;
        f.add_spot(s1);
        f.add_spot(ParkingSpot::new("A2", ParkingType::Lot, 32.0, 34.0));
        assert_eq!(f.available_spots(), 1);
    }

    #[test]
    fn test_occupancy() {
        let mut f = ParkingFacility::new("Lot");
        let mut s1 = ParkingSpot::new("A1", ParkingType::Lot, 32.0, 34.0);
        s1.available = false;
        f.add_spot(s1);
        f.add_spot(ParkingSpot::new("A2", ParkingType::Lot, 32.0, 34.0));
        assert!((f.occupancy_pct() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_facility() {
        let f = ParkingFacility::new("Empty");
        assert_eq!(f.total_spots(), 0);
        assert_eq!(f.occupancy_pct(), 0.0);
        assert_eq!(f.average_rate(), 0.0);
    }

    #[test]
    fn test_cheapest_available() {
        let mut f = ParkingFacility::new("Mixed");
        f.add_spot(ParkingSpot::new("A1", ParkingType::Valet, 32.0, 34.0));
        f.add_spot(ParkingSpot::new("A2", ParkingType::Street, 32.0, 34.0));
        let cheapest = f.cheapest_available().unwrap();
        assert_eq!(cheapest.id, "A2");
    }

    #[test]
    fn test_nearest_available() {
        let mut f = ParkingFacility::new("Spread");
        f.add_spot(ParkingSpot::new("Far", ParkingType::Street, 33.0, 35.0));
        f.add_spot(ParkingSpot::new(
            "Near",
            ParkingType::Street,
            32.001,
            34.001,
        ));
        let nearest = f.nearest_available(32.0, 34.0).unwrap();
        assert_eq!(nearest.id, "Near");
    }

    #[test]
    fn test_ev_spots() {
        let mut f = ParkingFacility::new("EV Hub");
        f.add_spot(ParkingSpot::new("E1", ParkingType::EvCharging, 32.0, 34.0));
        f.add_spot(ParkingSpot::new("A1", ParkingType::Garage, 32.0, 34.0));
        assert_eq!(f.ev_spots_available(), 1);
    }

    #[test]
    fn test_average_rate() {
        let mut f = ParkingFacility::new("Mixed");
        f.add_spot(ParkingSpot::new("A1", ParkingType::Street, 32.0, 34.0));
        f.add_spot(ParkingSpot::new("A2", ParkingType::Valet, 32.0, 34.0));
        let avg = f.average_rate();
        assert!((avg - 5.0).abs() < 0.01); // (2+8)/2
    }
}
