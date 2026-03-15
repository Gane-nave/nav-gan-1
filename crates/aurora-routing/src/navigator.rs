//! Navigator — real-time turn-by-turn navigation state machine.

use aurora_core::route::{Maneuver, ManeuverType, RoutePlan};
use aurora_core::types::GeoPosition;
use aurora_map::graph::haversine_m;
use tracing::{debug, info, warn};

/// Navigation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavState {
    /// Not navigating.
    #[default]
    Idle,
    /// Actively navigating a route.
    Navigating,
    /// Rerouting due to deviation.
    Rerouting,
    /// Arrived at destination.
    Arrived,
}

/// Real-time navigation controller.
pub struct Navigator {
    state: NavState,
    /// Active route plan.
    active_plan: Option<RoutePlan>,
    /// Current segment index.
    current_segment_idx: usize,
    /// Current maneuver index within segment.
    current_maneuver_idx: usize,
    /// Distance to next maneuver (metres).
    distance_to_next_maneuver_m: f64,
    /// Total distance remaining (metres).
    distance_remaining_m: f64,
    /// Total time remaining (seconds).
    time_remaining_s: f64,
    /// Deviation threshold (metres).
    deviation_threshold_m: f64,
    /// Number of reroutes performed.
    reroute_count: u32,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            state: NavState::Idle,
            active_plan: None,
            current_segment_idx: 0,
            current_maneuver_idx: 0,
            distance_to_next_maneuver_m: 0.0,
            distance_remaining_m: 0.0,
            time_remaining_s: 0.0,
            deviation_threshold_m: 50.0,
            reroute_count: 0,
        }
    }

    /// Start navigating a route plan.
    pub fn start(&mut self, plan: RoutePlan) {
        self.distance_remaining_m = plan.total_distance_m;
        self.time_remaining_s = plan.total_duration_s;
        self.active_plan = Some(plan);
        self.current_segment_idx = 0;
        self.current_maneuver_idx = 0;
        self.state = NavState::Navigating;
        info!("navigation started");
    }

    /// Update the navigator with a new position fix.
    pub fn update(&mut self, position: &GeoPosition) -> NavUpdate {
        if self.state != NavState::Navigating {
            return NavUpdate::default();
        }

        if self.active_plan.is_none() {
            return NavUpdate::default();
        }

        // Check arrival at destination.
        let dest_pos = self
            .active_plan
            .as_ref()
            .unwrap()
            .stops
            .last()
            .map(|s| s.position);
        if let Some(dp) = dest_pos {
            let dist_to_dest = haversine_m(position, &dp);
            if dist_to_dest < 30.0 {
                self.state = NavState::Arrived;
                info!("arrived at destination");
                return NavUpdate {
                    state: NavState::Arrived,
                    next_maneuver: None,
                    distance_to_next_maneuver_m: 0.0,
                    distance_remaining_m: 0.0,
                    time_remaining_s: 0.0,
                    should_reroute: false,
                    current_instruction: Some("You have arrived".into()),
                };
            }
        }

        // Find nearest segment and update progress.
        let (nearest_seg_idx, nearest_dist) = {
            let plan = self.active_plan.as_ref().unwrap();
            let cur = self.current_segment_idx;
            let start = if cur > 0 { cur - 1 } else { 0 };
            let mut best_idx = cur;
            let mut best_dist = f64::MAX;
            for (i, seg) in plan.segments.iter().enumerate().skip(start) {
                for point in &seg.geometry {
                    let dist = haversine_m(position, point);
                    if dist < best_dist {
                        best_dist = dist;
                        best_idx = i;
                    }
                }
            }
            (best_idx, best_dist)
        };

        // Check for deviation.
        if nearest_dist > self.deviation_threshold_m {
            warn!(
                deviation_m = nearest_dist,
                threshold_m = self.deviation_threshold_m,
                "route deviation detected"
            );
            self.state = NavState::Rerouting;
            self.reroute_count += 1;
            return NavUpdate {
                state: NavState::Rerouting,
                next_maneuver: None,
                distance_to_next_maneuver_m: 0.0,
                distance_remaining_m: self.distance_remaining_m,
                time_remaining_s: self.time_remaining_s,
                should_reroute: true,
                current_instruction: Some("Rerouting...".into()),
            };
        }

        // Update segment progress.
        self.current_segment_idx = nearest_seg_idx;

        // Calculate remaining distance and time.
        let (remaining, time_remaining) = {
            let plan = self.active_plan.as_ref().unwrap();
            let mut rem = 0.0;
            let mut time = 0.0;
            for seg in plan.segments.iter().skip(nearest_seg_idx) {
                rem += seg.distance_m;
                time += seg.duration_s;
            }
            (rem, time)
        };
        self.distance_remaining_m = remaining;
        self.time_remaining_s = time_remaining;

        // Find next maneuver.
        let next_maneuver = self.find_next_maneuver_owned(position);

        let instruction = next_maneuver.as_ref().map(|m| {
            format!(
                "In {:.0}m, {}",
                self.distance_to_next_maneuver_m, m.instruction
            )
        });

        debug!(
            segment = nearest_seg_idx,
            remaining_m = remaining,
            ?next_maneuver,
            "nav update"
        );

        NavUpdate {
            state: NavState::Navigating,
            next_maneuver,
            distance_to_next_maneuver_m: self.distance_to_next_maneuver_m,
            distance_remaining_m: self.distance_remaining_m,
            time_remaining_s: self.time_remaining_s,
            should_reroute: false,
            current_instruction: instruction,
        }
    }

    /// Apply a new route after rerouting.
    pub fn apply_reroute(&mut self, plan: RoutePlan) {
        info!(reroute_count = self.reroute_count, "reroute applied");
        self.start(plan);
    }

    /// Stop navigation.
    pub fn stop(&mut self) {
        self.state = NavState::Idle;
        self.active_plan = None;
        info!("navigation stopped");
    }

    /// Current navigation state.
    pub fn state(&self) -> NavState {
        self.state
    }

    /// Number of reroutes performed.
    pub fn reroute_count(&self) -> u32 {
        self.reroute_count
    }

    /// Active route plan.
    pub fn active_plan(&self) -> Option<&RoutePlan> {
        self.active_plan.as_ref()
    }

    // -- internal ---------------------------------------------------------------

    /// Find the next maneuver, returning an owned copy to avoid borrow issues.
    fn find_next_maneuver_owned(&mut self, pos: &GeoPosition) -> Option<Maneuver> {
        let plan = self.active_plan.as_ref()?;
        let seg_idx = self.current_segment_idx;
        let mut result: Option<(Maneuver, f64)> = None;

        for seg in plan.segments.iter().skip(seg_idx) {
            for maneuver in &seg.maneuvers {
                let dist = haversine_m(pos, &maneuver.position);
                // Skip depart maneuver if we're past it.
                if maneuver.maneuver_type == ManeuverType::Depart && dist < 20.0 {
                    continue;
                }
                if dist > 10.0 {
                    result = Some((maneuver.clone(), dist));
                    break;
                }
            }
            if result.is_some() {
                break;
            }
        }

        if let Some((maneuver, dist)) = result {
            self.distance_to_next_maneuver_m = dist;
            Some(maneuver)
        } else {
            None
        }
    }
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}

/// Update returned by the navigator on each position fix.
#[derive(Debug, Clone, Default)]
pub struct NavUpdate {
    pub state: NavState,
    pub next_maneuver: Option<Maneuver>,
    pub distance_to_next_maneuver_m: f64,
    pub distance_remaining_m: f64,
    pub time_remaining_s: f64,
    pub should_reroute: bool,
    pub current_instruction: Option<String>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::route::*;
    use aurora_core::types::EntityId;

    fn make_simple_plan() -> RoutePlan {
        let origin = GeoPosition {
            latitude_deg: 32.08,
            longitude_deg: 34.78,
            altitude_m: None,
        };
        let dest = GeoPosition {
            latitude_deg: 32.09,
            longitude_deg: 34.78,
            altitude_m: None,
        };

        RoutePlan {
            id: EntityId::new(),
            user_id: EntityId::new(),
            transport_mode: aurora_core::types::TransportMode::PrivateCar,
            stops: vec![
                Stop {
                    id: EntityId::new(),
                    position: origin,
                    label: Some("Origin".into()),
                    stop_type: StopType::Origin,
                    time_window: None,
                    duration_s: None,
                    precedence: Some(0),
                    arrived: false,
                },
                Stop {
                    id: EntityId::new(),
                    position: dest,
                    label: Some("Destination".into()),
                    stop_type: StopType::Destination,
                    time_window: None,
                    duration_s: None,
                    precedence: Some(1),
                    arrived: false,
                },
            ],
            segments: vec![RouteSegment {
                id: EntityId::new(),
                from_stop: EntityId::new(),
                to_stop: EntityId::new(),
                geometry: vec![origin, dest],
                distance_m: 1110.0,
                duration_s: 80.0,
                road_class: Some("Primary".into()),
                speed_limit_kmh: Some(50.0),
                maneuvers: vec![
                    Maneuver {
                        position: origin,
                        maneuver_type: ManeuverType::Depart,
                        instruction: "Start navigation".into(),
                        distance_to_m: 0.0,
                        street_name: None,
                        lane_guidance: None,
                    },
                    Maneuver {
                        position: dest,
                        maneuver_type: ManeuverType::Arrive,
                        instruction: "You have arrived".into(),
                        distance_to_m: 1110.0,
                        street_name: None,
                        lane_guidance: None,
                    },
                ],
                risk_score: 0.0,
                confidence: 0.9,
            }],
            total_distance_m: 1110.0,
            total_duration_s: 80.0,
            eta: chrono::Utc::now(),
            eta_confidence: 0.85,
            alternatives: Vec::new(),
            optimization: OptimizationObjective::Fastest,
            status: RouteStatus::Planned,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn navigator_starts_idle() {
        let nav = Navigator::new();
        assert_eq!(nav.state(), NavState::Idle);
    }

    #[test]
    fn start_sets_navigating() {
        let mut nav = Navigator::new();
        nav.start(make_simple_plan());
        assert_eq!(nav.state(), NavState::Navigating);
    }

    #[test]
    fn arrival_detected_near_destination() {
        let mut nav = Navigator::new();
        nav.start(make_simple_plan());

        let near_dest = GeoPosition {
            latitude_deg: 32.09,
            longitude_deg: 34.78,
            altitude_m: None,
        };
        let update = nav.update(&near_dest);
        assert_eq!(update.state, NavState::Arrived);
    }

    #[test]
    fn deviation_triggers_reroute() {
        let mut nav = Navigator::new();
        nav.start(make_simple_plan());

        let far_away = GeoPosition {
            latitude_deg: 33.0,
            longitude_deg: 35.0,
            altitude_m: None,
        };
        let update = nav.update(&far_away);
        assert_eq!(update.state, NavState::Rerouting);
        assert!(update.should_reroute);
    }

    #[test]
    fn stop_returns_to_idle() {
        let mut nav = Navigator::new();
        nav.start(make_simple_plan());
        nav.stop();
        assert_eq!(nav.state(), NavState::Idle);
    }
}
