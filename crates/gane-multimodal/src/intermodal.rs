//! Intermodal journey planner.
//!
//! Combines pedestrian, bicycle, and transit legs into seamless
//! multi-modal journeys with optimised transfer points.

/// Transport mode for a journey leg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Walk,
    Bicycle,
    BikeShare,
    Bus,
    Tram,
    Metro,
    Train,
    Ferry,
    Taxi,
    Drive,
}

impl Mode {
    /// Whether this mode is human-powered.
    pub fn is_active(self) -> bool {
        matches!(self, Self::Walk | Self::Bicycle)
    }

    /// Whether this mode uses public transit.
    pub fn is_transit(self) -> bool {
        matches!(
            self,
            Self::Bus | Self::Tram | Self::Metro | Self::Train | Self::Ferry
        )
    }

    /// Approximate CO2 per passenger-km in grams.
    pub fn co2_per_km(self) -> f64 {
        match self {
            Self::Walk | Self::Bicycle | Self::BikeShare => 0.0,
            Self::Metro => 5.0,
            Self::Tram => 8.0,
            Self::Train => 12.0,
            Self::Bus => 30.0,
            Self::Ferry => 50.0,
            Self::Taxi => 120.0,
            Self::Drive => 150.0,
        }
    }
}

/// A leg of an intermodal journey.
#[derive(Debug, Clone)]
pub struct IntermodalLeg {
    pub mode: Mode,
    pub from_lat: f64,
    pub from_lon: f64,
    pub to_lat: f64,
    pub to_lon: f64,
    pub distance_m: f64,
    pub duration_s: f64,
    pub fare_cents: u32,
    pub instructions: String,
}

/// Complete intermodal journey.
#[derive(Debug, Clone)]
pub struct IntermodalJourney {
    pub legs: Vec<IntermodalLeg>,
}

impl IntermodalJourney {
    /// Total distance in metres.
    pub fn total_distance_m(&self) -> f64 {
        self.legs.iter().map(|l| l.distance_m).sum()
    }

    /// Total duration in seconds.
    pub fn total_duration_s(&self) -> f64 {
        self.legs.iter().map(|l| l.duration_s).sum()
    }

    /// Total fare in cents.
    pub fn total_fare_cents(&self) -> u32 {
        self.legs.iter().map(|l| l.fare_cents).sum()
    }

    /// Total CO2 emissions in grams.
    pub fn total_co2_g(&self) -> f64 {
        self.legs
            .iter()
            .map(|l| l.mode.co2_per_km() * l.distance_m / 1000.0)
            .sum()
    }

    /// Active travel distance (walking + cycling) in metres.
    pub fn active_distance_m(&self) -> f64 {
        self.legs
            .iter()
            .filter(|l| l.mode.is_active())
            .map(|l| l.distance_m)
            .sum()
    }

    /// Number of mode changes.
    pub fn mode_changes(&self) -> usize {
        if self.legs.len() <= 1 {
            return 0;
        }
        self.legs
            .windows(2)
            .filter(|w| w[0].mode != w[1].mode)
            .count()
    }
}

/// Intermodal journey scoring criteria.
#[derive(Debug, Clone)]
pub struct ScoringWeights {
    pub time_weight: f64,
    pub cost_weight: f64,
    pub co2_weight: f64,
    pub transfer_penalty: f64,
    pub active_bonus: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            time_weight: 1.0,
            cost_weight: 0.5,
            co2_weight: 0.3,
            transfer_penalty: 120.0,
            active_bonus: 0.1,
        }
    }
}

/// Intermodal journey planner.
#[derive(Debug)]
pub struct IntermodalPlanner {
    weights: ScoringWeights,
}

impl IntermodalPlanner {
    pub fn new(weights: ScoringWeights) -> Self {
        Self { weights }
    }

    /// Score a journey (lower = better).
    pub fn score(&self, journey: &IntermodalJourney) -> f64 {
        let time = journey.total_duration_s() * self.weights.time_weight;
        let cost = journey.total_fare_cents() as f64 * self.weights.cost_weight;
        let co2 = journey.total_co2_g() * self.weights.co2_weight;
        let transfers = journey.mode_changes() as f64 * self.weights.transfer_penalty;
        let active = journey.active_distance_m() * self.weights.active_bonus;
        time + cost + co2 + transfers - active
    }

    /// Select the best journey from candidates.
    pub fn best<'a>(&self, journeys: &'a [IntermodalJourney]) -> Option<&'a IntermodalJourney> {
        journeys.iter().min_by(|a, b| {
            self.score(a)
                .partial_cmp(&self.score(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Rank journeys by score (best first).
    pub fn rank(&self, journeys: &[IntermodalJourney]) -> Vec<(usize, f64)> {
        let mut scored: Vec<(usize, f64)> = journeys
            .iter()
            .enumerate()
            .map(|(i, j)| (i, self.score(j)))
            .collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leg(mode: Mode, dist: f64, dur: f64, fare: u32) -> IntermodalLeg {
        IntermodalLeg {
            mode,
            from_lat: 0.0,
            from_lon: 0.0,
            to_lat: 0.0,
            to_lon: 0.01,
            distance_m: dist,
            duration_s: dur,
            fare_cents: fare,
            instructions: String::new(),
        }
    }

    #[test]
    fn test_mode_co2() {
        assert!((Mode::Walk.co2_per_km()).abs() < f64::EPSILON);
        assert!(Mode::Drive.co2_per_km() > Mode::Metro.co2_per_km());
    }

    #[test]
    fn test_mode_classification() {
        assert!(Mode::Walk.is_active());
        assert!(Mode::Metro.is_transit());
        assert!(!Mode::Drive.is_transit());
    }

    #[test]
    fn test_journey_totals() {
        let j = IntermodalJourney {
            legs: vec![
                leg(Mode::Walk, 500.0, 360.0, 0),
                leg(Mode::Metro, 5000.0, 600.0, 350),
                leg(Mode::Walk, 300.0, 216.0, 0),
            ],
        };
        assert!((j.total_distance_m() - 5800.0).abs() < f64::EPSILON);
        assert!((j.total_duration_s() - 1176.0).abs() < f64::EPSILON);
        assert_eq!(j.total_fare_cents(), 350);
    }

    #[test]
    fn test_co2_calculation() {
        let j = IntermodalJourney {
            legs: vec![
                leg(Mode::Walk, 1000.0, 720.0, 0),
                leg(Mode::Drive, 10000.0, 600.0, 0),
            ],
        };
        // Walk: 0, Drive: 150 * 10 = 1500g
        assert!((j.total_co2_g() - 1500.0).abs() < 0.01);
    }

    #[test]
    fn test_active_distance() {
        let j = IntermodalJourney {
            legs: vec![
                leg(Mode::Walk, 500.0, 360.0, 0),
                leg(Mode::Metro, 5000.0, 600.0, 350),
                leg(Mode::Bicycle, 2000.0, 400.0, 0),
            ],
        };
        assert!((j.active_distance_m() - 2500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mode_changes() {
        let j = IntermodalJourney {
            legs: vec![
                leg(Mode::Walk, 500.0, 360.0, 0),
                leg(Mode::Metro, 5000.0, 600.0, 350),
                leg(Mode::Metro, 3000.0, 400.0, 0), // same mode
                leg(Mode::Walk, 300.0, 216.0, 0),
            ],
        };
        assert_eq!(j.mode_changes(), 2); // Walk→Metro, Metro→Walk
    }

    #[test]
    fn test_planner_prefers_green() {
        let planner = IntermodalPlanner::new(ScoringWeights {
            co2_weight: 10.0, // Heavily weight CO2
            ..Default::default()
        });
        let green = IntermodalJourney {
            legs: vec![
                leg(Mode::Walk, 500.0, 360.0, 0),
                leg(Mode::Metro, 5000.0, 600.0, 350),
            ],
        };
        let dirty = IntermodalJourney {
            legs: vec![leg(Mode::Drive, 5500.0, 500.0, 0)],
        };
        assert!(planner.score(&green) < planner.score(&dirty));
    }

    #[test]
    fn test_planner_rank() {
        let planner = IntermodalPlanner::new(Default::default());
        let fast = IntermodalJourney {
            legs: vec![leg(Mode::Metro, 5000.0, 300.0, 350)],
        };
        let slow = IntermodalJourney {
            legs: vec![leg(Mode::Bus, 5000.0, 900.0, 200)],
        };
        let ranked = planner.rank(&[slow, fast]);
        assert_eq!(ranked[0].0, 1); // fast is index 1
    }

    #[test]
    fn test_planner_best() {
        let planner = IntermodalPlanner::new(Default::default());
        let a = IntermodalJourney {
            legs: vec![leg(Mode::Metro, 5000.0, 300.0, 350)],
        };
        let b = IntermodalJourney {
            legs: vec![leg(Mode::Bus, 5000.0, 900.0, 200)],
        };
        let candidates = [b, a];
        let best = planner.best(&candidates).unwrap();
        assert!(best.total_duration_s() <= 300.0);
    }
}
