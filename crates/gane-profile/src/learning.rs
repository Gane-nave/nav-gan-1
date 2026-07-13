//! Driving pattern learning engine.
//!
//! Learns from user behaviour to personalise route suggestions,
//! departure time recommendations, and comfort preferences.

use std::collections::HashMap;

/// A recorded trip summary.
#[derive(Debug, Clone)]
pub struct TripRecord {
    pub from_label: String,
    pub to_label: String,
    pub departure_hour: u8,
    pub day_of_week: u8,
    pub duration_s: u64,
    pub distance_m: f64,
    pub route_id: Option<String>,
}

/// Time-of-day bucket for pattern analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeBucket {
    EarlyMorning, // 5-7
    MorningRush,  // 7-9
    Morning,      // 9-12
    Afternoon,    // 12-15
    EveningRush,  // 15-18
    Evening,      // 18-21
    Night,        // 21-5
}

impl TimeBucket {
    pub fn from_hour(h: u8) -> Self {
        match h {
            5..=6 => Self::EarlyMorning,
            7..=8 => Self::MorningRush,
            9..=11 => Self::Morning,
            12..=14 => Self::Afternoon,
            15..=17 => Self::EveningRush,
            18..=20 => Self::Evening,
            _ => Self::Night,
        }
    }
}

/// A learned route pattern (from → to at a certain time).
#[derive(Debug, Clone)]
pub struct RoutePattern {
    pub from_label: String,
    pub to_label: String,
    pub preferred_time: TimeBucket,
    pub trip_count: u32,
    pub avg_duration_s: f64,
    pub preferred_route_id: Option<String>,
}

/// Driving style metrics (learned over time).
#[derive(Debug, Clone)]
pub struct DrivingStyle {
    /// Average acceleration aggressiveness 0.0 (gentle) .. 1.0 (aggressive).
    pub acceleration_profile: f64,
    /// Average braking aggressiveness.
    pub braking_profile: f64,
    /// Tendency to exceed speed limit 0.0 (never) .. 1.0 (always).
    pub speed_tendency: f64,
    /// Comfort preference: prefers smooth roads vs shortcuts.
    pub comfort_vs_speed: f64,
    /// Number of samples contributing to these metrics.
    pub sample_count: u32,
}

impl Default for DrivingStyle {
    fn default() -> Self {
        Self {
            acceleration_profile: 0.5,
            braking_profile: 0.5,
            speed_tendency: 0.0,
            comfort_vs_speed: 0.5,
            sample_count: 0,
        }
    }
}

impl DrivingStyle {
    /// Update with a new observation using exponential moving average.
    pub fn update(&mut self, accel: f64, brake: f64, speed_excess: f64, comfort: f64) {
        let alpha = if self.sample_count < 10 { 0.3 } else { 0.1 };
        self.acceleration_profile = lerp(self.acceleration_profile, accel.clamp(0.0, 1.0), alpha);
        self.braking_profile = lerp(self.braking_profile, brake.clamp(0.0, 1.0), alpha);
        self.speed_tendency = lerp(self.speed_tendency, speed_excess.clamp(0.0, 1.0), alpha);
        self.comfort_vs_speed = lerp(self.comfort_vs_speed, comfort.clamp(0.0, 1.0), alpha);
        self.sample_count += 1;
    }

    /// Whether the driver tends to drive aggressively.
    pub fn is_aggressive(&self) -> bool {
        self.sample_count >= 5 && (self.acceleration_profile > 0.7 || self.braking_profile > 0.7)
    }

    /// Whether the driver prioritises comfort.
    pub fn prefers_comfort(&self) -> bool {
        self.sample_count >= 5 && self.comfort_vs_speed > 0.6
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Learning engine that accumulates trip data.
#[derive(Debug)]
pub struct LearningEngine {
    trips: Vec<TripRecord>,
    style: DrivingStyle,
    max_trips: usize,
}

impl LearningEngine {
    pub fn new(max_trips: usize) -> Self {
        Self {
            trips: Vec::new(),
            style: DrivingStyle::default(),
            max_trips,
        }
    }

    /// Record a completed trip.
    pub fn record_trip(&mut self, trip: TripRecord) {
        if self.trips.len() >= self.max_trips {
            self.trips.remove(0);
        }
        self.trips.push(trip);
    }

    /// Extract route patterns from recorded trips.
    pub fn extract_patterns(&self) -> Vec<RoutePattern> {
        let mut groups: HashMap<(String, String, TimeBucket), Vec<&TripRecord>> = HashMap::new();
        for trip in &self.trips {
            let bucket = TimeBucket::from_hour(trip.departure_hour);
            groups
                .entry((trip.from_label.clone(), trip.to_label.clone(), bucket))
                .or_default()
                .push(trip);
        }

        groups
            .into_iter()
            .map(|((from, to, time), trips)| {
                let count = trips.len() as u32;
                let avg = trips.iter().map(|t| t.duration_s as f64).sum::<f64>() / count as f64;
                // Most common route_id
                let mut route_counts: HashMap<&str, u32> = HashMap::new();
                for t in &trips {
                    if let Some(ref r) = t.route_id {
                        *route_counts.entry(r.as_str()).or_default() += 1;
                    }
                }
                let preferred = route_counts
                    .into_iter()
                    .max_by_key(|&(_, c)| c)
                    .map(|(r, _)| r.to_string());

                RoutePattern {
                    from_label: from,
                    to_label: to,
                    preferred_time: time,
                    trip_count: count,
                    avg_duration_s: avg,
                    preferred_route_id: preferred,
                }
            })
            .collect()
    }

    /// Get a suggested departure time for a route.
    pub fn suggested_departure(&self, from: &str, to: &str) -> Option<TimeBucket> {
        let patterns = self.extract_patterns();
        patterns
            .iter()
            .filter(|p| p.from_label == from && p.to_label == to)
            .max_by_key(|p| p.trip_count)
            .map(|p| p.preferred_time)
    }

    /// Update driving style with new observation.
    pub fn update_style(&mut self, accel: f64, brake: f64, speed_excess: f64, comfort: f64) {
        self.style.update(accel, brake, speed_excess, comfort);
    }

    pub fn driving_style(&self) -> &DrivingStyle {
        &self.style
    }

    pub fn trip_count(&self) -> usize {
        self.trips.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trip(from: &str, to: &str, hour: u8, dur: u64) -> TripRecord {
        TripRecord {
            from_label: from.into(),
            to_label: to.into(),
            departure_hour: hour,
            day_of_week: 1,
            duration_s: dur,
            distance_m: 5000.0,
            route_id: Some("r1".into()),
        }
    }

    #[test]
    fn test_time_bucket() {
        assert_eq!(TimeBucket::from_hour(7), TimeBucket::MorningRush);
        assert_eq!(TimeBucket::from_hour(16), TimeBucket::EveningRush);
        assert_eq!(TimeBucket::from_hour(23), TimeBucket::Night);
        assert_eq!(TimeBucket::from_hour(3), TimeBucket::Night);
    }

    #[test]
    fn test_record_trip() {
        let mut engine = LearningEngine::new(100);
        engine.record_trip(trip("Home", "Work", 8, 1800));
        assert_eq!(engine.trip_count(), 1);
    }

    #[test]
    fn test_max_trips_eviction() {
        let mut engine = LearningEngine::new(2);
        engine.record_trip(trip("A", "B", 8, 100));
        engine.record_trip(trip("C", "D", 9, 200));
        engine.record_trip(trip("E", "F", 10, 300));
        assert_eq!(engine.trip_count(), 2);
    }

    #[test]
    fn test_extract_patterns() {
        let mut engine = LearningEngine::new(100);
        engine.record_trip(trip("Home", "Work", 8, 1800));
        engine.record_trip(trip("Home", "Work", 8, 2000));
        engine.record_trip(trip("Home", "Work", 8, 1900));
        let patterns = engine.extract_patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].trip_count, 3);
        assert!((patterns[0].avg_duration_s - 1900.0).abs() < 0.01);
    }

    #[test]
    fn test_suggested_departure() {
        let mut engine = LearningEngine::new(100);
        for _ in 0..5 {
            engine.record_trip(trip("Home", "Work", 7, 1800));
        }
        engine.record_trip(trip("Home", "Work", 16, 2000));
        let suggested = engine.suggested_departure("Home", "Work").unwrap();
        assert_eq!(suggested, TimeBucket::MorningRush);
    }

    #[test]
    fn test_driving_style_default() {
        let style = DrivingStyle::default();
        assert!((style.acceleration_profile - 0.5).abs() < f64::EPSILON);
        assert_eq!(style.sample_count, 0);
    }

    #[test]
    fn test_driving_style_update() {
        let mut style = DrivingStyle::default();
        // Update toward aggressive
        for _ in 0..20 {
            style.update(0.9, 0.9, 0.5, 0.3);
        }
        assert!(style.acceleration_profile > 0.7);
        assert!(style.is_aggressive());
    }

    #[test]
    fn test_driving_style_comfort() {
        let mut style = DrivingStyle::default();
        for _ in 0..10 {
            style.update(0.3, 0.3, 0.0, 0.9);
        }
        assert!(style.prefers_comfort());
        assert!(!style.is_aggressive());
    }

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 1.0, 0.5) - 0.5).abs() < f64::EPSILON);
        assert!((lerp(0.0, 1.0, 0.0) - 0.0).abs() < f64::EPSILON);
        assert!((lerp(0.0, 1.0, 1.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_aggressive_needs_samples() {
        let style = DrivingStyle {
            acceleration_profile: 0.9,
            ..Default::default()
        };
        // Not enough samples
        assert!(!style.is_aggressive());
    }

    #[test]
    fn test_preferred_route_id() {
        let mut engine = LearningEngine::new(100);
        let mut t1 = trip("A", "B", 8, 100);
        t1.route_id = Some("fast".into());
        let mut t2 = trip("A", "B", 8, 100);
        t2.route_id = Some("fast".into());
        let mut t3 = trip("A", "B", 8, 100);
        t3.route_id = Some("scenic".into());
        engine.record_trip(t1);
        engine.record_trip(t2);
        engine.record_trip(t3);
        let patterns = engine.extract_patterns();
        assert_eq!(patterns[0].preferred_route_id, Some("fast".to_string()));
    }
}
