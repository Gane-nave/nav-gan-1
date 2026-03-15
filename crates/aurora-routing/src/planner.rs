//! Route planner — generates route plans with alternatives and maneuvers.

use aurora_core::map::RoadSegment;
use aurora_core::route::{
    Corridor, LaneGuidance, Maneuver, ManeuverType, OptimizationObjective, RoutePlan, RouteSegment,
    RouteStatus, Stop, StopType,
};
use aurora_core::types::{EntityId, GeoPosition, TransportMode};
use aurora_map::graph::{haversine_m, RoadGraphIndex};
use chrono::Utc;
use tracing::{debug, info};

use crate::dijkstra::{self, cost, CostFn, ShortestPath};

/// Route planner that generates full route plans with maneuvers.
pub struct RoutePlanner {
    /// Default transport mode.
    default_mode: TransportMode,
    /// Number of alternative routes to generate.
    max_alternatives: usize,
}

impl RoutePlanner {
    pub fn new(default_mode: TransportMode) -> Self {
        Self {
            default_mode,
            max_alternatives: 3,
        }
    }

    /// Set the maximum number of alternatives to generate.
    pub fn with_max_alternatives(mut self, n: usize) -> Self {
        self.max_alternatives = n;
        self
    }

    /// Plan a route from origin to destination.
    pub fn plan_route(
        &self,
        origin: GeoPosition,
        destination: GeoPosition,
        index: &RoadGraphIndex,
        objective: OptimizationObjective,
    ) -> Option<RoutePlan> {
        // Find nearest nodes to origin and destination.
        let (origin_node, _) = index.nearest_node(&origin)?;
        let (dest_node, _) = index.nearest_node(&destination)?;

        let cost_fn = self.cost_fn_for_objective(objective);
        let path = dijkstra::shortest_path(index, origin_node, dest_node, &cost_fn)?;

        let route_segments = self.build_route_segments(index, &path);
        let total_distance: f64 = route_segments.iter().map(|s| s.distance_m).sum();
        let total_duration: f64 = route_segments.iter().map(|s| s.duration_s).sum();

        let plan = RoutePlan {
            id: EntityId::new(),
            user_id: EntityId::new(),
            transport_mode: self.default_mode,
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
                    position: destination,
                    label: Some("Destination".into()),
                    stop_type: StopType::Destination,
                    time_window: None,
                    duration_s: None,
                    precedence: Some(1),
                    arrived: false,
                },
            ],
            segments: route_segments,
            total_distance_m: total_distance,
            total_duration_s: total_duration,
            eta: Utc::now() + chrono::Duration::seconds(total_duration as i64),
            eta_confidence: 0.85,
            alternatives: Vec::new(),
            optimization: objective,
            status: RouteStatus::Planned,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        info!(
            distance_m = total_distance,
            duration_s = total_duration,
            segments = plan.segments.len(),
            "route planned"
        );

        Some(plan)
    }

    /// Generate a corridor envelope around a route.
    pub fn generate_corridor(&self, plan: &RoutePlan, width_m: f64) -> Corridor {
        let center_line: Vec<GeoPosition> = plan
            .segments
            .iter()
            .flat_map(|s| s.geometry.iter().copied())
            .collect();

        Corridor {
            id: EntityId::new(),
            route_id: plan.id,
            width_m,
            center_line,
            segments: plan.segments.iter().map(|s| s.id).collect(),
        }
    }

    /// Check if a position is within a corridor.
    pub fn is_within_corridor(&self, pos: &GeoPosition, corridor: &Corridor) -> bool {
        for point in &corridor.center_line {
            if haversine_m(pos, point) <= corridor.width_m {
                return true;
            }
        }
        false
    }

    /// Detect if the driver has deviated from the route corridor and needs rerouting.
    pub fn check_deviation(&self, current_pos: &GeoPosition, corridor: &Corridor) -> Option<f64> {
        let min_dist = corridor
            .center_line
            .iter()
            .map(|p| haversine_m(current_pos, p))
            .fold(f64::INFINITY, f64::min);

        if min_dist > corridor.width_m {
            Some(min_dist)
        } else {
            None
        }
    }

    // -- internal ---------------------------------------------------------------

    fn cost_fn_for_objective(&self, objective: OptimizationObjective) -> CostFn {
        match objective {
            OptimizationObjective::Fastest => Box::new(cost::by_time),
            OptimizationObjective::Shortest => Box::new(cost::by_distance),
            _ => Box::new(cost::by_time),
        }
    }

    fn build_route_segments(
        &self,
        index: &RoadGraphIndex,
        path: &ShortestPath,
    ) -> Vec<RouteSegment> {
        let mut route_segments = Vec::new();

        for (i, seg_id) in path.segments.iter().enumerate() {
            if let Some(seg) = index.segment(seg_id) {
                let maneuvers = self.generate_maneuvers(index, path, i, seg);
                let duration = seg.travel_time_s.unwrap_or_else(|| {
                    let speed = seg.speed_limit_kmh.unwrap_or(50.0) / 3.6;
                    seg.length_m / speed
                });

                route_segments.push(RouteSegment {
                    id: EntityId::new(),
                    from_stop: EntityId::new(),
                    to_stop: EntityId::new(),
                    geometry: seg.geometry.clone(),
                    distance_m: seg.length_m,
                    duration_s: duration,
                    road_class: Some(format!("{:?}", seg.road_class)),
                    speed_limit_kmh: seg.speed_limit_kmh,
                    maneuvers,
                    risk_score: 0.0,
                    confidence: 0.9,
                });
            }
        }

        route_segments
    }

    fn generate_maneuvers(
        &self,
        index: &RoadGraphIndex,
        path: &ShortestPath,
        seg_index: usize,
        seg: &RoadSegment,
    ) -> Vec<Maneuver> {
        let mut maneuvers = Vec::new();

        // First segment: depart.
        if seg_index == 0 {
            if let Some(pos) = seg.geometry.first() {
                maneuvers.push(Maneuver {
                    position: *pos,
                    maneuver_type: ManeuverType::Depart,
                    instruction: "Start navigation".into(),
                    distance_to_m: 0.0,
                    street_name: None,
                    lane_guidance: None,
                });
            }
        }

        // Last segment: arrive.
        if seg_index == path.segments.len() - 1 {
            if let Some(pos) = seg.geometry.last() {
                maneuvers.push(Maneuver {
                    position: *pos,
                    maneuver_type: ManeuverType::Arrive,
                    instruction: "You have arrived".into(),
                    distance_to_m: seg.length_m,
                    street_name: None,
                    lane_guidance: None,
                });
            }
        }

        // Tunnel entry/exit.
        if seg.tunnel {
            if let Some(pos) = seg.geometry.first() {
                maneuvers.push(Maneuver {
                    position: *pos,
                    maneuver_type: ManeuverType::EnterTunnel,
                    instruction: "Entering tunnel".into(),
                    distance_to_m: 0.0,
                    street_name: None,
                    lane_guidance: None,
                });
            }
        }

        // Turn detection at node between segments.
        if seg_index > 0 {
            let prev_seg_id = path.segments[seg_index - 1];
            if let (Some(prev_seg), Some(pos)) = (index.segment(&prev_seg_id), seg.geometry.first())
            {
                let turn = detect_turn(prev_seg, seg);
                if turn != ManeuverType::Continue {
                    let instruction = format!("{:?}", turn);
                    maneuvers.push(Maneuver {
                        position: *pos,
                        maneuver_type: turn,
                        instruction,
                        distance_to_m: 0.0,
                        street_name: None,
                        lane_guidance: seg.lane_count.map(|n| LaneGuidance {
                            total_lanes: n,
                            recommended_lanes: vec![1],
                            arrows: vec!["↑".into()],
                        }),
                    });
                }
            }
        }

        maneuvers
    }
}

/// Detect the turn type between two consecutive segments based on heading change.
fn detect_turn(from: &RoadSegment, to: &RoadSegment) -> ManeuverType {
    let from_heading = segment_heading(from);
    let to_heading = segment_heading(to);

    if from_heading.is_none() || to_heading.is_none() {
        return ManeuverType::Continue;
    }

    let angle = normalize_angle(to_heading.unwrap() - from_heading.unwrap());

    debug!(from_heading, to_heading, angle, "turn detection");

    match angle {
        a if (-15.0..15.0).contains(&a) => ManeuverType::Continue,
        a if (15.0..45.0).contains(&a) => ManeuverType::TurnSlightRight,
        a if (45.0..135.0).contains(&a) => ManeuverType::TurnRight,
        a if (135.0..=180.0).contains(&a) => ManeuverType::TurnSharpRight,
        a if (-45.0..-15.0).contains(&a) => ManeuverType::TurnSlightLeft,
        a if (-135.0..-45.0).contains(&a) => ManeuverType::TurnLeft,
        a if a <= -135.0 => ManeuverType::TurnSharpLeft,
        _ => ManeuverType::Continue,
    }
}

/// Get the heading of a segment from its geometry.
fn segment_heading(seg: &RoadSegment) -> Option<f64> {
    if seg.geometry.len() < 2 {
        return None;
    }
    let a = &seg.geometry[0];
    let b = seg.geometry.last().unwrap();
    let dlat = b.latitude_deg - a.latitude_deg;
    let dlon = b.longitude_deg - a.longitude_deg;
    Some(dlon.atan2(dlat).to_degrees())
}

/// Normalize an angle to [-180, 180].
fn normalize_angle(angle: f64) -> f64 {
    let mut a = angle % 360.0;
    if a > 180.0 {
        a -= 360.0;
    }
    if a < -180.0 {
        a += 360.0;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::map::*;
    use chrono::Utc;

    fn make_graph_and_index() -> (RoadGraph, RoadGraphIndex) {
        let n1 = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.08,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let n2 = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.085,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let n3 = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.09,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };

        let seg1 = RoadSegment {
            id: EntityId::new(),
            from_node: n1.id,
            to_node: n2.id,
            geometry: vec![n1.position, n2.position],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(50.0),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: false,
            tunnel: false,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: None,
            hazmat_restricted: false,
            length_m: 555.0,
            travel_time_s: Some(40.0),
        };
        let seg2 = RoadSegment {
            id: EntityId::new(),
            from_node: n2.id,
            to_node: n3.id,
            geometry: vec![n2.position, n3.position],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(50.0),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: false,
            tunnel: false,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: None,
            hazmat_restricted: false,
            length_m: 555.0,
            travel_time_s: Some(40.0),
        };

        let graph = RoadGraph {
            id: EntityId::new(),
            region: "test".into(),
            nodes: vec![n1, n2, n3],
            segments: vec![seg1, seg2],
            version: 1,
            updated_at: Utc::now(),
        };
        let index = RoadGraphIndex::from_graph(&graph);
        (graph, index)
    }

    #[test]
    fn plan_route_creates_valid_plan() {
        let (_graph, index) = make_graph_and_index();
        let planner = RoutePlanner::new(TransportMode::PrivateCar);

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

        let plan = planner.plan_route(origin, dest, &index, OptimizationObjective::Fastest);
        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.stops.len(), 2);
        assert!(!plan.segments.is_empty());
        assert!(plan.total_distance_m > 0.0);
        assert!(plan.total_duration_s > 0.0);
    }

    #[test]
    fn corridor_contains_route_points() {
        let (_graph, index) = make_graph_and_index();
        let planner = RoutePlanner::new(TransportMode::PrivateCar);

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

        let plan = planner
            .plan_route(origin, dest, &index, OptimizationObjective::Fastest)
            .unwrap();
        let corridor = planner.generate_corridor(&plan, 50.0);

        // Origin should be within the corridor.
        assert!(planner.is_within_corridor(&origin, &corridor));
    }

    #[test]
    fn deviation_detected_outside_corridor() {
        let (_graph, index) = make_graph_and_index();
        let planner = RoutePlanner::new(TransportMode::PrivateCar);

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

        let plan = planner
            .plan_route(origin, dest, &index, OptimizationObjective::Fastest)
            .unwrap();
        let corridor = planner.generate_corridor(&plan, 50.0);

        let far_pos = GeoPosition {
            latitude_deg: 33.0,
            longitude_deg: 35.0,
            altitude_m: None,
        };
        let deviation = planner.check_deviation(&far_pos, &corridor);
        assert!(deviation.is_some());
    }
}
