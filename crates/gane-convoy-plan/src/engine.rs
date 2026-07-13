/// Convoy planning: vehicle coordination, spacing, communication protocols.
#[derive(Debug, Clone, PartialEq)]
pub enum ConvoyRole {
    Lead,
    Follow,
    Rear,
    Scout,
    Support,
}

impl ConvoyRole {
    pub fn priority(&self) -> u8 {
        match self {
            ConvoyRole::Lead => 10,
            ConvoyRole::Scout => 8,
            ConvoyRole::Follow => 5,
            ConvoyRole::Support => 4,
            ConvoyRole::Rear => 3,
        }
    }

    pub fn comm_interval_sec(&self) -> f64 {
        match self {
            ConvoyRole::Lead => 5.0,
            ConvoyRole::Scout => 3.0,
            ConvoyRole::Rear => 10.0,
            _ => 8.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConvoyVehicle {
    pub id: String,
    pub role: ConvoyRole,
    pub position_km: f64,
    pub speed_kmh: f64,
    pub length_m: f64,
}

impl ConvoyVehicle {
    pub fn new(id: &str, role: ConvoyRole) -> Self {
        Self {
            id: id.to_string(),
            role,
            position_km: 0.0,
            speed_kmh: 80.0,
            length_m: 15.0,
        }
    }

    pub fn safe_following_distance_m(&self) -> f64 {
        let speed_ms = self.speed_kmh / 3.6;
        (speed_ms * 2.5).max(20.0)
    }
}

#[derive(Debug, Clone)]
pub struct ConvoyPlan {
    pub vehicles: Vec<ConvoyVehicle>,
    pub target_speed_kmh: f64,
    pub route_distance_km: f64,
}

impl ConvoyPlan {
    pub fn new(target_speed: f64, distance: f64) -> Self {
        Self {
            vehicles: Vec::new(),
            target_speed_kmh: target_speed,
            route_distance_km: distance,
        }
    }

    pub fn add_vehicle(&mut self, v: ConvoyVehicle) {
        self.vehicles.push(v);
    }

    pub fn convoy_length_m(&self) -> f64 {
        if self.vehicles.is_empty() {
            return 0.0;
        }
        let total_vehicle: f64 = self.vehicles.iter().map(|v| v.length_m).sum();
        let gaps = (self.vehicles.len() - 1) as f64 * self.vehicles[0].safe_following_distance_m();
        total_vehicle + gaps
    }

    pub fn estimated_time_hr(&self) -> f64 {
        if self.target_speed_kmh <= 0.0 {
            return f64::INFINITY;
        }
        self.route_distance_km / self.target_speed_kmh
    }

    pub fn vehicle_count(&self) -> usize {
        self.vehicles.len()
    }

    pub fn lead_vehicle(&self) -> Option<&ConvoyVehicle> {
        self.vehicles.iter().find(|v| v.role == ConvoyRole::Lead)
    }

    pub fn has_scout(&self) -> bool {
        self.vehicles.iter().any(|v| v.role == ConvoyRole::Scout)
    }

    pub fn max_comm_interval(&self) -> f64 {
        self.vehicles
            .iter()
            .map(|v| v.role.comm_interval_sec())
            .fold(0.0_f64, f64::max)
    }

    pub fn formation_valid(&self) -> bool {
        let leads = self
            .vehicles
            .iter()
            .filter(|v| v.role == ConvoyRole::Lead)
            .count();
        let rears = self
            .vehicles
            .iter()
            .filter(|v| v.role == ConvoyRole::Rear)
            .count();
        leads == 1 && rears <= 1 && self.vehicles.len() >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_priority() {
        assert!(ConvoyRole::Lead.priority() > ConvoyRole::Rear.priority());
    }

    #[test]
    fn test_comm_interval() {
        assert!(ConvoyRole::Scout.comm_interval_sec() < ConvoyRole::Rear.comm_interval_sec());
    }

    #[test]
    fn test_following_distance() {
        let v = ConvoyVehicle::new("V1", ConvoyRole::Follow);
        assert!(v.safe_following_distance_m() > 40.0);
    }

    #[test]
    fn test_convoy_length() {
        let mut p = ConvoyPlan::new(80.0, 100.0);
        p.add_vehicle(ConvoyVehicle::new("V1", ConvoyRole::Lead));
        p.add_vehicle(ConvoyVehicle::new("V2", ConvoyRole::Follow));
        assert!(p.convoy_length_m() > 50.0);
    }

    #[test]
    fn test_time_estimate() {
        let p = ConvoyPlan::new(80.0, 160.0);
        assert!((p.estimated_time_hr() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_lead_vehicle() {
        let mut p = ConvoyPlan::new(80.0, 100.0);
        p.add_vehicle(ConvoyVehicle::new("V1", ConvoyRole::Lead));
        assert!(p.lead_vehicle().is_some());
    }

    #[test]
    fn test_no_scout() {
        let mut p = ConvoyPlan::new(80.0, 100.0);
        p.add_vehicle(ConvoyVehicle::new("V1", ConvoyRole::Lead));
        assert!(!p.has_scout());
    }

    #[test]
    fn test_formation_valid() {
        let mut p = ConvoyPlan::new(80.0, 100.0);
        p.add_vehicle(ConvoyVehicle::new("V1", ConvoyRole::Lead));
        p.add_vehicle(ConvoyVehicle::new("V2", ConvoyRole::Rear));
        assert!(p.formation_valid());
    }

    #[test]
    fn test_formation_invalid_no_lead() {
        let mut p = ConvoyPlan::new(80.0, 100.0);
        p.add_vehicle(ConvoyVehicle::new("V1", ConvoyRole::Follow));
        p.add_vehicle(ConvoyVehicle::new("V2", ConvoyRole::Follow));
        assert!(!p.formation_valid());
    }

    #[test]
    fn test_empty_convoy() {
        let p = ConvoyPlan::new(80.0, 100.0);
        assert_eq!(p.convoy_length_m(), 0.0);
        assert_eq!(p.vehicle_count(), 0);
    }
}
