//! Dispatch management — coordinates multi-driver operations, batch dispatching,
//! route optimization for task sequences, and real-time re-dispatch.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A dispatch order — a set of tasks assigned to a driver in sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchOrder {
    pub id: EntityId,
    pub driver_id: EntityId,
    pub vehicle_id: Option<EntityId>,
    pub stops: Vec<DispatchStop>,
    pub status: DispatchStatus,
    pub created_at: DateTime<Utc>,
    pub dispatched_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_distance_km: f64,
    pub total_estimated_time_min: f64,
}

/// A stop in a dispatch order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchStop {
    pub task_id: EntityId,
    pub sequence: u32,
    pub location: GeoPosition,
    pub location_name: Option<String>,
    pub estimated_arrival: Option<DateTime<Utc>>,
    pub actual_arrival: Option<DateTime<Utc>>,
    pub estimated_departure: Option<DateTime<Utc>>,
    pub actual_departure: Option<DateTime<Utc>>,
    pub status: StopStatus,
    pub service_time_min: f64,
}

/// Status of a dispatch order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchStatus {
    Draft,
    Dispatched,
    InProgress,
    Completed,
    Cancelled,
    PartiallyCompleted,
}

/// Status of a single stop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopStatus {
    Pending,
    EnRoute,
    Arrived,
    Servicing,
    Completed,
    Skipped,
}

/// Dispatch manager — creates and manages dispatch orders for fleet drivers.
pub struct DispatchManager {
    orders: Vec<DispatchOrder>,
    max_stops_per_order: usize,
}

impl DispatchManager {
    pub fn new() -> Self {
        Self {
            orders: Vec::new(),
            max_stops_per_order: 20,
        }
    }

    /// Create a new dispatch order with an ordered list of stops.
    pub fn create_order(
        &mut self,
        driver_id: EntityId,
        stops: Vec<(EntityId, GeoPosition, f64)>, // (task_id, location, service_time_min)
    ) -> Option<EntityId> {
        if stops.is_empty() || stops.len() > self.max_stops_per_order {
            return None;
        }

        let order_id = EntityId::new();
        let dispatch_stops: Vec<DispatchStop> = stops
            .into_iter()
            .enumerate()
            .map(|(i, (task_id, location, service_time))| DispatchStop {
                task_id,
                sequence: i as u32,
                location,
                location_name: None,
                estimated_arrival: None,
                actual_arrival: None,
                estimated_departure: None,
                actual_departure: None,
                status: StopStatus::Pending,
                service_time_min: service_time,
            })
            .collect();

        let total_distance_km = Self::compute_route_distance(&dispatch_stops);
        let total_time: f64 = dispatch_stops
            .iter()
            .map(|s| s.service_time_min)
            .sum::<f64>()
            + total_distance_km * 1.5; // ~1.5 min/km travel estimate

        let order = DispatchOrder {
            id: order_id,
            driver_id,
            vehicle_id: None,
            stops: dispatch_stops,
            status: DispatchStatus::Draft,
            created_at: Utc::now(),
            dispatched_at: None,
            completed_at: None,
            total_distance_km,
            total_estimated_time_min: total_time,
        };

        debug!(
            order_id = %order_id,
            driver_id = %driver_id,
            stops = order.stops.len(),
            distance_km = total_distance_km,
            "Dispatch order created"
        );
        self.orders.push(order);
        Some(order_id)
    }

    /// Dispatch an order (send to driver).
    pub fn dispatch_order(&mut self, order_id: &EntityId) -> bool {
        let Some(order) = self.orders.iter_mut().find(|o| o.id == *order_id) else {
            return false;
        };
        if order.status != DispatchStatus::Draft {
            return false;
        }
        order.status = DispatchStatus::Dispatched;
        order.dispatched_at = Some(Utc::now());
        debug!(order_id = %order_id, "Dispatch order sent");
        true
    }

    /// Mark a stop as arrived.
    pub fn arrive_at_stop(&mut self, order_id: &EntityId, stop_index: usize) -> bool {
        let Some(order) = self.orders.iter_mut().find(|o| o.id == *order_id) else {
            return false;
        };
        if stop_index >= order.stops.len() {
            return false;
        }
        if order.status == DispatchStatus::Dispatched {
            order.status = DispatchStatus::InProgress;
        }
        order.stops[stop_index].status = StopStatus::Arrived;
        order.stops[stop_index].actual_arrival = Some(Utc::now());
        true
    }

    /// Complete a stop.
    pub fn complete_stop(&mut self, order_id: &EntityId, stop_index: usize) -> bool {
        let Some(order) = self.orders.iter_mut().find(|o| o.id == *order_id) else {
            return false;
        };
        if stop_index >= order.stops.len() {
            return false;
        }
        order.stops[stop_index].status = StopStatus::Completed;
        order.stops[stop_index].actual_departure = Some(Utc::now());

        // Check if all stops are complete.
        let all_done = order
            .stops
            .iter()
            .all(|s| s.status == StopStatus::Completed || s.status == StopStatus::Skipped);
        if all_done {
            order.status = DispatchStatus::Completed;
            order.completed_at = Some(Utc::now());
            debug!(order_id = %order_id, "All stops completed");
        }
        true
    }

    /// Skip a stop.
    pub fn skip_stop(&mut self, order_id: &EntityId, stop_index: usize) -> bool {
        let Some(order) = self.orders.iter_mut().find(|o| o.id == *order_id) else {
            return false;
        };
        if stop_index >= order.stops.len() {
            return false;
        }
        order.stops[stop_index].status = StopStatus::Skipped;

        // Check if remaining stops are done.
        let has_skips = order.stops.iter().any(|s| s.status == StopStatus::Skipped);
        let all_resolved = order
            .stops
            .iter()
            .all(|s| s.status == StopStatus::Completed || s.status == StopStatus::Skipped);
        if all_resolved && has_skips {
            order.status = DispatchStatus::PartiallyCompleted;
            order.completed_at = Some(Utc::now());
        }
        true
    }

    /// Insert a new stop into an existing order at a given position.
    pub fn insert_stop(
        &mut self,
        order_id: &EntityId,
        position: usize,
        task_id: EntityId,
        location: GeoPosition,
        service_time_min: f64,
    ) -> bool {
        let Some(order) = self.orders.iter_mut().find(|o| o.id == *order_id) else {
            return false;
        };
        if order.status == DispatchStatus::Completed || order.status == DispatchStatus::Cancelled {
            return false;
        }
        if order.stops.len() >= self.max_stops_per_order {
            return false;
        }

        let stop = DispatchStop {
            task_id,
            sequence: position as u32,
            location,
            location_name: None,
            estimated_arrival: None,
            actual_arrival: None,
            estimated_departure: None,
            actual_departure: None,
            status: StopStatus::Pending,
            service_time_min,
        };

        let insert_pos = position.min(order.stops.len());
        order.stops.insert(insert_pos, stop);

        // Re-sequence.
        for (i, s) in order.stops.iter_mut().enumerate() {
            s.sequence = i as u32;
        }

        // Recalculate distance.
        order.total_distance_km = Self::compute_route_distance(&order.stops);
        debug!(order_id = %order_id, position, "Stop inserted into dispatch order");
        true
    }

    /// Get an order by ID.
    pub fn order(&self, id: &EntityId) -> Option<&DispatchOrder> {
        self.orders.iter().find(|o| o.id == *id)
    }

    /// Get orders for a driver.
    pub fn orders_for_driver(&self, driver_id: &EntityId) -> Vec<&DispatchOrder> {
        self.orders
            .iter()
            .filter(|o| o.driver_id == *driver_id)
            .collect()
    }

    /// Get active (non-terminal) orders.
    pub fn active_orders(&self) -> Vec<&DispatchOrder> {
        self.orders
            .iter()
            .filter(|o| {
                o.status != DispatchStatus::Completed
                    && o.status != DispatchStatus::Cancelled
                    && o.status != DispatchStatus::PartiallyCompleted
            })
            .collect()
    }

    /// Next pending stop in an order.
    pub fn next_stop(&self, order_id: &EntityId) -> Option<&DispatchStop> {
        self.orders
            .iter()
            .find(|o| o.id == *order_id)
            .and_then(|o| o.stops.iter().find(|s| s.status == StopStatus::Pending))
    }

    /// Completion percentage for an order.
    pub fn completion_pct(&self, order_id: &EntityId) -> Option<f64> {
        self.orders.iter().find(|o| o.id == *order_id).map(|o| {
            if o.stops.is_empty() {
                return 0.0;
            }
            let done = o
                .stops
                .iter()
                .filter(|s| s.status == StopStatus::Completed || s.status == StopStatus::Skipped)
                .count();
            done as f64 / o.stops.len() as f64
        })
    }

    /// Total order count.
    pub fn order_count(&self) -> usize {
        self.orders.len()
    }

    /// Compute total route distance from sequential stops.
    fn compute_route_distance(stops: &[DispatchStop]) -> f64 {
        if stops.len() < 2 {
            return 0.0;
        }
        let mut total = 0.0;
        for pair in stops.windows(2) {
            total += haversine_distance(&pair[0].location, &pair[1].location) / 1000.0;
        }
        total
    }
}

impl Default for DispatchManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let d_lat = (b.latitude_deg - a.latitude_deg).to_radians();
    let d_lon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let h = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(lat: f64, lon: f64) -> GeoPosition {
        GeoPosition {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: None,
        }
    }

    fn make_stops(n: usize) -> Vec<(EntityId, GeoPosition, f64)> {
        (0..n)
            .map(|i| {
                (
                    EntityId::new(),
                    pos(32.0 + i as f64 * 0.01, 34.0 + i as f64 * 0.01),
                    10.0,
                )
            })
            .collect()
    }

    #[test]
    fn create_and_dispatch_order() {
        let mut mgr = DispatchManager::new();
        let driver_id = EntityId::new();
        let order_id = mgr.create_order(driver_id, make_stops(3)).unwrap();

        let order = mgr.order(&order_id).unwrap();
        assert_eq!(order.status, DispatchStatus::Draft);
        assert_eq!(order.stops.len(), 3);

        assert!(mgr.dispatch_order(&order_id));
        assert_eq!(
            mgr.order(&order_id).unwrap().status,
            DispatchStatus::Dispatched
        );
    }

    #[test]
    fn empty_stops_rejected() {
        let mut mgr = DispatchManager::new();
        assert!(mgr.create_order(EntityId::new(), vec![]).is_none());
    }

    #[test]
    fn complete_all_stops() {
        let mut mgr = DispatchManager::new();
        let order_id = mgr.create_order(EntityId::new(), make_stops(2)).unwrap();
        mgr.dispatch_order(&order_id);

        mgr.arrive_at_stop(&order_id, 0);
        mgr.complete_stop(&order_id, 0);
        mgr.arrive_at_stop(&order_id, 1);
        mgr.complete_stop(&order_id, 1);

        assert_eq!(
            mgr.order(&order_id).unwrap().status,
            DispatchStatus::Completed
        );
        assert!(mgr.order(&order_id).unwrap().completed_at.is_some());
    }

    #[test]
    fn skip_stop_partially_completes() {
        let mut mgr = DispatchManager::new();
        let order_id = mgr.create_order(EntityId::new(), make_stops(2)).unwrap();
        mgr.dispatch_order(&order_id);

        mgr.arrive_at_stop(&order_id, 0);
        mgr.complete_stop(&order_id, 0);
        mgr.skip_stop(&order_id, 1);

        assert_eq!(
            mgr.order(&order_id).unwrap().status,
            DispatchStatus::PartiallyCompleted
        );
    }

    #[test]
    fn insert_stop_resequences() {
        let mut mgr = DispatchManager::new();
        let order_id = mgr.create_order(EntityId::new(), make_stops(2)).unwrap();

        let new_task = EntityId::new();
        assert!(mgr.insert_stop(&order_id, 1, new_task, pos(32.05, 34.05), 15.0));

        let order = mgr.order(&order_id).unwrap();
        assert_eq!(order.stops.len(), 3);
        assert_eq!(order.stops[1].task_id, new_task);
        // Sequences should be 0, 1, 2.
        for (i, stop) in order.stops.iter().enumerate() {
            assert_eq!(stop.sequence, i as u32);
        }
    }

    #[test]
    fn completion_percentage() {
        let mut mgr = DispatchManager::new();
        let order_id = mgr.create_order(EntityId::new(), make_stops(4)).unwrap();
        mgr.dispatch_order(&order_id);

        assert_eq!(mgr.completion_pct(&order_id), Some(0.0));

        mgr.arrive_at_stop(&order_id, 0);
        mgr.complete_stop(&order_id, 0);
        assert!((mgr.completion_pct(&order_id).unwrap() - 0.25).abs() < 0.01);

        mgr.arrive_at_stop(&order_id, 1);
        mgr.complete_stop(&order_id, 1);
        assert!((mgr.completion_pct(&order_id).unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn next_stop_returns_first_pending() {
        let mut mgr = DispatchManager::new();
        let order_id = mgr.create_order(EntityId::new(), make_stops(3)).unwrap();
        mgr.dispatch_order(&order_id);

        let next = mgr.next_stop(&order_id).unwrap();
        assert_eq!(next.sequence, 0);

        mgr.arrive_at_stop(&order_id, 0);
        mgr.complete_stop(&order_id, 0);
        let next = mgr.next_stop(&order_id).unwrap();
        assert_eq!(next.sequence, 1);
    }

    #[test]
    fn route_distance_calculation() {
        let mut mgr = DispatchManager::new();
        let stops = vec![
            (EntityId::new(), pos(32.0, 34.0), 10.0),
            (EntityId::new(), pos(32.1, 34.1), 10.0),
        ];
        let order_id = mgr.create_order(EntityId::new(), stops).unwrap();
        let order = mgr.order(&order_id).unwrap();
        assert!(
            order.total_distance_km > 0.0,
            "Route distance should be positive"
        );
        assert!(
            order.total_distance_km < 20.0,
            "~13km expected for 0.1 deg offset"
        );
    }

    #[test]
    fn orders_for_driver() {
        let mut mgr = DispatchManager::new();
        let driver_a = EntityId::new();
        let driver_b = EntityId::new();
        mgr.create_order(driver_a, make_stops(2));
        mgr.create_order(driver_a, make_stops(1));
        mgr.create_order(driver_b, make_stops(3));

        assert_eq!(mgr.orders_for_driver(&driver_a).len(), 2);
        assert_eq!(mgr.orders_for_driver(&driver_b).len(), 1);
    }

    #[test]
    fn active_orders_excludes_terminal() {
        let mut mgr = DispatchManager::new();
        let id1 = mgr.create_order(EntityId::new(), make_stops(1)).unwrap();
        mgr.create_order(EntityId::new(), make_stops(1));

        mgr.dispatch_order(&id1);
        mgr.arrive_at_stop(&id1, 0);
        mgr.complete_stop(&id1, 0);

        // id1 is Completed → excluded from active.
        assert_eq!(mgr.active_orders().len(), 1);
    }

    #[test]
    fn cannot_dispatch_completed_order() {
        let mut mgr = DispatchManager::new();
        let id = mgr.create_order(EntityId::new(), make_stops(1)).unwrap();
        mgr.dispatch_order(&id);
        mgr.arrive_at_stop(&id, 0);
        mgr.complete_stop(&id, 0);

        // Cannot dispatch again.
        assert!(!mgr.dispatch_order(&id));
    }
}
