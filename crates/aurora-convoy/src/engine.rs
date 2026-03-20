/// Convoy/platoon driving engine: formation management, inter-vehicle coordination.
#[derive(Debug, Clone, PartialEq)]
pub enum ConvoyRole {
    Leader,
    Follower(u32), // position index
    Tail,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormationType {
    Column,
    Staggered,
    Echelon,
    Diamond,
}

#[derive(Debug, Clone)]
pub struct ConvoyVehicle {
    pub id: String,
    pub role: ConvoyRole,
    pub speed_kmh: f64,
    pub gap_m: f64,
    pub lat: f64,
    pub lon: f64,
    pub heading_deg: f64,
    pub brake_capability: f64,
    pub comm_latency_ms: u32,
}

impl ConvoyVehicle {
    pub fn new(id: &str, role: ConvoyRole) -> Self {
        Self {
            id: id.to_string(),
            role,
            speed_kmh: 0.0,
            gap_m: 15.0,
            lat: 0.0,
            lon: 0.0,
            heading_deg: 0.0,
            brake_capability: 1.0,
            comm_latency_ms: 10,
        }
    }

    pub fn is_leader(&self) -> bool {
        self.role == ConvoyRole::Leader
    }

    pub fn safe_gap(&self) -> f64 {
        let speed_factor = self.speed_kmh / 3.6; // m/s
        let reaction_dist = speed_factor * (self.comm_latency_ms as f64 / 1000.0);
        let brake_dist =
            (speed_factor * speed_factor) / (2.0 * 9.81 * self.brake_capability.clamp(0.1, 1.0));
        (reaction_dist + brake_dist).max(5.0)
    }

    pub fn gap_status(&self) -> GapStatus {
        let safe = self.safe_gap();
        if self.gap_m >= safe * 1.2 {
            GapStatus::TooFar
        } else if self.gap_m >= safe * 0.8 {
            GapStatus::Optimal
        } else {
            GapStatus::TooClose
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GapStatus {
    TooClose,
    Optimal,
    TooFar,
}

#[derive(Debug, Clone)]
pub struct Convoy {
    pub name: String,
    pub formation: FormationType,
    pub vehicles: Vec<ConvoyVehicle>,
    pub target_speed_kmh: f64,
    pub max_gap_m: f64,
    pub min_gap_m: f64,
}

impl Default for Convoy {
    fn default() -> Self {
        Self::new("default")
    }
}

impl Convoy {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            formation: FormationType::Column,
            vehicles: Vec::new(),
            target_speed_kmh: 80.0,
            max_gap_m: 30.0,
            min_gap_m: 5.0,
        }
    }

    pub fn add_vehicle(&mut self, vehicle: ConvoyVehicle) {
        self.vehicles.push(vehicle);
    }

    pub fn size(&self) -> usize {
        self.vehicles.len()
    }

    pub fn leader(&self) -> Option<&ConvoyVehicle> {
        self.vehicles.iter().find(|v| v.is_leader())
    }

    pub fn followers(&self) -> Vec<&ConvoyVehicle> {
        self.vehicles.iter().filter(|v| !v.is_leader()).collect()
    }

    pub fn average_speed(&self) -> f64 {
        if self.vehicles.is_empty() {
            return 0.0;
        }
        let total: f64 = self.vehicles.iter().map(|v| v.speed_kmh).sum();
        total / self.vehicles.len() as f64
    }

    pub fn speed_variance(&self) -> f64 {
        if self.vehicles.len() < 2 {
            return 0.0;
        }
        let avg = self.average_speed();
        let sum_sq: f64 = self
            .vehicles
            .iter()
            .map(|v| (v.speed_kmh - avg).powi(2))
            .sum();
        sum_sq / self.vehicles.len() as f64
    }

    pub fn is_coherent(&self) -> bool {
        self.speed_variance() < 25.0
            && self
                .vehicles
                .iter()
                .all(|v| v.is_leader() || (v.gap_m >= self.min_gap_m && v.gap_m <= self.max_gap_m))
    }

    pub fn convoy_length_m(&self) -> f64 {
        if self.vehicles.len() <= 1 {
            return 0.0;
        }
        self.vehicles
            .iter()
            .skip(1)
            .map(|v| v.gap_m + 5.0) // 5m average vehicle length
            .sum()
    }

    pub fn worst_comm_latency(&self) -> u32 {
        self.vehicles
            .iter()
            .map(|v| v.comm_latency_ms)
            .max()
            .unwrap_or(0)
    }

    pub fn formation_quality(&self) -> f64 {
        if self.vehicles.is_empty() {
            return 0.0;
        }
        let speed_score = 1.0 - (self.speed_variance() / 100.0).min(1.0);
        let gap_score = self
            .vehicles
            .iter()
            .filter(|v| !v.is_leader())
            .map(|v| match v.gap_status() {
                GapStatus::Optimal => 1.0,
                GapStatus::TooFar => 0.5,
                GapStatus::TooClose => 0.2,
            })
            .sum::<f64>()
            / self.followers().len().max(1) as f64;
        let comm_score = 1.0 - (self.worst_comm_latency() as f64 / 200.0).min(1.0);
        (speed_score * 0.4 + gap_score * 0.4 + comm_score * 0.2).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConvoyCommand {
    SetSpeed(f64),
    AdjustGap(f64),
    EmergencyBrake,
    ChangeFormation(FormationType),
    AddVehicle(String),
    RemoveVehicle(String),
    Dissolve,
}

#[derive(Debug, Clone)]
pub struct ConvoyPlanner {
    pub min_vehicles: usize,
    pub max_vehicles: usize,
    pub min_speed_kmh: f64,
    pub max_speed_kmh: f64,
}

impl Default for ConvoyPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl ConvoyPlanner {
    pub fn new() -> Self {
        Self {
            min_vehicles: 2,
            max_vehicles: 10,
            min_speed_kmh: 30.0,
            max_speed_kmh: 120.0,
        }
    }

    pub fn can_form_convoy(&self, vehicle_count: usize, avg_speed: f64) -> bool {
        vehicle_count >= self.min_vehicles
            && vehicle_count <= self.max_vehicles
            && avg_speed >= self.min_speed_kmh
            && avg_speed <= self.max_speed_kmh
    }

    pub fn recommended_formation(&self, vehicle_count: usize) -> FormationType {
        match vehicle_count {
            0..=2 => FormationType::Column,
            3..=4 => FormationType::Staggered,
            5..=6 => FormationType::Echelon,
            _ => FormationType::Diamond,
        }
    }

    pub fn fuel_savings_pct(&self, position: usize, speed_kmh: f64) -> f64 {
        if position == 0 {
            return 2.0; // leader gets minimal savings
        }
        let base = 10.0 + (position as f64 * 2.0).min(8.0);
        let speed_bonus = (speed_kmh / 100.0).min(1.0) * 5.0;
        (base + speed_bonus).min(25.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_convoy() -> Convoy {
        let mut c = Convoy::new("test");
        c.add_vehicle(ConvoyVehicle::new("v1", ConvoyRole::Leader));
        let mut f1 = ConvoyVehicle::new("v2", ConvoyRole::Follower(1));
        f1.speed_kmh = 80.0;
        f1.gap_m = 15.0;
        c.add_vehicle(f1);
        let mut f2 = ConvoyVehicle::new("v3", ConvoyRole::Follower(2));
        f2.speed_kmh = 80.0;
        f2.gap_m = 15.0;
        c.add_vehicle(f2);
        let mut tail = ConvoyVehicle::new("v4", ConvoyRole::Tail);
        tail.speed_kmh = 80.0;
        tail.gap_m = 15.0;
        c.add_vehicle(tail);
        if let Some(leader) = c.vehicles.first_mut() {
            leader.speed_kmh = 80.0;
        }
        c
    }

    #[test]
    fn test_vehicle_is_leader() {
        let v = ConvoyVehicle::new("v1", ConvoyRole::Leader);
        assert!(v.is_leader());
    }

    #[test]
    fn test_vehicle_not_leader() {
        let v = ConvoyVehicle::new("v2", ConvoyRole::Follower(1));
        assert!(!v.is_leader());
    }

    #[test]
    fn test_safe_gap() {
        let mut v = ConvoyVehicle::new("v1", ConvoyRole::Follower(1));
        v.speed_kmh = 100.0;
        assert!(v.safe_gap() > 5.0);
    }

    #[test]
    fn test_gap_status_optimal() {
        let mut v = ConvoyVehicle::new("v1", ConvoyRole::Follower(1));
        v.speed_kmh = 80.0;
        v.gap_m = v.safe_gap();
        assert_eq!(v.gap_status(), GapStatus::Optimal);
    }

    #[test]
    fn test_gap_status_too_close() {
        let mut v = ConvoyVehicle::new("v1", ConvoyRole::Follower(1));
        v.speed_kmh = 100.0;
        v.gap_m = 2.0;
        assert_eq!(v.gap_status(), GapStatus::TooClose);
    }

    #[test]
    fn test_convoy_size() {
        let c = test_convoy();
        assert_eq!(c.size(), 4);
    }

    #[test]
    fn test_convoy_leader() {
        let c = test_convoy();
        assert!(c.leader().is_some());
        assert!(c.leader().unwrap().is_leader());
    }

    #[test]
    fn test_convoy_followers() {
        let c = test_convoy();
        assert_eq!(c.followers().len(), 3);
    }

    #[test]
    fn test_average_speed() {
        let c = test_convoy();
        assert!((c.average_speed() - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_speed_variance_uniform() {
        let c = test_convoy();
        assert!(c.speed_variance() < 0.01);
    }

    #[test]
    fn test_is_coherent() {
        let c = test_convoy();
        assert!(c.is_coherent());
    }

    #[test]
    fn test_convoy_length() {
        let c = test_convoy();
        assert!(c.convoy_length_m() > 0.0);
    }

    #[test]
    fn test_worst_comm_latency() {
        let c = test_convoy();
        assert_eq!(c.worst_comm_latency(), 10);
    }

    #[test]
    fn test_formation_quality() {
        let c = test_convoy();
        assert!(c.formation_quality() > 0.5);
    }

    #[test]
    fn test_empty_convoy() {
        let c = Convoy::new("empty");
        assert_eq!(c.size(), 0);
        assert!(c.leader().is_none());
        assert_eq!(c.average_speed(), 0.0);
    }

    #[test]
    fn test_planner_can_form() {
        let p = ConvoyPlanner::new();
        assert!(p.can_form_convoy(3, 80.0));
        assert!(!p.can_form_convoy(1, 80.0));
        assert!(!p.can_form_convoy(3, 10.0));
    }

    #[test]
    fn test_planner_recommended_formation() {
        let p = ConvoyPlanner::new();
        assert_eq!(p.recommended_formation(2), FormationType::Column);
        assert_eq!(p.recommended_formation(4), FormationType::Staggered);
        assert_eq!(p.recommended_formation(6), FormationType::Echelon);
        assert_eq!(p.recommended_formation(8), FormationType::Diamond);
    }

    #[test]
    fn test_fuel_savings_leader() {
        let p = ConvoyPlanner::new();
        assert!((p.fuel_savings_pct(0, 80.0) - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_fuel_savings_follower() {
        let p = ConvoyPlanner::new();
        let savings = p.fuel_savings_pct(2, 80.0);
        assert!(savings > 10.0);
    }

    #[test]
    fn test_fuel_savings_cap() {
        let p = ConvoyPlanner::new();
        let savings = p.fuel_savings_pct(10, 120.0);
        assert!(savings <= 25.0);
    }

    #[test]
    fn test_default_convoy() {
        let c = Convoy::default();
        assert_eq!(c.name, "default");
        assert_eq!(c.size(), 0);
    }

    #[test]
    fn test_default_planner() {
        let p = ConvoyPlanner::default();
        assert_eq!(p.min_vehicles, 2);
        assert_eq!(p.max_vehicles, 10);
    }
}
