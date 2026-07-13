/// Fuel optimization engine: eco-routing, consumption prediction, regenerative braking.
#[derive(Debug, Clone, PartialEq)]
pub enum DrivetrainType {
    Gasoline,
    Diesel,
    Hybrid,
    PluginHybrid,
    BatteryElectric,
    Hydrogen,
}

impl DrivetrainType {
    pub fn base_efficiency(&self) -> f64 {
        match self {
            DrivetrainType::Gasoline => 0.30,
            DrivetrainType::Diesel => 0.35,
            DrivetrainType::Hybrid => 0.45,
            DrivetrainType::PluginHybrid => 0.55,
            DrivetrainType::BatteryElectric => 0.85,
            DrivetrainType::Hydrogen => 0.50,
        }
    }

    pub fn can_regen_brake(&self) -> bool {
        matches!(
            self,
            DrivetrainType::Hybrid | DrivetrainType::PluginHybrid | DrivetrainType::BatteryElectric
        )
    }

    pub fn co2_grams_per_kwh(&self) -> f64 {
        match self {
            DrivetrainType::Gasoline => 250.0,
            DrivetrainType::Diesel => 220.0,
            DrivetrainType::Hybrid => 150.0,
            DrivetrainType::PluginHybrid => 80.0,
            DrivetrainType::BatteryElectric => 0.0,
            DrivetrainType::Hydrogen => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VehicleProfile {
    pub drivetrain: DrivetrainType,
    pub mass_kg: f64,
    pub drag_coefficient: f64,
    pub frontal_area_m2: f64,
    pub rolling_resistance: f64,
    pub regen_efficiency: f64,
}

impl Default for VehicleProfile {
    fn default() -> Self {
        Self {
            drivetrain: DrivetrainType::Gasoline,
            mass_kg: 1500.0,
            drag_coefficient: 0.30,
            frontal_area_m2: 2.2,
            rolling_resistance: 0.012,
            regen_efficiency: 0.0,
        }
    }
}

impl VehicleProfile {
    pub fn electric() -> Self {
        Self {
            drivetrain: DrivetrainType::BatteryElectric,
            mass_kg: 1800.0,
            drag_coefficient: 0.23,
            frontal_area_m2: 2.3,
            rolling_resistance: 0.010,
            regen_efficiency: 0.7,
        }
    }

    pub fn rolling_force(&self) -> f64 {
        self.mass_kg * 9.81 * self.rolling_resistance
    }

    pub fn aero_force(&self, speed_ms: f64) -> f64 {
        0.5 * 1.225 * self.drag_coefficient * self.frontal_area_m2 * speed_ms * speed_ms
    }

    pub fn total_resistance(&self, speed_ms: f64) -> f64 {
        self.rolling_force() + self.aero_force(speed_ms)
    }

    pub fn grade_force(&self, grade_pct: f64) -> f64 {
        self.mass_kg * 9.81 * (grade_pct / 100.0)
    }
}

#[derive(Debug, Clone)]
pub struct FuelSegment {
    pub distance_m: f64,
    pub speed_kmh: f64,
    pub grade_pct: f64,
    pub headwind_ms: f64,
}

impl FuelSegment {
    pub fn new(distance_m: f64, speed_kmh: f64) -> Self {
        Self {
            distance_m,
            speed_kmh,
            grade_pct: 0.0,
            headwind_ms: 0.0,
        }
    }

    pub fn energy_kwh(&self, vehicle: &VehicleProfile) -> f64 {
        let v = self.speed_kmh / 3.6;
        let effective_v = (v + self.headwind_ms).max(0.0);
        let rolling = vehicle.rolling_force();
        let aero = 0.5
            * 1.225
            * vehicle.drag_coefficient
            * vehicle.frontal_area_m2
            * effective_v
            * effective_v;
        let grade = vehicle.grade_force(self.grade_pct);
        let total_force = rolling + aero + grade;
        let work_j = total_force * self.distance_m;
        let efficiency = vehicle.drivetrain.base_efficiency();

        if work_j < 0.0 && vehicle.drivetrain.can_regen_brake() {
            // Regenerative braking recovers energy
            work_j.abs() * vehicle.regen_efficiency / (3_600_000.0)
        } else if efficiency < f64::EPSILON {
            0.0
        } else {
            (work_j / efficiency) / 3_600_000.0 // Joules to kWh
        }
    }

    pub fn co2_grams(&self, vehicle: &VehicleProfile) -> f64 {
        let energy = self.energy_kwh(vehicle);
        energy * vehicle.drivetrain.co2_grams_per_kwh()
    }
}

#[derive(Debug, Clone)]
pub struct FuelRoute {
    pub segments: Vec<FuelSegment>,
}

impl Default for FuelRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelRoute {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, seg: FuelSegment) {
        self.segments.push(seg);
    }

    pub fn total_distance_km(&self) -> f64 {
        self.segments.iter().map(|s| s.distance_m).sum::<f64>() / 1000.0
    }

    pub fn total_energy_kwh(&self, vehicle: &VehicleProfile) -> f64 {
        self.segments.iter().map(|s| s.energy_kwh(vehicle)).sum()
    }

    pub fn total_co2_grams(&self, vehicle: &VehicleProfile) -> f64 {
        self.segments.iter().map(|s| s.co2_grams(vehicle)).sum()
    }

    pub fn efficiency_kwh_per_km(&self, vehicle: &VehicleProfile) -> f64 {
        let dist = self.total_distance_km();
        if dist < f64::EPSILON {
            return 0.0;
        }
        self.total_energy_kwh(vehicle) / dist
    }

    pub fn optimal_speed_range(&self, vehicle: &VehicleProfile) -> (f64, f64) {
        // Find speed range that minimizes energy per km
        let mut best_speed = 80.0;
        let mut best_eff = f64::INFINITY;
        for speed in (30..=130).step_by(5) {
            let s = speed as f64;
            let seg = FuelSegment::new(1000.0, s);
            let eff = seg.energy_kwh(vehicle) / 1.0; // per km
            if eff < best_eff {
                best_eff = eff;
                best_speed = s;
            }
        }
        (
            (best_speed - 10.0).max(30.0),
            (best_speed + 10.0).min(130.0),
        )
    }

    pub fn eco_score(&self, vehicle: &VehicleProfile) -> f64 {
        let eff = self.efficiency_kwh_per_km(vehicle);
        if eff < f64::EPSILON {
            return 100.0;
        }
        let baseline = match vehicle.drivetrain {
            DrivetrainType::BatteryElectric => 0.20,
            DrivetrainType::Hybrid | DrivetrainType::PluginHybrid => 0.40,
            _ => 0.80,
        };
        ((baseline / eff) * 100.0).clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gasoline_efficiency() {
        assert!((DrivetrainType::Gasoline.base_efficiency() - 0.30).abs() < 0.01);
    }

    #[test]
    fn test_ev_efficiency() {
        assert!((DrivetrainType::BatteryElectric.base_efficiency() - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_regen_brake() {
        assert!(DrivetrainType::BatteryElectric.can_regen_brake());
        assert!(DrivetrainType::Hybrid.can_regen_brake());
        assert!(!DrivetrainType::Gasoline.can_regen_brake());
    }

    #[test]
    fn test_ev_zero_co2() {
        assert!((DrivetrainType::BatteryElectric.co2_grams_per_kwh()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_default_profile() {
        let p = VehicleProfile::default();
        assert!((p.mass_kg - 1500.0).abs() < 0.01);
    }

    #[test]
    fn test_electric_profile() {
        let p = VehicleProfile::electric();
        assert!(p.regen_efficiency > 0.5);
        assert!(p.drivetrain.can_regen_brake());
    }

    #[test]
    fn test_rolling_force() {
        let p = VehicleProfile::default();
        let f = p.rolling_force();
        assert!(f > 100.0 && f < 300.0);
    }

    #[test]
    fn test_aero_force_increases_with_speed() {
        let p = VehicleProfile::default();
        let slow = p.aero_force(10.0);
        let fast = p.aero_force(30.0);
        assert!(fast > slow * 5.0); // quadratic with speed
    }

    #[test]
    fn test_grade_force_uphill() {
        let p = VehicleProfile::default();
        let f = p.grade_force(5.0);
        assert!(f > 500.0); // significant force on 5% grade
    }

    #[test]
    fn test_grade_force_downhill() {
        let p = VehicleProfile::default();
        let f = p.grade_force(-5.0);
        assert!(f < 0.0);
    }

    #[test]
    fn test_segment_energy_positive() {
        let p = VehicleProfile::default();
        let s = FuelSegment::new(1000.0, 80.0);
        assert!(s.energy_kwh(&p) > 0.0);
    }

    #[test]
    fn test_ev_less_energy() {
        let gas = VehicleProfile::default();
        let ev = VehicleProfile::electric();
        let s = FuelSegment::new(1000.0, 80.0);
        assert!(s.energy_kwh(&ev) < s.energy_kwh(&gas));
    }

    #[test]
    fn test_co2_gasoline() {
        let p = VehicleProfile::default();
        let s = FuelSegment::new(1000.0, 80.0);
        assert!(s.co2_grams(&p) > 0.0);
    }

    #[test]
    fn test_co2_ev_zero() {
        let p = VehicleProfile::electric();
        let s = FuelSegment::new(1000.0, 80.0);
        assert!((s.co2_grams(&p)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_route_total_distance() {
        let mut r = FuelRoute::new();
        r.add_segment(FuelSegment::new(1000.0, 80.0));
        r.add_segment(FuelSegment::new(2000.0, 60.0));
        assert!((r.total_distance_km() - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_route() {
        let r = FuelRoute::new();
        let p = VehicleProfile::default();
        assert_eq!(r.total_distance_km(), 0.0);
        assert_eq!(r.efficiency_kwh_per_km(&p), 0.0);
    }

    #[test]
    fn test_optimal_speed_range() {
        let p = VehicleProfile::default();
        let r = FuelRoute::new();
        let (lo, hi) = r.optimal_speed_range(&p);
        assert!(lo >= 30.0 && hi <= 130.0);
        assert!(hi > lo);
    }

    #[test]
    fn test_eco_score() {
        let p = VehicleProfile::default();
        let mut r = FuelRoute::new();
        r.add_segment(FuelSegment::new(10000.0, 80.0));
        let score = r.eco_score(&p);
        assert!(score > 0.0 && score <= 100.0);
    }

    #[test]
    fn test_headwind_increases_energy() {
        let p = VehicleProfile::default();
        let calm = FuelSegment::new(1000.0, 80.0);
        let mut windy = FuelSegment::new(1000.0, 80.0);
        windy.headwind_ms = 10.0;
        assert!(windy.energy_kwh(&p) > calm.energy_kwh(&p));
    }
}
