//! Public transit routing engine.
//!
//! GTFS-style schedule modelling, real-time arrival predictions,
//! fare calculation, and multi-leg journey planning.

/// Transit mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitMode {
    Bus,
    Tram,
    Metro,
    Train,
    Ferry,
    CableCar,
}

impl TransitMode {
    /// Typical reliability score 0.0 .. 1.0.
    pub fn reliability(self) -> f64 {
        match self {
            Self::Metro => 0.95,
            Self::Tram => 0.88,
            Self::Train => 0.85,
            Self::CableCar => 0.90,
            Self::Bus => 0.75,
            Self::Ferry => 0.70,
        }
    }

    /// Average boarding time in seconds.
    pub fn boarding_time_s(self) -> f64 {
        match self {
            Self::Bus | Self::Tram => 15.0,
            Self::Metro | Self::Train => 30.0,
            Self::Ferry => 120.0,
            Self::CableCar => 60.0,
        }
    }
}

/// A transit stop.
#[derive(Debug, Clone)]
pub struct TransitStop {
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub modes: Vec<TransitMode>,
    pub wheelchair_accessible: bool,
}

/// A scheduled departure.
#[derive(Debug, Clone)]
pub struct Departure {
    pub route_id: String,
    pub route_name: String,
    pub mode: TransitMode,
    pub stop_id: String,
    pub scheduled_time_s: u64,
    pub estimated_delay_s: i64,
    pub destination: String,
}

impl Departure {
    /// Predicted actual departure time.
    pub fn predicted_time_s(&self) -> u64 {
        (self.scheduled_time_s as i64 + self.estimated_delay_s).max(0) as u64
    }
}

/// A single leg of a transit journey.
#[derive(Debug, Clone)]
pub struct TransitLeg {
    pub mode: TransitMode,
    pub route_name: String,
    pub board_stop: String,
    pub alight_stop: String,
    pub board_time_s: u64,
    pub alight_time_s: u64,
    pub num_stops: u32,
    pub fare_cents: u32,
}

impl TransitLeg {
    /// Duration of this leg in seconds.
    pub fn duration_s(&self) -> u64 {
        self.alight_time_s.saturating_sub(self.board_time_s)
    }
}

/// Complete transit journey.
#[derive(Debug, Clone)]
pub struct TransitJourney {
    pub legs: Vec<TransitLeg>,
    /// Walking segments between legs (transfer walks), in seconds.
    pub transfer_walks_s: Vec<u64>,
}

impl TransitJourney {
    /// Total journey time from first boarding to last alighting + transfers.
    pub fn total_time_s(&self) -> u64 {
        if self.legs.is_empty() {
            return 0;
        }
        let ride =
            self.legs.last().unwrap().alight_time_s - self.legs.first().unwrap().board_time_s;
        let walks: u64 = self.transfer_walks_s.iter().sum();
        ride + walks
    }

    /// Total fare in cents.
    pub fn total_fare_cents(&self) -> u32 {
        self.legs.iter().map(|l| l.fare_cents).sum()
    }

    /// Number of transfers.
    pub fn transfers(&self) -> usize {
        self.legs.len().saturating_sub(1)
    }

    /// Journey reliability (product of leg reliabilities).
    pub fn reliability(&self) -> f64 {
        self.legs.iter().map(|l| l.mode.reliability()).product()
    }
}

/// Transit routing preferences.
#[derive(Debug, Clone)]
pub struct TransitPreferences {
    /// Maximum acceptable transfers.
    pub max_transfers: usize,
    /// Maximum walking distance to/from stops in metres.
    pub max_walk_m: f64,
    /// Prefer fewer transfers over faster routes.
    pub prefer_fewer_transfers: bool,
    /// Require wheelchair-accessible stops.
    pub wheelchair: bool,
    /// Preferred modes (empty = all).
    pub preferred_modes: Vec<TransitMode>,
}

impl Default for TransitPreferences {
    fn default() -> Self {
        Self {
            max_transfers: 3,
            max_walk_m: 800.0,
            prefer_fewer_transfers: false,
            wheelchair: false,
            preferred_modes: Vec::new(),
        }
    }
}

/// Transit schedule engine.
#[derive(Debug)]
pub struct TransitEngine {
    stops: Vec<TransitStop>,
    prefs: TransitPreferences,
}

impl TransitEngine {
    pub fn new(stops: Vec<TransitStop>, prefs: TransitPreferences) -> Self {
        Self { stops, prefs }
    }

    /// Find stops within walking distance of a coordinate.
    pub fn nearby_stops(&self, lat: f64, lon: f64, radius_m: f64) -> Vec<&TransitStop> {
        let radius_deg = radius_m / 111_320.0;
        self.stops
            .iter()
            .filter(|s| {
                let dlat = s.lat - lat;
                let dlon = s.lon - lon;
                (dlat * dlat + dlon * dlon).sqrt() <= radius_deg
            })
            .filter(|s| !self.prefs.wheelchair || s.wheelchair_accessible)
            .collect()
    }

    /// Check if a journey meets preferences.
    pub fn is_acceptable(&self, journey: &TransitJourney) -> bool {
        if journey.transfers() > self.prefs.max_transfers {
            return false;
        }
        if !self.prefs.preferred_modes.is_empty() {
            for leg in &journey.legs {
                if !self.prefs.preferred_modes.contains(&leg.mode) {
                    return false;
                }
            }
        }
        true
    }

    /// Score a journey (lower = better).
    pub fn journey_score(&self, journey: &TransitJourney) -> f64 {
        let time = journey.total_time_s() as f64;
        let transfer_penalty = if self.prefs.prefer_fewer_transfers {
            journey.transfers() as f64 * 300.0
        } else {
            journey.transfers() as f64 * 60.0
        };
        let reliability_bonus = (1.0 - journey.reliability()) * 200.0;
        time + transfer_penalty + reliability_bonus
    }

    /// Select the best journey from candidates.
    pub fn best_journey<'a>(&self, candidates: &'a [TransitJourney]) -> Option<&'a TransitJourney> {
        candidates
            .iter()
            .filter(|j| self.is_acceptable(j))
            .min_by(|a, b| {
                self.journey_score(a)
                    .partial_cmp(&self.journey_score(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Number of stops in the engine.
    pub fn stop_count(&self) -> usize {
        self.stops.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stop(id: &str, lat: f64, lon: f64) -> TransitStop {
        TransitStop {
            id: id.into(),
            name: format!("Stop {id}"),
            lat,
            lon,
            modes: vec![TransitMode::Bus],
            wheelchair_accessible: true,
        }
    }

    fn make_journey(legs: Vec<(TransitMode, u64, u64, u32)>) -> TransitJourney {
        TransitJourney {
            legs: legs
                .into_iter()
                .enumerate()
                .map(|(i, (mode, board, alight, fare))| TransitLeg {
                    mode,
                    route_name: format!("Route {i}"),
                    board_stop: format!("S{i}"),
                    alight_stop: format!("S{}", i + 1),
                    board_time_s: board,
                    alight_time_s: alight,
                    num_stops: 3,
                    fare_cents: fare,
                })
                .collect(),
            transfer_walks_s: Vec::new(),
        }
    }

    #[test]
    fn test_transit_mode_reliability() {
        assert!(TransitMode::Metro.reliability() > TransitMode::Bus.reliability());
    }

    #[test]
    fn test_departure_prediction() {
        let d = Departure {
            route_id: "1".into(),
            route_name: "Line 1".into(),
            mode: TransitMode::Bus,
            stop_id: "s1".into(),
            scheduled_time_s: 1000,
            estimated_delay_s: 120,
            destination: "Central".into(),
        };
        assert_eq!(d.predicted_time_s(), 1120);
    }

    #[test]
    fn test_departure_negative_delay() {
        let d = Departure {
            route_id: "1".into(),
            route_name: "Line 1".into(),
            mode: TransitMode::Bus,
            stop_id: "s1".into(),
            scheduled_time_s: 100,
            estimated_delay_s: -200,
            destination: "Central".into(),
        };
        assert_eq!(d.predicted_time_s(), 0);
    }

    #[test]
    fn test_journey_total_time() {
        let j = make_journey(vec![
            (TransitMode::Bus, 1000, 1600, 250),
            (TransitMode::Metro, 1700, 2200, 350),
        ]);
        // 2200 - 1000 = 1200s ride, 0 walks
        assert_eq!(j.total_time_s(), 1200);
    }

    #[test]
    fn test_journey_fare() {
        let j = make_journey(vec![
            (TransitMode::Bus, 0, 600, 250),
            (TransitMode::Metro, 700, 1200, 350),
        ]);
        assert_eq!(j.total_fare_cents(), 600);
    }

    #[test]
    fn test_journey_transfers() {
        let j = make_journey(vec![
            (TransitMode::Bus, 0, 600, 0),
            (TransitMode::Metro, 700, 1200, 0),
            (TransitMode::Tram, 1300, 1800, 0),
        ]);
        assert_eq!(j.transfers(), 2);
    }

    #[test]
    fn test_nearby_stops() {
        let stops = vec![make_stop("near", 0.001, 0.001), make_stop("far", 1.0, 1.0)];
        let engine = TransitEngine::new(stops, Default::default());
        let nearby = engine.nearby_stops(0.0, 0.0, 500.0);
        assert_eq!(nearby.len(), 1);
        assert_eq!(nearby[0].id, "near");
    }

    #[test]
    fn test_max_transfers_filter() {
        let engine = TransitEngine::new(
            Vec::new(),
            TransitPreferences {
                max_transfers: 1,
                ..Default::default()
            },
        );
        let j = make_journey(vec![
            (TransitMode::Bus, 0, 600, 0),
            (TransitMode::Metro, 700, 1200, 0),
            (TransitMode::Tram, 1300, 1800, 0),
        ]);
        assert!(!engine.is_acceptable(&j)); // 2 transfers > max 1
    }

    #[test]
    fn test_best_journey() {
        let engine = TransitEngine::new(Vec::new(), Default::default());
        let fast = make_journey(vec![(TransitMode::Metro, 0, 600, 300)]);
        let slow = make_journey(vec![(TransitMode::Bus, 0, 1200, 200)]);
        let candidates = vec![slow, fast];
        let best = engine.best_journey(&candidates).unwrap();
        assert!(best.total_time_s() <= 600);
    }

    #[test]
    fn test_wheelchair_filter() {
        let stops = vec![TransitStop {
            id: "a".into(),
            name: "A".into(),
            lat: 0.0,
            lon: 0.0,
            modes: vec![TransitMode::Bus],
            wheelchair_accessible: false,
        }];
        let engine = TransitEngine::new(
            stops,
            TransitPreferences {
                wheelchair: true,
                ..Default::default()
            },
        );
        let nearby = engine.nearby_stops(0.0, 0.0, 1000.0);
        assert!(nearby.is_empty());
    }
}
