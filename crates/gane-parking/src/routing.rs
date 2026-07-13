//! Parking routing — park-to-destination navigation, parking entry routing,
//! and reservation integration.

use chrono::{DateTime, Utc};
use gane_core::types::{EntityId, GeoPosition};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A parking reservation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingReservation {
    pub id: EntityId,
    pub user_id: EntityId,
    pub zone_id: EntityId,
    pub spot_id: Option<EntityId>,
    pub reserved_from: DateTime<Utc>,
    pub reserved_until: DateTime<Utc>,
    pub status: ReservationStatus,
    pub confirmation_code: Option<String>,
}

/// Reservation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Active,
    Expired,
    Cancelled,
}

/// A parking route plan — from current position to parking, then walking to destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingRoutePlan {
    pub id: EntityId,
    pub parking_zone_id: EntityId,
    pub parking_zone_name: String,
    pub drive_distance_m: f64,
    pub drive_duration_s: f64,
    pub walk_distance_m: f64,
    pub walk_duration_s: f64,
    pub total_duration_s: f64,
    pub estimated_cost: Option<f64>,
    pub parking_position: GeoPosition,
    pub destination_position: GeoPosition,
    pub created_at: DateTime<Utc>,
}

/// Parking router — finds the best combined drive + park + walk route.
pub struct ParkingRouter {
    reservations: Vec<ParkingReservation>,
    route_plans: Vec<ParkingRoutePlan>,
    /// Average walking speed (m/s).
    walk_speed_mps: f64,
    /// Average driving speed to parking (m/s).
    drive_speed_mps: f64,
}

impl ParkingRouter {
    pub fn new() -> Self {
        Self {
            reservations: Vec::new(),
            route_plans: Vec::new(),
            walk_speed_mps: 1.4,   // ~5 km/h walking
            drive_speed_mps: 8.33, // ~30 km/h urban driving
        }
    }

    /// Create a parking route plan considering drive + park + walk.
    pub fn plan_route(
        &mut self,
        current_pos: &GeoPosition,
        destination: &GeoPosition,
        parking_pos: &GeoPosition,
        zone_id: EntityId,
        zone_name: &str,
        hourly_rate: Option<f64>,
    ) -> ParkingRoutePlan {
        let drive_dist = haversine_distance(current_pos, parking_pos);
        let walk_dist = haversine_distance(parking_pos, destination);
        let drive_dur = drive_dist / self.drive_speed_mps;
        let walk_dur = walk_dist / self.walk_speed_mps;

        let plan = ParkingRoutePlan {
            id: EntityId::new(),
            parking_zone_id: zone_id,
            parking_zone_name: zone_name.into(),
            drive_distance_m: drive_dist,
            drive_duration_s: drive_dur,
            walk_distance_m: walk_dist,
            walk_duration_s: walk_dur,
            total_duration_s: drive_dur + walk_dur,
            estimated_cost: hourly_rate,
            parking_position: *parking_pos,
            destination_position: *destination,
            created_at: Utc::now(),
        };

        debug!(
            zone = zone_name,
            drive_m = drive_dist,
            walk_m = walk_dist,
            total_s = plan.total_duration_s,
            "parking route planned"
        );

        self.route_plans.push(plan.clone());
        plan
    }

    /// Compare multiple parking options and rank them by total time.
    pub fn rank_by_total_time(&self) -> Vec<&ParkingRoutePlan> {
        let mut plans: Vec<&ParkingRoutePlan> = self.route_plans.iter().collect();
        plans.sort_by(|a, b| {
            a.total_duration_s
                .partial_cmp(&b.total_duration_s)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        plans
    }

    /// Compare multiple parking options and rank them by walk distance.
    pub fn rank_by_walk_distance(&self) -> Vec<&ParkingRoutePlan> {
        let mut plans: Vec<&ParkingRoutePlan> = self.route_plans.iter().collect();
        plans.sort_by(|a, b| {
            a.walk_distance_m
                .partial_cmp(&b.walk_distance_m)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        plans
    }

    /// Add a reservation.
    pub fn add_reservation(&mut self, reservation: ParkingReservation) {
        debug!(
            zone = %reservation.zone_id,
            status = ?reservation.status,
            "parking reservation added"
        );
        self.reservations.push(reservation);
    }

    /// Get active reservations for a user.
    pub fn active_reservations(&self, user_id: &EntityId) -> Vec<&ParkingReservation> {
        self.reservations
            .iter()
            .filter(|r| {
                r.user_id == *user_id
                    && matches!(
                        r.status,
                        ReservationStatus::Confirmed | ReservationStatus::Active
                    )
            })
            .collect()
    }

    /// Cancel a reservation.
    pub fn cancel_reservation(&mut self, reservation_id: &EntityId) -> bool {
        if let Some(res) = self
            .reservations
            .iter_mut()
            .find(|r| r.id == *reservation_id)
        {
            res.status = ReservationStatus::Cancelled;
            true
        } else {
            false
        }
    }

    /// Expire old reservations.
    pub fn expire_old_reservations(&mut self) {
        let now = Utc::now();
        for res in &mut self.reservations {
            if res.reserved_until < now
                && matches!(
                    res.status,
                    ReservationStatus::Confirmed | ReservationStatus::Pending
                )
            {
                res.status = ReservationStatus::Expired;
            }
        }
    }

    /// Get all route plans.
    pub fn route_plans(&self) -> &[ParkingRoutePlan] {
        &self.route_plans
    }

    /// Clear route plans.
    pub fn clear_plans(&mut self) {
        self.route_plans.clear();
    }
}

impl Default for ParkingRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let dlat = (b.latitude_deg - a.latitude_deg).to_radians();
    let dlon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let a_val = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a_val.sqrt().atan2((1.0 - a_val).sqrt());
    r * c
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

    #[test]
    fn plan_route_calculates_durations() {
        let mut router = ParkingRouter::new();
        let plan = router.plan_route(
            &pos(32.0, 34.0),
            &pos(32.005, 34.005),
            &pos(32.002, 34.002),
            EntityId::new(),
            "Lot A",
            Some(10.0),
        );

        assert!(plan.drive_distance_m > 0.0);
        assert!(plan.walk_distance_m > 0.0);
        assert!(plan.drive_duration_s > 0.0);
        assert!(plan.walk_duration_s > 0.0);
        assert!(
            (plan.total_duration_s - (plan.drive_duration_s + plan.walk_duration_s)).abs() < 0.01
        );
    }

    #[test]
    fn rank_by_total_time() {
        let mut router = ParkingRouter::new();

        // Far parking but close to destination.
        router.plan_route(
            &pos(32.0, 34.0),
            &pos(32.005, 34.005),
            &pos(32.004, 34.004),
            EntityId::new(),
            "Close Walk",
            None,
        );

        // Close parking but far walk.
        router.plan_route(
            &pos(32.0, 34.0),
            &pos(32.005, 34.005),
            &pos(32.001, 34.001),
            EntityId::new(),
            "Far Walk",
            None,
        );

        let ranked = router.rank_by_total_time();
        assert_eq!(ranked.len(), 2);
        assert!(ranked[0].total_duration_s <= ranked[1].total_duration_s);
    }

    #[test]
    fn rank_by_walk_distance() {
        let mut router = ParkingRouter::new();

        router.plan_route(
            &pos(32.0, 34.0),
            &pos(32.005, 34.005),
            &pos(32.004, 34.004),
            EntityId::new(),
            "Short Walk",
            None,
        );

        router.plan_route(
            &pos(32.0, 34.0),
            &pos(32.005, 34.005),
            &pos(32.001, 34.001),
            EntityId::new(),
            "Long Walk",
            None,
        );

        let ranked = router.rank_by_walk_distance();
        assert!(ranked[0].walk_distance_m <= ranked[1].walk_distance_m);
    }

    #[test]
    fn reservation_lifecycle() {
        let mut router = ParkingRouter::new();
        let user = EntityId::new();
        let res = ParkingReservation {
            id: EntityId::new(),
            user_id: user,
            zone_id: EntityId::new(),
            spot_id: None,
            reserved_from: Utc::now(),
            reserved_until: Utc::now() + chrono::Duration::hours(2),
            status: ReservationStatus::Confirmed,
            confirmation_code: Some("ABC123".into()),
        };
        let rid = res.id;
        router.add_reservation(res);

        assert_eq!(router.active_reservations(&user).len(), 1);
        assert!(router.cancel_reservation(&rid));
        assert_eq!(router.active_reservations(&user).len(), 0);
    }

    #[test]
    fn expire_old_reservations() {
        let mut router = ParkingRouter::new();
        let user = EntityId::new();
        let res = ParkingReservation {
            id: EntityId::new(),
            user_id: user,
            zone_id: EntityId::new(),
            spot_id: None,
            reserved_from: Utc::now() - chrono::Duration::hours(3),
            reserved_until: Utc::now() - chrono::Duration::hours(1),
            status: ReservationStatus::Confirmed,
            confirmation_code: None,
        };
        router.add_reservation(res);

        router.expire_old_reservations();
        assert_eq!(router.active_reservations(&user).len(), 0);
    }

    #[test]
    fn clear_plans() {
        let mut router = ParkingRouter::new();
        router.plan_route(
            &pos(32.0, 34.0),
            &pos(32.005, 34.005),
            &pos(32.002, 34.002),
            EntityId::new(),
            "Lot",
            None,
        );
        assert_eq!(router.route_plans().len(), 1);
        router.clear_plans();
        assert!(router.route_plans().is_empty());
    }
}
