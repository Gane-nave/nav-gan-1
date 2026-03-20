/// Driver scoring engine: behavior analysis, safety rating, eco-driving assessment.
#[derive(Debug, Clone, PartialEq)]
pub enum DrivingEvent {
    HardBrake,
    HardAcceleration,
    SharpTurn,
    Speeding,
    TailGating,
    LaneDrift,
    SmoothDrive,
    EcoBrake,
    SafeFollowing,
    SignalUse,
}

impl DrivingEvent {
    pub fn penalty(&self) -> f64 {
        match self {
            DrivingEvent::HardBrake => 5.0,
            DrivingEvent::HardAcceleration => 3.0,
            DrivingEvent::SharpTurn => 4.0,
            DrivingEvent::Speeding => 8.0,
            DrivingEvent::TailGating => 7.0,
            DrivingEvent::LaneDrift => 6.0,
            DrivingEvent::SmoothDrive => 0.0,
            DrivingEvent::EcoBrake => 0.0,
            DrivingEvent::SafeFollowing => 0.0,
            DrivingEvent::SignalUse => 0.0,
        }
    }

    pub fn bonus(&self) -> f64 {
        match self {
            DrivingEvent::SmoothDrive => 2.0,
            DrivingEvent::EcoBrake => 3.0,
            DrivingEvent::SafeFollowing => 2.5,
            DrivingEvent::SignalUse => 1.0,
            _ => 0.0,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.bonus() > 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.penalty() > 0.0
    }

    pub fn category(&self) -> &str {
        match self {
            DrivingEvent::HardBrake | DrivingEvent::HardAcceleration | DrivingEvent::SharpTurn => {
                "aggressiveness"
            }
            DrivingEvent::Speeding | DrivingEvent::TailGating | DrivingEvent::LaneDrift => "safety",
            DrivingEvent::SmoothDrive | DrivingEvent::EcoBrake => "eco",
            DrivingEvent::SafeFollowing | DrivingEvent::SignalUse => "compliance",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TripRecord {
    pub events: Vec<DrivingEvent>,
    pub distance_km: f64,
    pub duration_min: f64,
}

impl TripRecord {
    pub fn new(distance_km: f64, duration_min: f64) -> Self {
        Self {
            events: Vec::new(),
            distance_km,
            duration_min,
        }
    }

    pub fn add_event(&mut self, event: DrivingEvent) {
        self.events.push(event);
    }

    pub fn total_penalties(&self) -> f64 {
        self.events.iter().map(|e| e.penalty()).sum()
    }

    pub fn total_bonuses(&self) -> f64 {
        self.events.iter().map(|e| e.bonus()).sum()
    }

    pub fn negative_event_count(&self) -> usize {
        self.events.iter().filter(|e| e.is_negative()).count()
    }

    pub fn positive_event_count(&self) -> usize {
        self.events.iter().filter(|e| e.is_positive()).count()
    }

    pub fn events_per_km(&self) -> f64 {
        if self.distance_km < f64::EPSILON {
            return 0.0;
        }
        self.negative_event_count() as f64 / self.distance_km
    }

    pub fn trip_score(&self) -> f64 {
        let base = 100.0;
        let penalties = self.total_penalties();
        let bonuses = self.total_bonuses();
        (base - penalties + bonuses).clamp(0.0, 100.0)
    }

    pub fn average_speed_kmh(&self) -> f64 {
        if self.duration_min < f64::EPSILON {
            return 0.0;
        }
        self.distance_km / (self.duration_min / 60.0)
    }
}

#[derive(Debug, Clone)]
pub struct DriverProfile {
    pub trips: Vec<TripRecord>,
}

impl Default for DriverProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverProfile {
    pub fn new() -> Self {
        Self { trips: Vec::new() }
    }

    pub fn add_trip(&mut self, trip: TripRecord) {
        self.trips.push(trip);
    }

    pub fn total_distance_km(&self) -> f64 {
        self.trips.iter().map(|t| t.distance_km).sum()
    }

    pub fn total_trips(&self) -> usize {
        self.trips.len()
    }

    pub fn overall_score(&self) -> f64 {
        if self.trips.is_empty() {
            return 100.0;
        }
        let total: f64 = self.trips.iter().map(|t| t.trip_score()).sum();
        total / self.trips.len() as f64
    }

    pub fn safety_rating(&self) -> &str {
        let score = self.overall_score();
        if score >= 90.0 {
            "Excellent"
        } else if score >= 75.0 {
            "Good"
        } else if score >= 60.0 {
            "Fair"
        } else if score >= 40.0 {
            "Poor"
        } else {
            "Dangerous"
        }
    }

    pub fn worst_trip_score(&self) -> f64 {
        self.trips
            .iter()
            .map(|t| t.trip_score())
            .fold(f64::INFINITY, f64::min)
    }

    pub fn best_trip_score(&self) -> f64 {
        self.trips
            .iter()
            .map(|t| t.trip_score())
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn total_negative_events(&self) -> usize {
        self.trips.iter().map(|t| t.negative_event_count()).sum()
    }

    pub fn negative_events_per_km(&self) -> f64 {
        let dist = self.total_distance_km();
        if dist < f64::EPSILON {
            return 0.0;
        }
        self.total_negative_events() as f64 / dist
    }

    pub fn improvement_trend(&self) -> f64 {
        if self.trips.len() < 2 {
            return 0.0;
        }
        let half = self.trips.len() / 2;
        let first_half: f64 = self.trips[..half]
            .iter()
            .map(|t| t.trip_score())
            .sum::<f64>()
            / half as f64;
        let second_half: f64 = self.trips[half..]
            .iter()
            .map(|t| t.trip_score())
            .sum::<f64>()
            / (self.trips.len() - half) as f64;
        second_half - first_half
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penalty_values() {
        assert!((DrivingEvent::Speeding.penalty() - 8.0).abs() < 0.01);
        assert!((DrivingEvent::SmoothDrive.penalty()).abs() < 0.01);
    }

    #[test]
    fn test_bonus_values() {
        assert!((DrivingEvent::EcoBrake.bonus() - 3.0).abs() < 0.01);
        assert!((DrivingEvent::Speeding.bonus()).abs() < 0.01);
    }

    #[test]
    fn test_positive_negative() {
        assert!(DrivingEvent::SmoothDrive.is_positive());
        assert!(!DrivingEvent::SmoothDrive.is_negative());
        assert!(DrivingEvent::Speeding.is_negative());
        assert!(!DrivingEvent::Speeding.is_positive());
    }

    #[test]
    fn test_category() {
        assert_eq!(DrivingEvent::HardBrake.category(), "aggressiveness");
        assert_eq!(DrivingEvent::Speeding.category(), "safety");
        assert_eq!(DrivingEvent::EcoBrake.category(), "eco");
        assert_eq!(DrivingEvent::SignalUse.category(), "compliance");
    }

    #[test]
    fn test_trip_score_perfect() {
        let trip = TripRecord::new(50.0, 30.0);
        assert!((trip.trip_score() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_trip_score_with_penalties() {
        let mut trip = TripRecord::new(50.0, 30.0);
        trip.add_event(DrivingEvent::Speeding);
        trip.add_event(DrivingEvent::HardBrake);
        assert!((trip.trip_score() - 87.0).abs() < 0.01);
    }

    #[test]
    fn test_trip_score_with_bonuses() {
        let mut trip = TripRecord::new(50.0, 30.0);
        trip.add_event(DrivingEvent::SmoothDrive);
        trip.add_event(DrivingEvent::EcoBrake);
        assert!((trip.trip_score() - 100.0).abs() < 0.01); // clamped at 100
    }

    #[test]
    fn test_trip_score_floor() {
        let mut trip = TripRecord::new(50.0, 30.0);
        for _ in 0..20 {
            trip.add_event(DrivingEvent::Speeding);
        }
        assert!((trip.trip_score()).abs() < 0.01); // clamped at 0
    }

    #[test]
    fn test_events_per_km() {
        let mut trip = TripRecord::new(10.0, 15.0);
        trip.add_event(DrivingEvent::HardBrake);
        trip.add_event(DrivingEvent::Speeding);
        assert!((trip.events_per_km() - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_events_per_km_zero_distance() {
        let trip = TripRecord::new(0.0, 0.0);
        assert_eq!(trip.events_per_km(), 0.0);
    }

    #[test]
    fn test_average_speed() {
        let trip = TripRecord::new(100.0, 60.0);
        assert!((trip.average_speed_kmh() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_driver_profile_empty() {
        let p = DriverProfile::new();
        assert_eq!(p.total_trips(), 0);
        assert!((p.overall_score() - 100.0).abs() < 0.01);
        assert_eq!(p.safety_rating(), "Excellent");
    }

    #[test]
    fn test_overall_score() {
        let mut p = DriverProfile::new();
        let mut t1 = TripRecord::new(50.0, 30.0);
        t1.add_event(DrivingEvent::Speeding); // 100 - 8 = 92
        p.add_trip(t1);
        let t2 = TripRecord::new(50.0, 30.0); // 100
        p.add_trip(t2);
        assert!((p.overall_score() - 96.0).abs() < 0.01);
    }

    #[test]
    fn test_safety_rating() {
        let mut p = DriverProfile::new();
        p.add_trip(TripRecord::new(50.0, 30.0));
        assert_eq!(p.safety_rating(), "Excellent");
    }

    #[test]
    fn test_worst_best_trip() {
        let mut p = DriverProfile::new();
        let mut bad = TripRecord::new(50.0, 30.0);
        for _ in 0..5 {
            bad.add_event(DrivingEvent::Speeding);
        }
        p.add_trip(bad);
        p.add_trip(TripRecord::new(50.0, 30.0));
        assert!(p.worst_trip_score() < 70.0);
        assert!((p.best_trip_score() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_improvement_trend() {
        let mut p = DriverProfile::new();
        // First half: bad trips
        for _ in 0..3 {
            let mut t = TripRecord::new(50.0, 30.0);
            t.add_event(DrivingEvent::Speeding);
            t.add_event(DrivingEvent::HardBrake);
            p.add_trip(t);
        }
        // Second half: good trips
        for _ in 0..3 {
            p.add_trip(TripRecord::new(50.0, 30.0));
        }
        assert!(p.improvement_trend() > 0.0);
    }

    #[test]
    fn test_negative_events_per_km() {
        let mut p = DriverProfile::new();
        let mut t = TripRecord::new(100.0, 60.0);
        t.add_event(DrivingEvent::HardBrake);
        t.add_event(DrivingEvent::Speeding);
        p.add_trip(t);
        assert!((p.negative_events_per_km() - 0.02).abs() < 0.01);
    }

    #[test]
    fn test_total_distance() {
        let mut p = DriverProfile::new();
        p.add_trip(TripRecord::new(50.0, 30.0));
        p.add_trip(TripRecord::new(100.0, 60.0));
        assert!((p.total_distance_km() - 150.0).abs() < 0.01);
    }
}
